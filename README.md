<div align="center">
<img src="https://packsquash.aylas.org/logo/kawaii.png" alt="PackSquash" width="600">

<a
href="https://github.com/ComunidadAylas/PackSquash/actions?query=workflow%3ACI"><img
alt="CI workflow status"
src="https://github.com/ComunidadAylas/PackSquash/actions/workflows/ci.yml/badge.svg"></a>
<a href="https://github.com/ComunidadAylas/PackSquash/releases/latest"><img
alt="Latest version"
src="https://img.shields.io/github/v/release/ComunidadAylas/PackSquash?label=Latest%20version"></a>
<img alt="Total downloads"
src="https://img.shields.io/github/downloads/ComunidadAylas/PackSquash/total?label=Downloads">
<a href="https://discord.gg/bGUSamzJYp"><img alt="Discord server online count"
src="https://img.shields.io/discord/85364538328768512?label=Discord&logo=discord"></a>
</div>

PackSquash is a Minecraft: Java Edition resource and data pack optimizer which
aims to achieve the best possible compression, performance, and protection,
improving pack distribution, storage, and in-game load times.

Anecdotal evidence shows that, with the default options, version v0.2.1 was
capable of reducing the size of the _Witchcraft & Wizardary_ resource pack ZIP
file by Floo Network (version 1.6.2) from 118 MiB to 57 MiB, a 51.69% size
reduction, and it got better over time.

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

Alternatively, you might be interested in third-party tools that integrate with
PackSquash, such as:

- [Nexo](https://docs.nexomc.com/configuration/resourcepack#packsquash-integration),
  a Minecraft server plugin for creating custom items, blocks, armor, and more.
- [Git Pack Manager](https://git-pack-manager.docs.offsetmonkey538.top/guides/packsquash),
  a Minecraft server mod/plugin for managing the server's resource pack using git.

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
some money would be a great way to do so!

<p align="center"><a href="https://ko-fi.com/K3K758Q08"><img src="https://cdn.ko-fi.com/cdn/kofi2.png?v=2" alt="Buy me a coffee at ko-fi.com" height="36"/></a></p>
<p align="center"><a href="https://www.paypal.me/alejandrogonzalezg98"><img src="https://www.bathtownshippubliclibrary.org/site-assets/images/paypal-donate-button-high-quality-png.png/@@images/image.png" alt="Donate via PayPal.me" height="64"/></a></p>

In principle, I will use any funds I receive for personal spending. However, I
may decide to introduce sponsor tiers and perks (retroactively, benefiting those
who sponsored the project before they got introduced too), add more payment
methods, share the funds with other high-profile contributors, or use them for
donations and campaigns. I will be transparent about any major decisions I make
about this, communicating them in public announcements.

## ✨ Contributors

Thanks goes to these wonderful people and projects ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://alegon.dev/"><img src="https://avatars.githubusercontent.com/u/7822554?v=4?s=75" width="75px;" alt=" Alejandro González"/><br /><sub><b> Alejandro González</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/commits?author=AlexTMjugador" title="Code">💻</a> <a href="#content-AlexTMjugador" title="Content">🖋</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=AlexTMjugador" title="Documentation">📖</a> <a href="#example-AlexTMjugador" title="Examples">💡</a> <a href="#maintenance-AlexTMjugador" title="Maintenance">🚧</a> <a href="#projectManagement-AlexTMjugador" title="Project Management">📆</a> <a href="#mentoring-AlexTMjugador" title="Mentoring">🧑‍🏫</a> <a href="#question-AlexTMjugador" title="Answering Questions">💬</a> <a href="#research-AlexTMjugador" title="Research">🔬</a> <a href="#translation-AlexTMjugador" title="Translation">🌍</a> <a href="https://github.com/ComunidadAylas/PackSquash/pulls?q=is%3Apr+reviewed-by%3AAlexTMjugador" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=AlexTMjugador" title="Tests">⚠️</a> <a href="#promotion-AlexTMjugador" title="Promotion">📣</a> <a href="#platform-AlexTMjugador" title="Packaging/porting to new platform">📦</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Aiamded"><img src="https://avatars.githubusercontent.com/u/71924226?v=4?s=75" width="75px;" alt="Aiamded"/><br /><sub><b>Aiamded</b></sub></a><br /><a href="#ideas-Aiamded" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3AAiamded" title="Bug reports">🐛</a> <a href="#data-Aiamded" title="Data">🔣</a> <a href="#userTesting-Aiamded" title="User Testing">📓</a> <a href="#financial-Aiamded" title="Financial">💵</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://lx862.com/"><img src="https://avatars.githubusercontent.com/u/28094366?v=4?s=75" width="75px;" alt="AmberFrost"/><br /><sub><b>AmberFrost</b></sub></a><br /><a href="#ideas-amberisfrozen" title="Ideas, Planning, & Feedback">🤔</a> <a href="#design-amberisfrozen" title="Design">🎨</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/benwickham"><img src="https://avatars.githubusercontent.com/u/137005985?v=4?s=75" width="75px;" alt="Ben Wickham (Eraze)"/><br /><sub><b>Ben Wickham (Eraze)</b></sub></a><br /><a href="#financial-benwickham" title="Financial">💵</a> <a href="#ideas-benwickham" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/CallumBugajski"><img src="https://avatars.githubusercontent.com/u/11320476?v=4?s=75" width="75px;" alt="CallumBugajski"/><br /><sub><b>CallumBugajski</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3ACallumBugajski" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ChenCMD"><img src="https://avatars.githubusercontent.com/u/46134240?v=4?s=75" width="75px;" alt="Chen"/><br /><sub><b>Chen</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3AChenCMD" title="Bug reports">🐛</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=ChenCMD" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://luedt.ke/"><img src="https://avatars.githubusercontent.com/u/13185260?v=4?s=75" width="75px;" alt="David Lüdtke"/><br /><sub><b>David Lüdtke</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3AMrKinau" title="Bug reports">🐛</a> <a href="#ideas-MrKinau" title="Ideas, Planning, & Feedback">🤔</a> <a href="#userTesting-MrKinau" title="User Testing">📓</a> <a href="#question-MrKinau" title="Answering Questions">💬</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Felixx61"><img src="https://avatars.githubusercontent.com/u/4580537?v=4?s=75" width="75px;" alt="Felixx61"/><br /><sub><b>Felixx61</b></sub></a><br /><a href="#userTesting-Felixx61" title="User Testing">📓</a> <a href="#data-Felixx61" title="Data">🔣</a> <a href="#ideas-Felixx61" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/mergu"><img src="https://avatars.githubusercontent.com/u/29823405?v=4?s=75" width="75px;" alt="Jay Hopkins"/><br /><sub><b>Jay Hopkins</b></sub></a><br /><a href="#userTesting-mergu" title="User Testing">📓</a> <a href="#data-mergu" title="Data">🔣</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Joshinn-io"><img src="https://avatars.githubusercontent.com/u/39896452?v=4?s=75" width="75px;" alt="Joshua"/><br /><sub><b>Joshua</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3AJoshinn-io" title="Bug reports">🐛</a> <a href="#financial-Joshinn-io" title="Financial">💵</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.planetminecraft.com/member/luish54/"><img src="https://avatars.githubusercontent.com/u/103937012?v=4?s=75" width="75px;" alt="Luis"/><br /><sub><b>Luis</b></sub></a><br /><a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3ALuishMC" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MKMakroM"><img src="https://avatars.githubusercontent.com/u/45736418?v=4?s=75" width="75px;" alt="MKMakroM"/><br /><sub><b>MKMakroM</b></sub></a><br /><a href="#financial-MKMakroM" title="Financial">💵</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://manacube.com/"><img src="https://i.imgur.com/D9gJEr6.png?s=75" width="75px;" alt="ManaCube"/><br /><sub><b>ManaCube</b></sub></a><br /><a href="#financial" title="Financial">💵</a> <a href="https://github.com/ComunidadAylas/PackSquash/issues?q=author%3A" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://iptables.sh/"><img src="https://avatars.githubusercontent.com/u/28601081?v=4?s=75" width="75px;" alt="Michael"/><br /><sub><b>Michael</b></sub></a><br /><a href="#data-clrxbl" title="Data">🔣</a> <a href="#ideas-clrxbl" title="Ideas, Planning, & Feedback">🤔</a> <a href="#userTesting-clrxbl" title="User Testing">📓</a> <a href="#fundingFinding-clrxbl" title="Funding Finding">🔍</a> <a href="#tool-clrxbl" title="Tools">🔧</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://discord.gg/RVAgQRS"><img src="https://avatars.githubusercontent.com/u/31966940?v=4?s=75" width="75px;" alt="Miguel"/><br /><sub><b>Miguel</b></sub></a><br /><a href="#design-MiguelDreamer" title="Design">🎨</a> <a href="#ideas-MiguelDreamer" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/xMikux"><img src="https://avatars.githubusercontent.com/u/26039249?v=4?s=75" width="75px;" alt="Miku"/><br /><sub><b>Miku</b></sub></a><br /><a href="#translation-xMikux" title="Translation">🌍</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=xMikux" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MinecraftAdmin"><img src="https://avatars.githubusercontent.com/u/1650123?v=4?s=75" width="75px;" alt="MinecraftAdmin"/><br /><sub><b>MinecraftAdmin</b></sub></a><br /><a href="#ideas-MinecraftAdmin" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://paulf.me/"><img src="https://avatars.githubusercontent.com/u/5201207?v=4?s=75" width="75px;" alt="Paul Fulham"/><br /><sub><b>Paul Fulham</b></sub></a><br /><a href="#ideas-pau101" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=pau101" title="Code">💻</a> <a href="#research-pau101" title="Research">🔬</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Derkades"><img src="https://avatars.githubusercontent.com/u/15892014?v=4?s=75" width="75px;" alt="Robin"/><br /><sub><b>Robin</b></sub></a><br /><a href="#ideas-Derkades" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=Derkades" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Silmarost"><img src="https://avatars.githubusercontent.com/u/52896544?v=4?s=75" width="75px;" alt="Silmarost"/><br /><sub><b>Silmarost</b></sub></a><br /><a href="#data-Silmarost" title="Data">🔣</a> <a href="#ideas-Silmarost" title="Ideas, Planning, & Feedback">🤔</a> <a href="#userTesting-Silmarost" title="User Testing">📓</a> <a href="#financial-Silmarost" title="Financial">💵</a> <a href="#translation-Silmarost" title="Translation">🌍</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/SuperJohan16"><img src="https://avatars.githubusercontent.com/u/73500074?v=4?s=75" width="75px;" alt="SuperJohan16"/><br /><sub><b>SuperJohan16</b></sub></a><br /><a href="#translation-SuperJohan16" title="Translation">🌍</a> <a href="#financial-SuperJohan16" title="Financial">💵</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://alumina6767.net/"><img src="https://avatars.githubusercontent.com/u/85728971?v=4?s=75" width="75px;" alt="alumina6767"/><br /><sub><b>alumina6767</b></sub></a><br /><a href="#blog-alumina6767" title="Blogposts">📝</a> <a href="#translation-alumina6767" title="Translation">🌍</a> <a href="#ideas-alumina6767" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/pacificminer"><img src="https://avatars.githubusercontent.com/u/24826002?v=4?s=75" width="75px;" alt="pacificminer"/><br /><sub><b>pacificminer</b></sub></a><br /><a href="#financial-pacificminer" title="Financial">💵</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/realkarmakun"><img src="https://avatars.githubusercontent.com/u/20980281?v=4?s=75" width="75px;" alt="realkarmakun"/><br /><sub><b>realkarmakun</b></sub></a><br /><a href="#platform-realkarmakun" title="Packaging/porting to new platform">📦</a> <a href="#ideas-realkarmakun" title="Ideas, Planning, & Feedback">🤔</a> <a href="#userTesting-realkarmakun" title="User Testing">📓</a> <a href="https://github.com/ComunidadAylas/PackSquash/commits?author=realkarmakun" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/rswishart"><img src="https://avatars.githubusercontent.com/u/115955715?v=4?s=75" width="75px;" alt="rswishart"/><br /><sub><b>rswishart</b></sub></a><br /><a href="#financial-rswishart" title="Financial">💵</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/victorlf4"><img src="https://avatars.githubusercontent.com/u/33629877?v=4?s=75" width="75px;" alt="victorlf4"/><br /><sub><b>victorlf4</b></sub></a><br /><a href="#ideas-victorlf4" title="Ideas, Planning, & Feedback">🤔</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the
[all-contributors](https://github.com/all-contributors/all-contributors)
specification. Contributions of any kind welcome! If you are interested, you can
check out the [contributing
guidelines](https://github.com/ComunidadAylas/PackSquash/blob/master/CONTRIBUTING.md)
to get started.
