<p align="center"><img src="https://user-images.githubusercontent.com/7822554/243825966-500faa74-de2f-462a-8f76-3a122b69856e.png" alt="PackSquash logo" width="300" height="300"></p>
<p align="center"><i>Create more. Worry less about distribution and storage intricacies.</i></p>

# PackSquash [![Build](https://github.com/ComunidadAylas/PackSquash/workflows/CI/badge.svg)](https://github.com/ComunidadAylas/PackSquash/actions?query=workflow%3ACI) [![Latest version](https://img.shields.io/github/v/release/ComunidadAylas/PackSquash?label=Latest%20version)](https://github.com/ComunidadAylas/PackSquash/releases/latest) [![Total downloads](https://img.shields.io/github/downloads/ComunidadAylas/PackSquash/total?label=Downloads)](https://github.com/ComunidadAylas/PackSquash/releases/latest) [![Discord server online count](https://img.shields.io/discord/85364538328768512?label=Discord&logo=discord)](https://discord.gg/bGUSamzJYp)

A Minecraft: Java Edition resource and data pack optimizer which aims to achieve
the best possible compression, performance, and protection, improving pack
distribution, storage, and in-game load times. Anecdotal evidence shows that,
with the default options, version v0.2.1 was capable of reducing the size of the
_Witchcraft & Wizardary_ resource pack ZIP file by Floo Network (version 1.6.2)
from 118 MiB to 57 MiB, a 51.69% size reduction, and it got better over time.

## 🔎 Overview

PackSquash walks through the pack files that it recognizes in a directory,
applies per-file configurable lossy and lossless compression techniques, and
builds a ZIP file that can be directly used by the game. Currently, PackSquash
can apply the following specific techniques:

* For PNG images: color quantization to generate a color palette, lossless bit
  depth, compression and color type reduction, and metadata removal. The
  quantization, although it is pretty subtle or even unneeded for common texture
  sizes, can be disabled if lossless quality is desired. These operations are
  performed by the well-known `imagequant` (used in `pngquant`) and `oxipng`
  libraries.
* For Ogg (.ogg and .oga), MP3, Opus, FLAC, and WAV files: channel mixing,
  downsampling, transcoding, pitch shifting, and tag removal. The default
  settings are meant to be good enough for in-game music, such that most
  listeners will think that the quality is good and not distracting. Because
  channel mixing may change how the Minecraft sound engine computes positional
  effects (see [MC-146721](https://bugs.mojang.com/browse/MC-146721)),
  PackSquash won't do it by default.
* For JSON files (.json, .jsonc, .mcmeta and .mcmetac; if OptiFine mod support
  is enabled, also .jem, .jemc, .jpm and .jpmc): minification, by removing
  unneeded whitespace. As a side bonus, because minification requires parsing
  first, PackSquash also acts as a strict JSON file validator. Comments are the
  only syntax extension accepted, and they are discarded by default, although
  they can be rejected too in files whose extension does not end with a "c"
  letter. The validation done doesn't catch all the possible errors that might
  confuse Minecraft.
* For shader files (.vsh, .fsh and .glsl): minification, by removing unneeded
  whitespace. As with JSON files, PackSquash will perform basic validation on
  them.
* For legacy language files (.lang; used in Minecraft 1.12.2 and lower):
  minification, removing blank lines and comments. Duplicate keys and non-blank
  lines that are not comments and do not contain a key-value separator are
  treated as errors, which helps catch mistakes and keep your files tidy.
* For OptiFine properties files (Java .properties; only if OptiFine mod support
  is enabled): minification, by removing unneeded whitespace. As with JSON
  files, performing minification requires parsing the file, so PackSquash will
  show basic validation errors.
* For BlockBench model files (.bbmodel; only if Minecraft Transit Railway 3
  support is enabled): minification, by removing unneeded whitespace and keys.
  The minification might also improve privacy, as some keys containing
  environment-specific metadata are removed.

In addition to these techniques, the files that are not already compressed by
design (namely, Ogg and PNG files) are losslessly compressed using the Zopfli
algorithm, which is a state-of-the-art DEFLATE encoder made by Google. It is
tuned for very high space savings at the cost of performance whilst being
compatible with every DEFLATE decoder. For even higher savings, it is possible
to compress already compressed files and increase the number of compression
iterations, although this slows down the process significantly. Conversely, if
lower savings are acceptable in exchange for faster execution, it is possible to
reduce the number of compression iterations or skip it altogether.

PackSquash is also capable of doing the following things, although they are
disabled by default because they require informed decisions to be made by the
user:

* Reusing the ZIP files it previously generated to skip compressing files that
  did not change between executions, vastly speeding it up for incremental
  workflows, where only a few files change each time.
* Deduplicating identical files within the generated ZIP file: the contents of
  files repeated several times over will be stored only once.
* Protecting the generated ZIP files, making them harder to read by most ZIP
  manipulation programs.

## 🔗 Download and installation

PackSquash is distributed for a bunch of operating systems and environments.
Check out the [installation
guide](https://github.com/ComunidadAylas/PackSquash/wiki/Installation-guide) for
details on how to download and get PackSquash.

## 📝 Usage

PackSquash is a command-line application, so it must be executed from a command
prompt, a shortcut, a command-line shell, or a script. You can customize how it
works via
[options](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files). In
addition, it also accepts some command-line arguments. You can know about these
by launching PackSquash with the `-h` or `--help` argument.

If you want a GUI for using PackSquash or even authoring resource packs in
general, you may want to try out the
[Quiver](https://github.com/DeflatedPickle/Quiver) project, which is a
third-party resource pack creator and manager for Minecraft that integrates with
PackSquash. Just install PackSquash dependencies, open the resource pack folder,
tweak the options file if you need to, and enjoy!

## ✉️ Contact and support

Like the license says, this software is provided without any warranty, with the
hope that you find it useful. But that doesn't mean I don't welcome constructive
feedback, suggestions, congratulations, or assisting you on your use of
PackSquash (if I can and want to). If you wish to drop me a line for whatever
reason related to PackSquash, you can contact me on Discord:
_AlexTMjugador#5124_.

Also, if you speak Spanish, you may find that our Spanish-speaking community,
_Comunidad Aylas_, suits you well. You can [join us on
Discord](https://discord.gg/bGUSamzJYp). Don't forget to introduce yourself!

## 🎁 Sponsoring

You can use all of PackSquash for free for whatever you want and will always be
able to. One of the things I enjoy the most about making free software is the
feeling that I'm doing something good and useful for others. With that said,
PackSquash is arguably innovative, and some things were only possible to
implement thanks to original knowledge of Minecraft internals acquired via
static analysis of its deobfuscated code, which is an activity that is commonly
referred to as reverse engineering. This is time-consuming, and engineers in
companies get paid for doing these kinds of things. If you want to say "thank
you" in a way that words can't describe or motivate me to stay committed to the
project and advance in the
[roadmap](https://github.com/ComunidadAylas/PackSquash/projects/1), sending me
some money would be a great way to do so! You can choose any of the platforms
shown below:

<p align="center"><a href="https://ko-fi.com/K3K758Q08"><img src="https://cdn.ko-fi.com/cdn/kofi2.png?v=2" alt="Buy me a coffee at ko-fi.com" height="36"/></a></p>
<p align="center"><a href="https://www.paypal.me/alejandrogonzalezg98"><img src="https://icon-library.com/images/paypal-donate-icon/paypal-donate-icon-7.jpg" alt="Donate via PayPal.me" height="64"/></a></p>

In principle, I will use any funds I receive for personal spending. However, I
may decide to introduce sponsor tiers and perks (retroactively, benefiting those
who sponsored the project before they got introduced too), add more payment
methods, share the funds with other high-profile contributors, or use them for
donations and campaigns. I will be transparent about any major decisions I make
about this, communicating them in public announcements.

## ✅ Acknowledgments

Special thanks to JetBrains s.r.o. for providing core PackSquash developers
with a free open source development license for CLion and other development
tools.

<p align="center"><a href="https://jb.gg/OpenSourceSupport"><img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jb_square.svg" alt="JetBrains logo"/></a></p>
