use bytes::Bytes;
use flate2::bufread::DeflateDecoder;
use flate2::read::MultiGzDecoder;
use futures::TryStreamExt;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read, Seek, Write};
use std::num::NonZeroU64;
use std::thread;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_stream::Stream;
use tokio_util::either::Either;
use tokio_util::io::ReaderStream;

#[cfg(test)]
mod tests;

use super::{AsyncReadAndSizeHint, OptimizedBytesChunk, PackFile, PackFileConstructor};
use crate::buffered_async_spooled_temp_file::BufferedAsyncSpooledTempFile;
use crate::config::CompressedCompoundNbtTagFileOptions;
use crate::pack_file::asset_type::PackFileAssetType;
use crate::pack_file::util::AccountingRead;
use crate::zopfli_iterations_time_model::ZopfliIterationsTimeModel;

/// Represents a gzip-compressed file containing a serialized NBT compound tag.
///
/// These files are used by vanilla Minecraft data packs for storing structure templates data.
/// The vanilla game decompressed them using the `java.util.zip.GZIPInputStream` class.
///
/// References:
/// - <https://minecraft.wiki/w/NBT_format>
/// - <https://minecraft.wiki/w/Structure_file>
/// - Minecraft's 24w04a `net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager` class
pub struct CompressedCompoundNbtTagFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	optimization_settings: CompressedCompoundNbtTagFileOptions
}

/// Represents an error that may happen while optimizing compressed compound NBT tag files.
#[derive(Error, Debug)]
pub enum OptimizationError {
	#[error("NBT serialization error: {0}")]
	Nbt(#[from] fastnbt::error::Error),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for CompressedCompoundNbtTagFile<T> {
	type ByteChunkType = Bytes;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream =
		impl Stream<Item = OptimizedBytesChunk<Self::ByteChunkType, Self::OptimizationError>>;

	fn process(mut self) -> Self::OptimizedByteChunksStream {
		const NBT_FILE_BUFFER_SIZE: usize = 2 * 1024 * 1024; // 2 MiB

		macro_rules! bail {
			($op:expr) => {
				match $op {
					Ok(val) => val,
					Err(err) => return Either::Left(tokio_stream::once(Err(err.into())))
				}
			};
		}

		// The scope below is ugly, but AsyncIoBridge or blocking the current thread in place do
		// not work well. Looking back, we probably overengineered our async usage quite a lot...
		let mut input_nbt_gzip =
			BufferedAsyncSpooledTempFile::with_capacity(self.file_length_hint, NBT_FILE_BUFFER_SIZE);
		bail!(thread::scope(|scope| {
			scope
				.spawn(|| {
					tokio::runtime::Builder::new_current_thread()
						.thread_name("nbt-copy")
						.build()
						.map_err(io::Error::other)
						.and_then(|runtime| {
							runtime.block_on(tokio::io::copy(&mut self.read, &mut input_nbt_gzip))
						})
				})
				.join()
				.unwrap()
		}));
		let input_nbt_gzip_size = bail!(input_nbt_gzip.stream_position());

		// Read the input gzipped NBT data, ensuring it parses as a valid NBT compound tag.
		// MultiGzEncoder is used to handle gzip streams with several members (i.e., files) by
		// concatenating them into a single stream of bytes, like Java's `GZIPInputStream` does
		let mut decompressed_nbt_size = 0;
		let nbt_compound_tag: HashMap<String, fastnbt::Value, ahash::RandomState> = {
			bail!(input_nbt_gzip.rewind());
			bail!(fastnbt::from_reader(AccountingRead::new(
				MultiGzDecoder::new(&mut input_nbt_gzip),
				&mut decompressed_nbt_size
			)))
		};

		let zopfli_iteration_count = ZopfliIterationsTimeModel::new(
			self.optimization_settings.nbt_compression_iterations,
			2.0
		)
		.iterations_for_data_size(decompressed_nbt_size.try_into().unwrap_or(u32::MAX), 0, 20);

		// Now serialize the root compound tag again, setting its name to the empty string, and
		// compress it using Zopfli or, if the Zopfli iteration count falls to zero, the best flate2
		// compression
		let mut recompressed_nbt_gzip =
			BufferedAsyncSpooledTempFile::with_capacity(self.file_length_hint, NBT_FILE_BUFFER_SIZE);
		match NonZeroU64::new(zopfli_iteration_count as u64) {
			Some(zopfli_iteration_count) => {
				bail!(fastnbt::to_writer(
					bail!(zopfli::GzipEncoder::new_buffered(
						zopfli::Options {
							iteration_count: zopfli_iteration_count,
							..zopfli::Options::default()
						},
						zopfli::BlockType::Dynamic,
						&mut recompressed_nbt_gzip
					)),
					&nbt_compound_tag
				))
			}
			None => {
				bail!(fastnbt::to_writer(
					flate2::write::GzEncoder::new(
						&mut recompressed_nbt_gzip,
						flate2::Compression::best()
					),
					&nbt_compound_tag
				))
			}
		};
		let recompressed_nbt_gzip_size = bail!(recompressed_nbt_gzip.stream_position());

		Either::Right(
			if input_nbt_gzip_size >= recompressed_nbt_gzip_size {
				// The most likely case: recompressing the NBT data serialization yielded a smaller
				// file, so rewind it for consumption by the caller

				bail!(recompressed_nbt_gzip.rewind());

				Either::Right(
					ReaderStream::new(recompressed_nbt_gzip).map_ok(|processed_bytes| {
						(Cow::Borrowed("Validated and optimized"), processed_bytes)
					})
				)
			} else {
				// Recompressing the NBT data serialization yielded a file that's larger than the
				// input, which may happen if the input was already compressed better than we
				// could, so fall back to a minimal, anonymizing gzip reencapsulation optimization
				// that never increases the file size. Eagerly drop the recompressed NBT data to save
				// on memory, as we won't use it anymore

				drop(recompressed_nbt_gzip);

				let mut reencapsulated_file = BufferedAsyncSpooledTempFile::with_capacity(
					self.file_length_hint,
					NBT_FILE_BUFFER_SIZE
				);
				bail!(reencapsulate_gzip_members(
					{
						bail!(input_nbt_gzip.rewind());
						BufReader::new(input_nbt_gzip)
					},
					&mut reencapsulated_file
				));
				bail!(reencapsulated_file.rewind());

				Either::Left(
					ReaderStream::new(reencapsulated_file).map_ok(|processed_bytes| {
						(
							Cow::Borrowed(
								"Validated, but barely optimized. \
								If not optimized externally, try tweaking options for extra savings"
							),
							processed_bytes
						)
					})
				)
			}
			.map_err(OptimizationError::from)
		)
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T>
	for CompressedCompoundNbtTagFile<T>
{
	type OptimizationSettings = CompressedCompoundNbtTagFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		_: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, size_hint)| Self {
			read,
			file_length_hint: size_hint.try_into().unwrap_or(usize::MAX),
			optimization_settings
		})
	}
}

/// Goes through the members of an assumed-valid [gzip] source and writes them to an output gzip
/// sink, keeping the compressed data as-is but replacing gzip member headers with the smallest
/// legal gzip header, dropping metadata fields.
///
/// By construction, this function guarantees that the output gzip stream will never be larger
/// than the input gzip stream, as the compressed data blocks are kept the same while the gzip
/// encapsulation overhead gets reduced or stays constant.
///
/// [gzip]: https://datatracker.ietf.org/doc/html/rfc1952
fn reencapsulate_gzip_members(
	mut input_gzip_source: impl BufRead + Seek,
	mut output_gzip_sink: impl Write
) -> Result<(), io::Error> {
	const CRC16_LEN: i64 = 2;
	const CRC32_LEN: u64 = 4;
	const ISIZE_LEN: u64 = 4;
	const GZIP_MEMBER_TRAILER_LEN: u64 = CRC32_LEN + ISIZE_LEN;

	loop {
		let mut gzip_member_header = [0; 10];
		match input_gzip_source.read_exact(&mut gzip_member_header) {
			Ok(()) => (),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
				// The input is assumed to be a valid gzip stream, so an EOF here is a normal
				// condition that signals no more members to process
				break;
			}
			err @ Err(_) => return err
		};

		// The mandatory header fields may be followed by optional fields, whose
		// presence is indicated by the header flags
		let header_flags = gzip_member_header[3];
		let have_header_crc = header_flags & (1 << 1) != 0;
		let have_extra_fields = header_flags & (1 << 2) != 0;
		let have_file_name = header_flags & (1 << 3) != 0;
		let have_file_comment = header_flags & (1 << 4) != 0;

		// Skip extra fields
		if have_extra_fields {
			let mut extra_field_length_bytes = [0; 2];
			input_gzip_source.read_exact(&mut extra_field_length_bytes)?;
			input_gzip_source.seek_relative(u16::from_le_bytes(extra_field_length_bytes) as i64)?;
		}

		// Skip file name and file comment
		for have_nul_terminated_string in [have_file_name, have_file_comment] {
			if !have_nul_terminated_string {
				continue;
			}

			for byte_skip_result in
				Read::by_ref(&mut input_gzip_source)
					.bytes()
					.take_while(|byte_result| {
						let byte_result = byte_result.as_ref();
						byte_result.is_err() || byte_result.is_ok_and(|byte| *byte != 0)
					}) {
				byte_skip_result?;
			}
		}

		// Skip header CRC16
		if have_header_crc {
			input_gzip_source.seek_relative(CRC16_LEN)?;
		}

		// What follows in the input gzip stream is the compressed data for this member,
		// so it's a good time to write a size-optimal header to the output gzip stream
		output_gzip_sink.write_all(&[
			0x1F, /* ID1 */
			0x8B, /* ID2 */
			8,    /* CM - DEFLATE */
			0,    /* FLG - binary, no extra fields, no file name, no file comment */
			0, 0, 0, 0, /* MTIME - no mtime */
			0, /* XFL */
			0  /* OS - FAT (chosen for better compressibility, as it's a longer run of zeros) */
		])?;

		// Go through the compressed data to get its length. We can't optimize this that much
		// because the gzip headers lack a compressed data length field, and DEFLATE blocks need
		// not be byte-aligned, which means that we need to parse them anyway to get where the
		// last compressed data block ends
		let compressed_data_length = {
			let mut deflate_decoder = DeflateDecoder::new(&mut input_gzip_source);
			io::copy(&mut deflate_decoder, &mut io::sink())?;

			i64::try_from(deflate_decoder.total_in()).map_err(io::Error::other)?
		};

		// Seek back to the start of the compressed data and copy it to the output gzip stream,
		// including the gzip member trailer
		input_gzip_source.seek_relative(-compressed_data_length)?;
		io::copy(
			&mut input_gzip_source
				.by_ref()
				.take(compressed_data_length as u64 + GZIP_MEMBER_TRAILER_LEN),
			&mut output_gzip_sink
		)?;
	}

	Ok(())
}
