use crate::relative_path::RelativePath;
use crate::scratch_file::{ScratchFile, ScratchFilesBudget};
use crate::squashed_pack_state::SquashedPackState;
use crate::util::strip_utf8_bom::{StripUtf8Bom, StripUtf8BomExt};
use crate::util::zero_copy_deserialize_traits::{
	AlwaysZeroCopyDeserializable, ZeroCopyDeserializable
};
use crate::vfs::{VfsFile, VfsMmap, VirtualFileSystem};
use json_comments::StripComments;
use serde::de::Visitor;
use serde::{Deserializer, Serialize};
use serde_json::ser::{CharEscape, CompactFormatter, Formatter, PrettyFormatter};
use std::io::{self, BufRead, Read, Seek, Write};
use std::time::SystemTime;
use thiserror::Error;

pub enum JsonAssetDeserializeOutcome<'path, T> {
	Value {
		value: T,
		canonical_path: RelativePath<'path>,
		size_hint: usize,
		fresh_in_previous_zip: bool
	},
	NotFound,
	FreshInPreviousZip {
		canonical_path: RelativePath<'path>
	},
	CanonicalPathAlreadyProcessed
}

#[derive(Error, Debug)]
pub enum JsonDeserializationError {
	#[error("JSON error: {0}")]
	Json(#[from] packsquash_util::PrettySerdePathErrorWrapper<serde_json::Error>),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

pub fn deserialize<
	'path,
	T: AlwaysZeroCopyDeserializable,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send,
	R
>(
	path: &'path RelativePath,
	vfs: &V,
	squashed_pack_state: &SquashedPackState<'_, '_, F>,
	deserialize_if_fresh_in_previous_zip: bool,
	always_allow_json_comments: bool,
	canonical_extension: &str,
	outcome_callback: impl OutcomeCallback<'path, T, R>
) -> Result<R, JsonDeserializationError> {
	macro_rules! vfs_operation {
		($operation:ident) => {
			match vfs.$operation(path) {
				Ok(file) => AssetFile::from(file),
				Err(err) if err.kind() == ::std::io::ErrorKind::NotFound => {
					// The caller may be interested in trying candidate paths that may not
					// exist, so don't consider that an error just yet
					return Ok(outcome_callback(JsonAssetDeserializeOutcome::NotFound));
				}
				Err(err) => return Err(err.into())
			}
		};
	}

	let allow_json_comments =
		always_allow_json_comments || *path == path.with_comment_extension_suffix();

	let mut asset_file = if allow_json_comments {
		// We need to strip comments in this case with a Read adapter, so there is no
		// point in reading the whole file to a in-memory buffer anyway, which is said
		// to be faster to deserialize
		vfs_operation!(open)
	} else {
		vfs_operation!(mmap)
	};

	let canonical_path = path.canonicalize_extension(canonical_extension);

	// Return early if it was or is being processed by another thread (the same asset may be
	// available at several non-canonical paths). If we later error out, it doesn't matter
	// that the file was not actually processed, because PackSquash will halt. We have to
	// mark it after trying to open the file to ignore non-existing alternative candidate
	// paths
	if !squashed_pack_state.mark_file_as_processed(&canonical_path) {
		return Ok(outcome_callback(
			JsonAssetDeserializeOutcome::CanonicalPathAlreadyProcessed
		));
	}

	let fresh_in_previous_zip = squashed_pack_state
		.is_previous_zip_file_fresh(&canonical_path, asset_file.modification_time());

	if fresh_in_previous_zip && !deserialize_if_fresh_in_previous_zip {
		Ok(outcome_callback(
			JsonAssetDeserializeOutcome::FreshInPreviousZip { canonical_path }
		))
	} else {
		let size_hint = asset_file.size_hint();
		let mut deserializer = asset_file.deserializer()?;

		Ok(outcome_callback(JsonAssetDeserializeOutcome::Value {
			value: packsquash_util::deserialize_with_pretty_path_on_error(&mut deserializer)?,
			canonical_path,
			size_hint,
			fresh_in_previous_zip
		}))
	}
}

pub fn serialize<T: Serialize>(
	value: T,
	scratch_files_budget: &ScratchFilesBudget,
	size_hint: usize,
	minify: bool
) -> io::Result<ScratchFile> {
	let mut scratch_file = ScratchFile::with_capacity(scratch_files_budget, size_hint)?;

	if minify {
		let mut serializer = serde_json::ser::Serializer::with_formatter(
			&mut scratch_file,
			CompactFormatterWithShortFloatRepresentation(CompactFormatter)
		);

		value.serialize(&mut serializer)?;
	} else {
		let mut serializer = serde_json::ser::Serializer::with_formatter(
			&mut scratch_file,
			PrettyFormatterWithShortFloatRepresentation(PrettyFormatter::new())
		);

		value.serialize(&mut serializer)?;
	}

	Ok(scratch_file)
}

pub trait Callback<'path, 'data, T: ZeroCopyDeserializable<'data>, R>:
	FnOnce(JsonAssetDeserializeOutcome<'path, T::Type>) -> R
{
}
impl<
		'path,
		'data,
		T: ZeroCopyDeserializable<'data>,
		R,
		F: FnOnce(JsonAssetDeserializeOutcome<'path, T::Type>) -> R
	> Callback<'path, 'data, T, R> for F
{
}

pub trait OutcomeCallback<'path, T: AlwaysZeroCopyDeserializable, R>:
	for<'any> Callback<'path, 'any, T, R>
{
}
impl<'path, T: AlwaysZeroCopyDeserializable, R, F: for<'any> Callback<'path, 'any, T, R>>
	OutcomeCallback<'path, T, R> for F
{
}

enum AssetFile<R: BufRead + Seek> {
	VfsFile(VfsFile<R>),
	VfsMmap(VfsMmap)
}

impl<R: BufRead + Seek> AssetFile<R> {
	fn modification_time(&self) -> Option<SystemTime> {
		match self {
			Self::VfsFile(file) => file.modification_time,
			Self::VfsMmap(mmap) => mmap.modification_time()
		}
	}

	fn size_hint(&self) -> usize {
		match self {
			Self::VfsFile(file) => file
				.file_size
				.map_or_else(|| 0, |file_size| file_size.try_into().unwrap_or(0)),
			Self::VfsMmap(mmap) => mmap.len()
		}
	}

	fn deserializer(
		&mut self
	) -> Result<JsonDeserializer<'_, StripUtf8Bom<StripComments<&mut R>>>, io::Error> {
		Ok(match self {
			Self::VfsFile(file) => {
				JsonDeserializer::ReaderDeserializer(serde_json::Deserializer::from_reader(
					StripComments::new(file.reader.by_ref()).strip_utf8_bom()?
				))
			}
			Self::VfsMmap(mmap) => JsonDeserializer::SliceDeserializer(
				serde_json::Deserializer::from_slice(mmap.strip_utf8_bom().unwrap())
			)
		})
	}
}

impl<R: BufRead + Seek> From<VfsFile<R>> for AssetFile<R> {
	fn from(value: VfsFile<R>) -> Self {
		AssetFile::VfsFile(value)
	}
}

impl<R: BufRead + Seek> From<VfsMmap> for AssetFile<R> {
	fn from(value: VfsMmap) -> Self {
		AssetFile::VfsMmap(value)
	}
}

struct CompactFormatterWithShortFloatRepresentation(CompactFormatter);
struct PrettyFormatterWithShortFloatRepresentation(PrettyFormatter<'static>);

macro_rules! formatter_newtype_wrapper_impl {
	($ty:ident) => {
		impl Formatter for $ty {
			fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_null(writer)
			}

			fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_bool(writer, value)
			}

			fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_i8(writer, value)
			}

			fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_i16(writer, value)
			}

			fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_i32(writer, value)
			}

			fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_i64(writer, value)
			}

			fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_i128(writer, value)
			}

			fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_u8(writer, value)
			}

			fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_u16(writer, value)
			}

			fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_u32(writer, value)
			}

			fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_u64(writer, value)
			}

			fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_u128(writer, value)
			}

			fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				// Internally, Rust uses flt2dec to format floats, which defaults to the shortest
				// representation possible. This is exactly what we want. The JSON specification
				// doesn't distinguish between integers and floats either. serde_json uses ryu by
				// default, which is allegedly faster for floats with more digits, but lots of
				// digits in floats aren't common in Minecraft assets, and omitting the decimal
				// part when possible provides significant space savings on e.g. models
				write!(writer, "{value}")
			}

			fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				write!(writer, "{value}")
			}

			fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_number_str(writer, value)
			}

			fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_string(writer)
			}

			fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_string(writer)
			}

			fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_string_fragment(writer, fragment)
			}

			fn write_char_escape<W>(
				&mut self,
				writer: &mut W,
				char_escape: CharEscape
			) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_char_escape(writer, char_escape)
			}

			fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_array(writer)
			}

			fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_array(writer)
			}

			fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_array_value(writer, first)
			}

			fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_array_value(writer)
			}

			fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_object(writer)
			}

			fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_object(writer)
			}

			fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_object_key(writer, first)
			}

			fn end_object_key<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_object_key(writer)
			}

			fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.begin_object_value(writer)
			}

			fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.end_object_value(writer)
			}

			fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
			where
				W: ?Sized + Write
			{
				self.0.write_raw_fragment(writer, fragment)
			}
		}
	};
}

formatter_newtype_wrapper_impl!(CompactFormatterWithShortFloatRepresentation);
formatter_newtype_wrapper_impl!(PrettyFormatterWithShortFloatRepresentation);

enum JsonDeserializer<'slice, R: Read> {
	ReaderDeserializer(serde_json::Deserializer<serde_json::de::IoRead<R>>),
	SliceDeserializer(serde_json::Deserializer<serde_json::de::SliceRead<'slice>>)
}

impl<'slice, R: Read> Deserializer<'slice> for &mut JsonDeserializer<'slice, R> {
	type Error = serde_json::Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_any(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_any(visitor)
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_bool(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_bool(visitor)
			}
		}
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_i8(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_i8(visitor)
		}
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_i16(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_i16(visitor)
		}
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_i32(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_i32(visitor)
		}
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_i64(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_i64(visitor)
		}
	}

	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_i128(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_i128(visitor)
			}
		}
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_u8(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_u8(visitor)
		}
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_u16(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_u16(visitor)
		}
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_u32(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_u32(visitor)
		}
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_u64(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_u64(visitor)
		}
	}

	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_u128(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_u128(visitor)
			}
		}
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_f32(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_f32(visitor)
		}
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_f64(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_f64(visitor)
		}
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_char(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_char(visitor)
			}
		}
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_str(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_str(visitor)
		}
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_string(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_string(visitor)
			}
		}
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_bytes(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_bytes(visitor)
			}
		}
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_byte_buf(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_byte_buf(visitor)
			}
		}
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_option(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_option(visitor)
			}
		}
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_unit(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_unit(visitor)
			}
		}
	}

	fn deserialize_unit_struct<V>(
		self,
		name: &'static str,
		visitor: V
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_unit_struct(name, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_unit_struct(name, visitor)
			}
		}
	}

	fn deserialize_newtype_struct<V>(
		self,
		name: &'static str,
		visitor: V
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_newtype_struct(name, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_newtype_struct(name, visitor)
			}
		}
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_seq(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_seq(visitor)
		}
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_tuple(len, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_tuple(len, visitor)
			}
		}
	}

	fn deserialize_tuple_struct<V>(
		self,
		name: &'static str,
		len: usize,
		visitor: V
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_tuple_struct(name, len, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_tuple_struct(name, len, visitor)
			}
		}
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_map(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => deserializer.deserialize_map(visitor)
		}
	}

	fn deserialize_struct<V>(
		self,
		name: &'static str,
		fields: &'static [&'static str],
		visitor: V
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_struct(name, fields, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_struct(name, fields, visitor)
			}
		}
	}

	fn deserialize_enum<V>(
		self,
		name: &'static str,
		variants: &'static [&'static str],
		visitor: V
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_enum(name, variants, visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_enum(name, variants, visitor)
			}
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_identifier(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_identifier(visitor)
			}
		}
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'slice>
	{
		match self {
			JsonDeserializer::ReaderDeserializer(deserializer) => {
				deserializer.deserialize_ignored_any(visitor)
			}
			JsonDeserializer::SliceDeserializer(deserializer) => {
				deserializer.deserialize_ignored_any(visitor)
			}
		}
	}
}
