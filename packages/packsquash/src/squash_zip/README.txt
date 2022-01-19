# SquashZip algorithms notes

## File addition
To add files, SquashZip implements the algorithm described in the following steps,
but in a multithreaded way, by using a spooled temporary file per thread instead:

0. Store the current file offset and Squash Time timestamp.
1. Create a local file header.
2. Write as many zero bytes as the local file header will take (i.e. reserve
   space for it in the output file).
3. Write processed, but uncompressed data from the processed_data stream to
   the output file. Store its size and CRC32 hash. Update the local file
   header with the CRC32, uncompressed data size, and Squash Time.
4. Compress the processed data right after the end of the data written in 3.
   Store the "compressed, processed data" size. Store the file offsets where
   both the "compressed, processed data" and "processed data" start.
5. Compare len(compressed, processed data) with len(processed data).
   5.1. If len(compressed, processed data) < len(processed data), move
        "compressed, processed data" to where "processed data" is. Store that
        DEFLATE is the compression method used.
   5.2. If len(compressed, processed data) >= len(processed data), store that
        STORE is the compression method used and make "compressed, processed
        data" equal to the "processed data" size computed in 3.
6. Get entry for map 1) (CRC32 hash, compressed data size) -> (local file header
   offset list). This map stores offsets that point to already written local
   file headers of files with some CRC32 and size combination, so if strict ZIP
   spec conformance is not desired this allows to check whether a file with the
   same data was already added to the ZIP file.
   6.1. If it is in map 1), compare the file data we have just added with the file
        data we have already stored and are pointed to by the offset list.
        6.1.1. If some file data matches (bitwise comparison succeeds), add an
               entry (partial CEN data list, with matched file local header offset)
               to the list 2) and discard the local file header and data added in
               previous steps, by rewinding the output file to the position stored
               in 0. Do not increment output ZIP file size.
        6.1.2. If no file data matches (bitwise comparison fails), push the new
               local file header to the value of the entry described in 6, and
               add an entry to the list 2) with the new local file header offset.
               Increment output ZIP file size by local file header size +
               "compressed, processed data" size. Seek the output file to the
               position stored in 0 and overwrite the local file header with its
               actual, final data. Finally, seek the output file to the position
               where the "compressed, processed data" ends.
   6.2. If it is not in map 1), proceed as in 6.1.2.
Alternate flows:
- If strict ZIP conformance is desired, make 6.1 behave like 6.2. Also, do not
  write Squash Time during 3.
- If no compression is desired, skip 4, treating the processed and uncompressed
  data as if it was compressed.
