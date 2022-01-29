#!/usr/bin/env python3
# Samples a pack, generating another that contains a random subset of its
# files, making it more amenable for microbenchmarking.

from os import makedirs, path, walk
from os.path import abspath, dirname, join
import sys
from shutil import copyfile
from random import sample


def samplePack(input_path, output_path, sample_size=None):
    input_path = abspath(input_path)
    output_path = abspath(output_path)
    input_path_prefix_len = len(input_path) + len(path.sep)

    makedirs(output_path, exist_ok=True)

    pack_files = []
    for current_dir, _, files in walk(input_path, followlinks=True):
        for filename in files:
            if current_dir is not input_path:
                filepath_input = join(current_dir, filename)
                filepath_output = join(
                    output_path, current_dir[input_path_prefix_len:], filename)

                pack_files.append((filepath_input, filepath_output))
            else:
                filepath_input = join(current_dir, filename)
                filepath_output = join(
                    output_path, current_dir[input_path_prefix_len:], filename)

                copyfile(filepath_input, filepath_output)

    if sample_size == None:
        sample_size = int(len(pack_files) / 10)

    pack_files = sample(pack_files, sample_size)

    for filepath_input, filepath_output in pack_files:
        output_path_dir = join(
            output_path, dirname(filepath_input[input_path_prefix_len:]))

        makedirs(join(output_path, output_path_dir), exist_ok=True)
        copyfile(filepath_input, filepath_output)

    print("Sample pack created")


if __name__ == "__main__":
    if len(sys.argv) == 4:
        samplePack(sys.argv[1], sys.argv[2], int(sys.argv[3]))
    elif len(sys.argv) == 3:
        samplePack(sys.argv[1], sys.argv[2])
    else:
        print("Usage: sample-pack.py <input_path> <output_path> [sample_size]")
        sys.exit(1)
