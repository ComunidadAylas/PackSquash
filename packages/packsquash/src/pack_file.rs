//! Container and high-level module for structs that optimize some pack file according to its data
//! format.

use std::{
	borrow::Cow,
	fmt::{Debug, Display},
	io
};

use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_stream::Stream;

pub use util::strip_utf8_bom;

use crate::pack_file::asset_type::PackFileAssetType;

pub mod asset_type;

mod util;

mod audio_file;
mod command_function_file;
mod json_file;
mod legacy_lang_file;
mod passthrough_file;
mod png_file;
mod shader_file;

#[cfg(feature = "optifine-support")]
#[doc(cfg(feature = "optifine-support"))]
mod properties_file;

/// Represents an error that may occur while optimizing a pack file.
#[derive(Error, Debug)]
#[error("{0}")]
pub enum OptimizationError {
	AudioFile(#[from] audio_file::OptimizationError),
	JsonFile(#[from] json_file::OptimizationError),
	PngFile(#[from] png_file::OptimizationError),
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	PropertiesFile(#[from] properties_file::OptimizationError),
	ShaderFile(#[from] shader_file::OptimizationError),
	LegacyLanguageFile(#[from] legacy_lang_file::OptimizationError),
	CommandFunctionFile(#[from] command_function_file::OptimizationError),
	IoError(#[from] io::Error)
}

/// The result of processing a chunk of pack file bytes to an optimized representation, boxed to
/// use dynamic dispatch.
pub type OptimizedBoxedBytesChunk =
	Result<(Cow<'static, str>, Box<dyn AsRef<[u8]> + Send>), OptimizationError>;

/// The result of processing a chunk of pack file bytes to an optimized representation.
type OptimizedBytesChunk<T, E> = Result<(Cow<'static, str>, T), E>;

/// A tuple that contains an [`AsyncRead`] for a pack file and its estimated size.
type AsyncReadAndSizeHint<T> = (T, u64);

/// A Minecraft pack file in some format, that may be processed in order to improve its internal
/// coding efficiency and/or its compressibility by a lossless data compression algorithm. This
/// processing can be lossy, although it is always possible to make it lossless; see the concrete
/// implementation for details.
///
/// A pack file is aware of its general format, but some details about how it is optimized may
/// depend on the concrete [`PackFileAssetType`] that it represents. A pack file may represent one
/// or several asset types, depending on how Minecraft parses it.
trait PackFile {
	/// The type of owned byte chunks that this pack file produces when being processed.
	type ByteChunkType: AsRef<[u8]> + Send + 'static;

	/// An error that may occur while computing the optimized representation of the pack file.
	type OptimizationError: Into<OptimizationError> + Debug + Display;

	/// The type of the stream that will yield either optimized byte chunks or processing errors for
	/// this pack file.
	type OptimizedByteChunksStream: Stream<Item = OptimizedBytesChunk<Self::ByteChunkType, Self::OptimizationError>>
		+ Send
		+ Unpin
		+ 'static;

	/// Processes this pack file, returning a stream of chunks of optimized bytes. Each chunk has a string
	/// that briefly describes the optimization strategy that was applied to it. If the stream returns no
	/// items, the file should be treated as if its optimized representation contains no bytes.
	///
	/// The returned stream is not necessarily fused (i.e. _undefined behavior_ may happen if the caller
	/// insists on consuming more items after it has returned that no more items will be yielded).
	///
	/// # Errors
	/// Errors may be yielded by the stream returned by this method if some I/O operation goes wrong, the
	/// pack file is malformed, or any other unrecoverable condition is encountered. In this case, users of
	/// this method _must_ stop iterating over the result stream immediately.
	fn process(self) -> Self::OptimizedByteChunksStream;

	/// Returns whether the contents of this pack file are already internally compressed, and as such any
	/// attempt to further compress them will likely result in lower than usual space savings.
	fn is_compressed(&self) -> bool;
}

/// Factory trait for a [`PackFile`] that allows it to be instantiated in an standard way. It is separated
/// from that trait because it can only be implemented in a sized type, and that constraint would make the
/// [`PackFile`] trait not object-safe, which may limit the usage of the pack file by client code.
///
/// Every [`PackFile`] must implement this trait in order to be used easily with the already existing
/// high-level library code.
trait PackFileConstructor<R: AsyncRead + Unpin + 'static>: PackFile + Sized {
	/// The type of optimization settings that have to be used to instantiate the pack file.
	type OptimizationSettings;

	/// Instantiates this pack file with the provided optimization settings, specific to its asset
	/// type, lazily associating it with the read struct that `file_read_producer` returns.
	///
	/// This operation will not yield a pack file instance if the pack file should be skipped, or if the
	/// read struct producer function returns `None`. It is the responsibility of the caller to deal with
	/// any I/O error that may happen during the execution of this producer function. If these conditions
	/// do not apply, this method is guaranteed to suceed and return `Some`.
	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<R>>,
		asset_type: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self>;
}

/// Contains the different pieces of data obtained by processing some pack file.
pub struct PackFileProcessData {
	/// A stream that contains the byte chunks of the processed pack file data.
	pub optimized_byte_chunks_stream: Box<dyn Stream<Item = OptimizedBoxedBytesChunk> + Send + Unpin>,
	/// Represents whether the contents of this pack file are already internally compressed,
	/// and as such any attempt to further compress them will likely result in lower than
	/// usual space savings.
	pub is_compressed: bool,
	/// The canonical extension for the pack file, which Minecraft expects. It might be `None`
	/// if the pack file is already known to have a canonical extension.
	pub canonical_extension: Option<&'static str>
}
