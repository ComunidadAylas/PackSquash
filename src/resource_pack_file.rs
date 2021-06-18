use std::{borrow::Cow, error::Error, ops::Deref};

use tokio_stream::Stream;

/// Wraps data that can be referenced as a byte slice to allow dereferecing
/// it back to a slice, improving ergonomics.
#[derive(Debug)]
struct OptimizedBytes<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> Deref for OptimizedBytes<T> {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}

/// The result of processing a chunk of resource pack file bytes to an optimized
/// representation.
type OptimizedBytesChunk<T, E> = Result<(Cow<'static, str>, OptimizedBytes<T>), E>;

/// A candidate for a Minecraft resource pack file, that may be processed in order to improve
/// its internal coding efficiency and/or its compressibility by a lossless data compression
/// algorithm. This processing can be lossy, although it is always possible to make it lossless;
/// see the concrete implementation for details.
trait ResourcePackFile<T: AsRef<[u8]>, E: Error, S: Stream<Item = OptimizedBytesChunk<T, E>> + Unpin>
{
	/// Processes this resource pack file, returning a stream of chunks of optimized bytes. Each chunk
	/// has a string that briefly describes the optimization strategy that was applied to it. If the
	/// stream returns no items, the file should be treated as if its optimized representation contains
	/// no bytes.
	///
	/// The returned stream is not necessarily fused (i.e. _undefined behavior_ may happen if the caller
	/// insists on consuming more items after it has returned that no more items will be yielded).
	///
	/// # Errors
	/// Errors may be yielded by the stream returned by this method if some I/O operation goes wrong, the
	/// resource pack file is malformed, or any other unrecoverable condition is encountered. In this case,
	/// users of this method _must_ stop iterating over the result stream inmediately.
	fn process(self) -> S;

	/// Returns the canonical extension for this resource pack file.
	///
	/// The canonical extension of a resource pack file is the extension that Minecraft expects when
	/// dealing with this resource pack file.
	fn canonical_extension(&self) -> &str;

	/// Returns whether the contents of this resource pack file are already internally compressed,
	/// and as such any attempt to further compress them will likely result in lower than normal
	/// space savings.
	fn is_compressed(&self) -> bool;
}

mod util;

mod audio_file;
mod json_file;
mod passthrough_file;
mod png_file;
#[cfg(any(feature = "optifine-support", doc))]
#[doc(cfg(feature = "optifine-support"))]
mod properties_file;
mod shader_file;
