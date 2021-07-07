<p align="center"><img src="https://user-images.githubusercontent.com/31966940/124388201-1f40eb80-dce2-11eb-88e8-3934d7d73c0a.png" alt="PackSquash logo" width="300" height="300"></p>
<p align="center"><i>Create more. Worry less about distribution and storage intricacies.</i></p>

# PackSquash [![Build](https://github.com/ComunidadAylas/PackSquash/workflows/Build/badge.svg)](https://github.com/ComunidadAylas/PackSquash/actions?query=workflow%3ABuild) [![Latest version](https://img.shields.io/github/v/release/ComunidadAylas/PackSquash?label=Latest%20version)](https://github.com/ComunidadAylas/PackSquash/releases/latest) [![Total downloads](https://img.shields.io/github/downloads/ComunidadAylas/PackSquash/total?label=Downloads)](https://github.com/ComunidadAylas/PackSquash/releases/latest) [![Discord server online count](https://img.shields.io/discord/85364538328768512?label=Discord&logo=discord)](https://discord.gg/RVAgQRS)

A Minecraft resource and data pack optimizer which aims to achieve the best possible compression, performance and protection, improving pack distribution, storage and in-game load times. Anecdotal evidence shows that, with the default options, version v0.2.1 was capable of reducing the size of the _Witchcraft & Wizardary_ resource pack ZIP file by Floo Network (version 1.6.2) from 118 MiB to 57 MiB, a 51.69% size reduction, and it got better over time.

## 🔎 How does it work?
PackSquash walks through the pack files that it recognizes in a directory, applying per-file configurable lossy and lossless compression techniques, and builds a ZIP file with the results that can be directly used by Minecraft. Currently, PackSquash can apply the following techniques:

* For PNG images: color quantization to generate a color palette, coupled with lossless bit depth, compression and color type reduction, and metadata removal. The quantization, although it is pretty subtle or even unneeded for common texture sizes, can be disabled if lossless quality is desired. These operations are performed by the well known `imagequant` (used in `pngquant`) and `oxipng` libraries.
* For Ogg, Oga, MP3, Opus, FLAC and WAV files: channel mixing, resampling, transcoding, pitch shifting and tag removal. The default settings are meant to be good enough for in-game music, such that most listeners will think that the quality is good and not distracting. Because channel mixing may change how the Minecraft sound engine computes positional effects (see [MC-146721](https://bugs.mojang.com/browse/MC-146721)), PackSquash won't do it by default.
* For JSON files: minification, by removing unneeded whitespace. As a side bonus, because minification requires parsing first, PackSquash also acts as a strict JSON file validator, which also accepts and discards comments. However, it doesn't catch errors that are not against the JSON format but can confuse Minecraft.
* For VSH and FSH shader files: minification, by removing unneeded whitespace. As with JSON files, PackSquash will perform basic validation on them.
* For Java property files (only if OptiFine mod support is enabled): minification, by removing unneeded whitespace. As with JSON files, performing minification requires parsing the file, so PackSquash will show basic validation errors.

In addition to these techniques, the files that are not already compressed by design (namely, OGG and PNG files) are losslessly compressed using the Zopfli algorithm, which is a state of the art DEFLATE encoder made by Google. It is tuned for very high space savings at the cost of performance, whilst being compatible with every DEFLATE decoder. For even higher savings it is possible to try to compress already compressed files and increase the number of compression iterations, although this slows down the process significantly. Conversely, if lower savings are acceptable in exchange for faster execution, it is possible to reduce the number of compression iterations or skip it altogether.

PackSquash is also capable of doing the following things, although they are disabled by default because they require informed decisions to be made by the user:

* Reusing the ZIP files it previously generated to skip compressing files that did not change between executions, which vastly speeds up its execution for incremental workflows, where only a few files change each time.
* Deduplicating identical files within the generated ZIP file. This means that the contents of files that are repeated several times over will be stored only once.
* Protecting the generated ZIP files, making them harder to read by most ZIP manipulation programs.

## 🔗 Download
You can get the executable for the latest stable release from [here](https://github.com/ComunidadAylas/PackSquash/releases/latest).

Alternatively, if you are into these sort of things, you can download the latest unstable build from [GitHub Actions](https://github.com/ComunidadAylas/PackSquash/actions?query=branch%3Amaster), or build the source yourself.

## 📝 Usage
PackSquash is a command line application, so it must be executed from a command prompt, a shortcut, a command-line shell or a script. You can customize how it works by means of a settings file, which contains per-file compression settings and several other parameters. If no settings file is specified, or if it is a dash ("-"), the settings will be read from the standard input stream (usually, your keyboard or the output of another command). If in doubt, you can check out the command line argument syntax by launching PackSquash with the `-h` parameter.

For more information about the format of the settings file, check [the wiki article about it](https://github.com/ComunidadAylas/PackSquash/wiki/Settings-file-format).

If you want a GUI for using PackSquash, or even authoring resource packs in general, you may want to try out the [Quiver](https://github.com/DeflatedPickle/Quiver) project, which is a third-party resource pack creator and manager for Minecraft that integrates with PackSquash. Just install PackSquash dependencies, open the resource pack folder, tweak the settings file if you need to, and enjoy!

## ✉️ Contact and support
Like the license says, this software is provided without any warranty, with the hope that you find it useful. But that doesn't mean I don't welcome constructive feedback, suggestions, congratulations or assisting you on your usage of PackSquash (if I can and want to). If you wish to drop me a line for whatever reason related to PackSquash, you can contact me on Discord: _AlexTMjugador#5124_.

Also, if you speak Spanish, you may find that _Comunidad Aylas_, our Spanish-speaking community, suits you well. You can [join us on Discord](https://discord.gg/RVAgQRS). Don't forget to introduce yourself!
