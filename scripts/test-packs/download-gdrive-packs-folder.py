#!/usr/bin/env python3
# Low-level Python script for downloading test packs from Google Drive folders.
# If in doubt, please use download.sh instead.

from gdown import download_folder
from pathlib import Path

# The ID of the Google Drive folder from which test packs will be downloaded.
FOLDER_ID = "1HaaNug7Vd-SZromvU9VR7vWo3MmO-_qn"

if __name__ == "__main__":
	git_dir = None
	for candidate_git_dir in Path(__file__).parents:
		if candidate_git_dir.joinpath(".git").exists():
			git_dir = candidate_git_dir
			break

	if git_dir is None:
		raise AssertionError(
			"Could not find the root Git repository directory. Was this script moved out of its repository?"
		)

	print("> Getting test packs...")

	downloaded_files = download_folder(
		f"https://drive.google.com/drive/folders/{FOLDER_ID}",
		use_cookies=False,
		output=str(
			git_dir.joinpath("packages", "packsquash", "benches", "data",
							 "packs")))

	print(
		f"> Successfully downloaded {len(downloaded_files)} pack(s)"
	)
