use std::{
	borrow::Cow,
	fmt::{Debug, Display},
	io,
	ops::Deref
};

use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_stream::Stream;

use crate::RelativePath;

mod util;

pub mod audio_file;
pub mod json_file;
pub mod passthrough_file;
pub mod png_file;
pub mod shader_file;

#[cfg(any(feature = "optifine-support", doc))]
#[doc(cfg(feature = "optifine-support"))]
pub mod properties_file;

/// Wraps data that can be referenced as a byte slice to allow dereferecing
/// it back to a slice, improving ergonomics.
#[derive(Debug)]
pub struct OptimizedBytes<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> Deref for OptimizedBytes<T> {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}

/// TODO
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
	IoError(#[from] io::Error)
}

/// The result of processing a chunk of pack file bytes to an optimized representation.
#[allow(dead_code)] // Actually used in the PackFile trait definition for nicer syntax
type OptimizedBytesChunk<T, E> = Result<(Cow<'static, str>, OptimizedBytes<T>), E>;

/// A candidate for a Minecraft pack file, that may be processed in order to improve its internal
/// coding efficiency and/or its compressibility by a lossless data compression algorithm. This
/// processing can be lossy, although it is always possible to make it lossless; see the concrete
/// implementation for details.
pub trait PackFile {
	/// The type of byte chunks that this pack file produces when being processed.
	type ByteChunkType: AsRef<[u8]>;

	/// An error that may occur while computing the optimized representation of the pack file.
	type OptimizationError: Into<OptimizationError> + Debug + Display;

	/// The type of the stream that will yield either optimized byte chunks or processing errors for
	/// this pack file.
	type OptimizedBytesChunksStream: Stream<Item = OptimizedBytesChunk<Self::ByteChunkType, Self::OptimizationError>>
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
	/// this method _must_ stop iterating over the result stream inmediately.
	fn process(self) -> Self::OptimizedBytesChunksStream;

	/// Returns the canonical extension for this pack file.
	///
	/// The canonical extension of a pack file is the extension that Minecraft expects when dealing with
	/// this pack file.
	fn canonical_extension(&self) -> &str;

	/// Returns whether the contents of this pack file are already internally compressed, and as such any
	/// attempt to further compress them will likely result in lower than normal space savings.
	fn is_compressed(&self) -> bool;
}

/// TODO
pub trait PackFileConstructor<R: AsyncRead + Unpin + 'static>: PackFile + Sized {
	/// The type of optimization settings that have to be used to instantiate the pack file.
	type OptimizationSettings;

	/// TODO
	fn new<F: FnMut() -> Option<(R, u64)>>(
		file_read_producer: F,
		args: PackFileConstructorArgs<'_, Self::OptimizationSettings>
	) -> Option<Self>;
}

/// TODO
pub struct PackFileConstructorArgs<'a, S> {
	pub path: &'a RelativePath<'a>,
	pub optimization_settings: S
}
