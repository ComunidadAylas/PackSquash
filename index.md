---
layout: default
title: Home
show_download: true
---

<p align="center"><img src="https://user-images.githubusercontent.com/7822554/243825966-500faa74-de2f-462a-8f76-3a122b69856e.png" alt="PackSquash logo" width="300" height="300"></p>
<p align="center"><i>Create more. Worry less about distribution and storage intricacies.</i></p>

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

- For PNG images: color quantization to generate a color palette, lossless bit
  depth, compression and color type reduction, metadata removal, and downsizing
  of single-color textures. The quantization, although it is pretty subtle or
  even unneeded for common texture sizes, can be disabled if lossless quality is
  desired. Downsizing can also be turned off. Some of these optimizations are
  done by the well-known `imagequant` (used in `pngquant`) and `oxipng`
  libraries.
- For Ogg (.ogg and .oga), MP3, M4A, FLAC, and WAV files: channel mixing,
  downsampling, transcoding with a state-of-the-art Vorbis encoder, pitch
  shifting, tag removal, silence truncation, and two-pass optimization and
  validation with [OptiVorbis](https://github.com/OptiVorbis/OptiVorbis). The
  default settings are meant to be good enough for in-game music, such that most
  listeners will think that the quality is good and not distracting. Because
  channel mixing may change how the Minecraft sound engine computes positional
  effects (see [MC-146721](https://bugs.mojang.com/browse/MC-146721)),
  PackSquash won't do it by default.
- For JSON files (.json, .jsonc, .mcmeta and .mcmetac; if OptiFine mod support
  is enabled, also .jem, .jemc, .jpm and .jpmc): minification, by removing
  unneeded whitespace. As a side bonus, because minification requires parsing
  first, PackSquash also acts as a strict JSON file validator. Comments are the
  only syntax extension accepted, and they are discarded by default, although
  they can be rejected too in files whose extension does not end with a "c"
  letter. The validation done doesn't catch all the possible errors that might
  confuse Minecraft.
- For shader files (.vsh, .fsh and .glsl): minification, by removing unneeded
  whitespace, and resolution and expansion of preprocessor directives. As with
  JSON files, PackSquash will perform basic validation on them. Please note that
  due to limitations in the current design of PackSquash [some shaders may
  require
  tweaking](https://github.com/ComunidadAylas/PackSquash/issues/187#issuecomment-1499365532).
  These limitations will be addressed in the future.
- For structure files (.nbt): recompression with Zopfli, Gzip member
  concatenation, and metadata removal.
- For legacy language files (.lang; used in Minecraft 1.12.2 and lower):
  minification, removing blank lines and comments. Duplicate keys and non-blank
  lines that are not comments and do not contain a key-value separator are
  treated as errors, which helps catch mistakes and keep your files tidy.
- For OptiFine properties files (Java .properties; only if OptiFine mod support
  is enabled): minification, by removing unneeded whitespace. As with JSON
  files, performing minification requires parsing the file, so PackSquash will
  show basic validation errors.
- For BlockBench model files (.bbmodel; only if Minecraft Transit Railway 3
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

- Reusing the ZIP files it previously generated to skip compressing files that
  did not change between executions, vastly speeding it up for incremental
  workflows, where only a few files change each time.
- Deduplicating identical files within the generated ZIP file: the contents of
  files repeated several times over will be stored only once.
- Protecting the generated ZIP files, making them harder to read by most ZIP
  manipulation programs, and some of the files stored inside such ZIP files.

## 🔗 Download and usage

PackSquash is a command-line application distributed for a bunch of operating
systems and environments. Check out the [getting started
guide](https://github.com/ComunidadAylas/PackSquash/wiki/Getting-started) for
details on how to begin your journey with PackSquash.

Alternatively, you might be interested in third-party frontends for PackSquash,
such as:

- [Quiver](https://github.com/DeflatedPickle/Quiver), a resource pack creator
  and manager. At the time of writing, Quiver is using an older, unsupported
  version of PackSquash, so it may not be representative of the experience with
  current versions. If you have the time, will, and know-how, you might want to
  help this project update its integration!

## ✉️ Contact and support

As the license says, this software is provided without any warranty, with the
hope that you find it useful. But that doesn't mean I don't welcome constructive
feedback, suggestions, congratulations, or assisting you on your use of
PackSquash (if I can and want to). If you wish to drop me a line for whatever
reason related to PackSquash, you can contact me on Discord: _alextmjugador_.

You can also [join the Discord server of our
community](https://discord.gg/bGUSamzJYp), _Comunidad Aylas_, which has
dedicated spaces to welcome English-language PackSquash chat and support, among
other topics. Don't forget to introduce yourself!

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
