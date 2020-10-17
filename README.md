# PackSquash ![Rust CI](https://github.com/ComunidadAylas/PackSquash/workflows/Rust%20CI/badge.svg) ![Latest version](https://img.shields.io/github/v/release/ComunidadAylas/PackSquash?label=Latest%20version) ![GitHub Releases downloads](https://img.shields.io/github/downloads/ComunidadAylas/PackSquash/latest/total?label=Downloads)
A Minecraft resource pack compressor which aims to achieve the best possible compression, which allows for efficient distribution and slightly improved load times in the game, at good speed. Anecdotal evidence shows that it is able to reduce the size of the _Witchcraft & Wizardary_ resource pack by Floo Network (version 1.6.2) from 118 MiB (when compressed as usual) to 57 MiB, a 48.3% size reduction.

## 🔎 How does it work?
PackSquash walks through the resource pack files that it recognizes in a directory, applying per-file configurable lossy and lossless compression techniques, and builds a ZIP file with the results that can be used directly by Minecraft. Currently, PackSquash can apply the following techniques:

* For PNG images: color quantization to generate a 256 color palette, coupled with lossless bit depth, compression and color type reduction. The quantization, although it is pretty subtle or even unneeded for common texture sizes, can be disabled if lossless quality is desired. These operations are performed by the well known `imagequant` (used in `pngquant`) and `oxipng` libraries.
* For OGG, OGA, MP3, FLAC and WAV files: channel mixing, resampling and transcoding. The default settings are meant to be good enough for in-game music, such that most listeners will think that the quality is good and not distracting. Unless configured otherwise, audio files are downmixed to mono, which may change how the Minecraft sound engine computes positional effects (see [MC-146721](https://bugs.mojang.com/browse/MC-146721)), but it is possible to disable this behavior, or even convert input mono files to stereo.
* For JSON files: minification, by removing unneeded whitespace. As a side bonus, because minification requires parsing first, PackSquash also acts as a strict JSON file validator, which also accepts and discard comments. However, it doesn't catch errors that are not against the JSON format but will confuse Minecraft, like duplicated entries in an object.
* For Java property files (only if OptiFine mod support is enabled): minification, by removing unneeded whitespace. As with JSON files, performing minification requires parsing the file, so PackSquash will show basic validation errors.

In addition to these techniques, the files that are not already compressed by design (like OGG and PNG images) are compressed using the Zopfli algorithm, which is a state of the art DEFLATE encoder made by Google. It is tuned for very high space savings at the cost of performance, whilst being compatible with every DEFLATE decoder. For even higher savings it is possible to try to compress already compressed files, but this is very likely to yield marginal savings, if at all.

## 🔗 Download
To get PackSquash, you can build it yourself, like the CI action in this repository does. You will also need to install both the development and runtime GStreamer libraries manually.

Alternatively, you can get a pre-built executable for 64-bit Windows, Linux and macOS systems from [here](https://github.com/ComunidadAylas/PackSquash/releases/latest).

## 📝 Usage
PackSquash is a command line application, so it must be executed from a command prompt, a shortcut, a command-line shell or a script. You can customize how it works by means of a settings file, which contains per-file compression settings and several other parameters. If no settings file is specified, or if it is a dash ("-"), the settings will be read from the standard input stream (usually, your keyboard or the output of another command). The syntax of the arguments accepted by PackSquash is as follows:

```
packsquash [OPTION]... [settings file path]

Options:
    -h, --help          Prints information about the command line arguments
                        accepted by this application and exits
    -v, --version       Prints version information of the application and
                        exits
```

## ✉️ Contact and support
Like the license says, this software is provided without any warranty, with the hope that you find it useful. But that doesn't mean I don't welcome constructive feedback, suggestions, congratulations or assisting you on your usage of PackSquash (if I can and want to). If you wish to drop me a line for whatever reason related to PackSquash, you can contact me on Discord: _AlexTMjugador#5124_.

Also, if you speak Spanish, you may find that _Comunidad Aylas_, our Spanish-speaking community, suits you well. You can join us on Discord: https://discord.gg/RVAgQRS. Don't forget to introduce yourself!
