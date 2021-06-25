use tempfile::SpooledTempFile;

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn works() {
	let data = [0, 1, 2, 3];
	let mut buf = [0; 4];
	let spool_buffer_sizes = [2, 4];

	for spool_buffer_size in &spool_buffer_sizes {
		let mut file = SpooledTempFile::new(*spool_buffer_size);

		file.write_all(&data)
			.expect("No error should happen while writing to the temporary file");

		file.seek(SeekFrom::Start(0))
			.expect("No error should happen while seeking the temporary file");

		// Test interleaved read/writes that don't span the entire file
		{
			let file_decorator = DoubleOffsetStreamDecorator::new(&mut file, 0, 1);

			(&file_decorator)
				.write_all(&[2]) // Should write at offset 1
				.expect("No error should happen while writing from the temporary file");

			(&file_decorator)
				.read_exact(&mut buf[..1]) // Should read from offset 0
				.expect("No error should happen while reading from the temporary file");

			assert_eq!(0, buf[0], "Unexpected byte read");

			file.seek(SeekFrom::Start(0))
				.expect("No error should happen while seeking the temporary file");

			file.read_exact(&mut buf)
				.expect("No error should happen while reading from the temporary file");

			assert_eq!([0, 2, 2, 3], buf, "Unexpected bytes read after write");
		}

		file.seek(SeekFrom::Start(0))
			.expect("No error should happen while seeking the temporary file");

		// Test multibyte interleaved read/writes that span the entire file
		{
			let file_decorator = DoubleOffsetStreamDecorator::new(&mut file, 0, 2);

			(&file_decorator)
				.write_all(&[4]) // Should write at offset 2
				.expect("No error should happen while writing from the temporary file");

			(&file_decorator)
				.read_exact(&mut buf[..1]) // Should read from offset 0
				.expect("No error should happen while reading from the temporary file");

			(&file_decorator)
				.write_all(&[6]) // Should write at offset 3
				.expect("No error should happen while writing from the temporary file");

			assert_eq!(
				4,
				file_decorator.get_write_position(),
				"Unexpected file write position after read and write operations"
			);

			(&file_decorator)
				.read_exact(&mut buf[1..2]) // Should read from offset 1
				.expect("No error should happen while reading from the temporary file");

			assert_eq!([0, 2], buf[0..2], "Unexpected resulting file contents");

			file.seek(SeekFrom::Start(0))
				.expect("No error should happen while seeking the temporary file");

			file.read_exact(&mut buf)
				.expect("No error should happen while reading from the temporary file");

			assert_eq!([0, 2, 4, 6], buf, "Unexpected resulting file contents");
		}
	}
}
