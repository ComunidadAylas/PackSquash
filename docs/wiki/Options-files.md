> [!IMPORTANT]
> **This document is for PackSquash version v0.4.1**. Different versions may use
> incompatible sets of options. If you have to use an older version, something
> that we don't recommend or support, please refer to the documentation
> appropriate for that version:
>
> - [v0.4.0](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files/daba53cb4368f0920cee5ca4560f5d4ba2e6032d).
> - [v0.3.1](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files/0237e66d78cebfa2c3d131612eb2dcc5bf89d2ec).
> - [v0.3.0](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files/1bf83590c971446817ca4a1ba718c98036a9f1b8).
> - [v0.2.1](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files/ba9eb71a23ba0e446f8aec7ba0e2aa092641b6b0).

<!-- omit from toc -->
## Table of contents

- [Introduction](#introduction)
- [How to pass options](#how-to-pass-options)
- [Global options](#global-options)
  - [`pack_directory`](#pack_directory)
  - [`output_file_path`](#output_file_path)
  - [`recompress_compressed_files`](#recompress_compressed_files)
  - [`zip_compression_iterations`](#zip_compression_iterations)
  - [`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
  - [`work_around_minecraft_quirks`](#work_around_minecraft_quirks)
  - [`automatic_asset_types_mask_detection`](#automatic_asset_types_mask_detection)
  - [`allow_mods`](#allow_mods)
  - [`skip_pack_icon`](#skip_pack_icon)
  - [`validate_pack_metadata_file`](#validate_pack_metadata_file)
  - [`ignore_system_and_hidden_files`](#ignore_system_and_hidden_files)
  - [`zip_spec_conformance_level`](#zip_spec_conformance_level)
  - [`size_increasing_zip_obfuscation`](#size_increasing_zip_obfuscation)
  - [`percentage_of_zip_structures_tuned_for_obfuscation_discretion`](#percentage_of_zip_structures_tuned_for_obfuscation_discretion)
  - [`never_store_squash_times`](#never_store_squash_times)
  - [`threads`](#threads)
  - [`spooling_buffers_size`](#spooling_buffers_size)
  - [`zip_comment`](#zip_comment)
- [Per-file options](#per-file-options)
  - [Audio files](#audio-files)
    - [`transcode_ogg`](#transcode_ogg)
    - [`two_pass_vorbis_optimization_and_validation`](#two_pass_vorbis_optimization_and_validation)
    - [`channels`](#channels)
    - [`sampling_frequency`](#sampling_frequency)
    - [`empty_audio_optimization`](#empty_audio_optimization)
    - [`bitrate_control_mode`](#bitrate_control_mode)
    - [`target_bitrate_control_metric`](#target_bitrate_control_metric)
    - [`ogg_obfuscation`](#ogg_obfuscation)
    - [`target_pitch`](#target_pitch)
  - [JSON files](#json-files)
    - [`minify_json`](#minify_json)
    - [`delete_bloat_keys`](#delete_bloat_keys)
    - [`always_allow_json_comments`](#always_allow_json_comments)
    - [`sort_json_object_keys`](#sort_json_object_keys)
  - [PNG files](#png-files)
    - [`image_data_compression_iterations`](#image_data_compression_iterations)
    - [`color_quantization_target`](#color_quantization_target)
    - [`color_quantization_dithering_level`](#color_quantization_dithering_level)
    - [`maximum_width_and_height`](#maximum_width_and_height)
    - [`skip_alpha_optimizations`](#skip_alpha_optimizations)
    - [`downsize_if_single_color`](#downsize_if_single_color)
    - [`png_obfuscation`](#png_obfuscation)
  - [Shader files](#shader-files)
    - [`shader_source_transformation_strategy`](#shader_source_transformation_strategy)
    - [`is_top_level_shader`](#is_top_level_shader)
  - [Legacy language files](#legacy-language-files)
    - [`minify_legacy_language`](#minify_legacy_language)
    - [`strip_legacy_language_bom`](#strip_legacy_language_bom)
  - [Command function files](#command-function-files)
    - [`minify_command_function`](#minify_command_function)
  - [Properties files](#properties-files)
    - [`minify_properties`](#minify_properties)
  - [Compressed compound NBT tag files](#compressed-compound-nbt-tag-files)
    - [`nbt_compression_iterations`](#nbt_compression_iterations)
  - [Custom files](#custom-files)
    - [`force_include`](#force_include)
- [Examples](#examples)
  - [Basic options file](#basic-options-file)
  - [Basic options file but tuned for extreme compression](#basic-options-file-but-tuned-for-extreme-compression)
  - [Basic options file with extraction protection](#basic-options-file-with-extraction-protection)
  - [Complex options file](#complex-options-file)

## Introduction

PackSquash requires options to be explicitly configured to function. These
options are read from [TOML v1.0.0](https://toml.io/en/v1.0.0) documents that
contain specific [keys](https://toml.io/en/v1.0.0#keys) and
[tables](https://toml.io/en/v1.0.0#table). TOML is expected to be easy for
humans to read and easy for programs to parse and generate.

The only required option is [`pack_directory`](#pack_directory). The following
sections describe in detail all the options you can customize. If they are too
dense for you, or you aren't sure how an option works, you may find the
[examples](#examples) at the end of this document useful to get started.

## How to pass options

PackSquash can parse TOML documents containing the options documented here from
two sources:

- From a file. The TOML document with options thus becomes the so-called options
  file. The path of the options file to read is passed as a command-line
  parameter. Options files are the recommended way for end-users to store,
  exchange, and make PackSquash use a set of options.
- From its standard input stream (usually, the user keyboard or pipe). This
  source is used if PackSquash is not passed a path in its command line
  parameters or if the path is a hyphen ("-"). Reading options from the standard
  input is suitable for making PackSquash consume the output of another program
  (piping) or advanced interactive usage.

You can also get a full list of supported command-line arguments via the
`--help` switch.

## Global options

The global options are [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair) that control what pack is
processed and how. They belong outside of any table (i.e., before any table
begins). PackSquash rejects options files that contain global options not
described in this section to improve extensibility. This section describes such
available options.

### `pack_directory`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: none (**required**)

The absolute or relative path to the directory where the pack that will be
optimized resides.

If you use a literal string (i.e., surround the path with single quotes, `'like
this'`) you won't need to escape any characters, but you won't be able to write
paths that contain a single quote. On the other hand, if you use basic strings
(i.e., surround the path by double quotes, `"like this"`), you will need to
escape double quotes, backslashes, and other special characters defined by the
TOML specification, but you will be able to write paths that contain a single
quote. For more details about how strings are parsed, please read the TOML
specification.

Example:

```toml
pack_directory = 'C:\path\to\pack'
```

### `output_file_path`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `pack.zip` (file `pack.zip` in the current working directory)

The absolute or relative path to the ZIP file that contains the optimized pack
that PackSquash will generate. This path must point to a file that may already
exist or not, but never to a directory.

If the ZIP file indicated by this path already exists, PackSquash may try to
open and use it to generate a new version, depending on the values of the
[`zip_spec_conformance_level`](#zip_spec_conformance_level) and
[`never_store_squash_times`](#never_store_squash_times) options. Please read
their description for more details.

Example:

```toml
output_file_path = 'C:\path\to\result\pack\zip\file\my_pack.zip'
```

### `recompress_compressed_files`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If `true`, this option makes PackSquash try to compress files whose contents are
already compressed before adding them to the generated ZIP file, after all the
file type-specific optimizations have been applied. This can squeeze in some
extra savings at the cost of noticeably increased pack processing times.
Currently, Ogg and PNG assets are the only already compressed files affected by
this option, but this may change in the future.

Example:

```toml
recompress_compressed_files = true
```

### `zip_compression_iterations`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) in the [0, 255] interval

**Default value**: `20`

The number of Zopfli compression iterations that PackSquash will do when
compressing a file of 1 MiB magnitude just before it is added to the generated
ZIP file. This affects files whose contents are not already compressed or all
files if [`recompress_compressed_files`](#recompress_compressed_files) is
enabled.

A higher number of iterations means that bigger files will be subject to more
compression iterations, which may lead to higher space savings but is slower.
Lower numbers imply that, in general, fewer compression iterations will be done,
which is quicker but reduces space savings.

Note that PackSquash calculates the exact number of iterations for a file
depending on its size, so this number of iterations only guides that
computation. More precisely, PackSquash targets a reference compression time, so
smaller files will be compressed more, and larger files will be compressed less.
Also, the file size is converted into a non-linear magnitude that grows slower
than the file size itself (in mathematical terms, the order of the function is
that of a fractional power, which is less than linear), and this magnitude is
what is really used to compute the number of iterations. A consequence of this
design is that PackSquash will be "hesitant" to reduce the number of iterations
for bigger files to meet the target time, exceeding it, but not by too much.

Unless set to zero, the number of iterations is clamped to the [1, 20] interval,
so using values outside that interval for this option will only change the
magnitude threshold where iterations start being reduced to meet a target time.

Zero is a special case: no file will be compressed, no matter its size. This is
useful to speed up the process without sacrificing file-specific optimization
techniques. It might also speed up the loading of your pack by Minecraft clients
because they won't have to decompress any file, which is a bottleneck,
especially with the advent of fast storage devices. The obvious downside is that
the generated ZIP files will likely be larger, which increases bandwidth
requirements. However, if the decompression speed is much greater than the
storage device speed (i.e., a beefy CPU is paired with a slow HDD, for example),
Minecraft clients may take longer to load the pack.

Example:

```toml
zip_compression_iterations = 5
```

### `automatic_minecraft_quirks_detection`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

When this option is set to `true`, PackSquash will try to automatically deduce
an appropriate set of Minecraft quirks that affect how pack files can be
optimized by looking at the pack files. This automatic detection works fine in
most circumstances, but because quirks affect specific Minecraft versions, and
maybe only under some conditions, it might be inexact.

If you exactly know what quirks affect your pack and do not want PackSquash to
come up with its own set of quirks to work around, set this option to `false`,
and configure [`work_around_minecraft_quirks`](#work_around_minecraft_quirks)
accordingly. Otherwise, you can leave it set to `true`.

Please note that the automatic Minecraft quirks detection may validate and
process the contents of the `pack.mcmeta` file, even if
[`validate_pack_metadata_file`](#validate_pack_metadata_file) and
[`automatic_asset_types_mask_detection`](#automatic_asset_types_mask_detection)
are set to `false`. To prevent PackSquash from validating that file, these
options should be all set to `false`.

Example:

```toml
automatic_minecraft_quirks_detection = true
```

### `work_around_minecraft_quirks`

**Type**: [Array](https://toml.io/en/v1.0.0#array) of
[String](https://toml.io/en/v1.0.0#string)

**Default value**: `[]` (empty array; no quirks worked around, unless
[`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
is set to `true` and quirks were detected)

Some Minecraft versions have some quirks that limit how some files can be
compressed before they stop being interpreted correctly by the game. PackSquash
can work around these quirks, but doing so may come at the cost of reduced space
savings, increased processing times, or other undesirable consequences, so such
workarounds should only be done if a pack is affected by them. This option
allows to manually specify a comma-separated list of quirks that will be worked
around if
[`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
is set to `false`. The following quirks are supported:

- `grayscale_images_gamma_miscorrection`: older versions of Minecraft (probably
  all versions since 1.6 until 1.13 are affected) assume that grayscale images
  are in a fairly uncommon color space instead of the more common sRGB space
  presumed for the rest of color types. Because PackSquash can compress
  full-color images that only have gray pixels to actual grayscale format to
  save space, affected Minecraft versions display those images with colors that
  look "washed out". The workaround implemented for this quirk stops PackSquash
  from trying to reduce color images to grayscale format under any
  circumstances, which may hurt compression.
- `restrictive_banner_layer_texture_format_check`: older versions of Minecraft
  (probably all versions from 1.6 until 1.13 are affected) require banner and
  shield layer textures to be stored in RGBA format, or else the layers they
  represent won't be applied at all, even if the palette contains transparency
  data. PackSquash can convert images encoded in RGBA format to palette format
  to save space, triggering this quirky behavior in affected versions. This
  workaround stops PackSquash from changing the color format of the affected
  textures to a palette, which includes color quantization, as it is used to
  generate a palette. This incurs some space costs.
- `bad_entity_eye_layer_texture_transparency_blending`: Minecraft versions older
  than 24w40a (1.21.2) overlay entity layer textures in a way that does not
  rightly account for transparency, taking into account their color and not only
  their transparency values as blending coefficients to use for overlying that
  texture (see [MC-235953](https://bugs.mojang.com/browse/MC-235953)).
  PackSquash can change the color of transparent pixels, triggering this
  behavior. This workaround stops PackSquash from changing the color of
  transparent pixels and quantizing the pixels to a palette to reduce texture
  file size, as both optimizations do not guarantee that the color of
  transparent pixels will stay the same.
- `java8_zip_parsing`: the latest Minecraft versions, from 1.17 onwards, are
  compiled for Java 16+, which means that they do not support older Java
  versions. On the other hand, Java 8 was used almost ubiquitously with older
  Minecraft clients, especially in modded environments. However, a lot of things
  have changed in newer Java versions, including low-level details of how ZIP
  files are parsed. When a ZIP specification conformance level that adds
  extraction protection is used, this workaround tells PackSquash to use
  obfuscation techniques that work fine with Java 8. This comes at the cost of
  protection that is a bit different, but the small differences will extremely
  likely not matter in protection strength. Compressibility can be impacted
  negatively, though. This quirk does not have any effect if an affected ZIP
  specification conformance level is not chosen or if the Minecraft client is
  run using newer Java versions.
- `ogg_obfuscation_incompatibility`: not all Minecraft versions are compatible
  with the techniques PackSquash uses to obfuscate Ogg Vorbis files. Currently,
  only versions from 1.14 to 24w14a (1.20.5) reliably support obfuscated files;
  other versions may display console errors or even freeze when attempting to
  play obfuscated sounds. This workaround disables obfuscation for any Ogg
  Vorbis files generated by PackSquash, allowing the pack to work across all
  Minecraft versions, at the cost of no obfuscation. Note that, since multiple
  Minecraft versions share the same pack format, the autodetection code for this
  quirk will err on the safe side and consider slightly more Minecraft versions
  to be affected than necessary.
- `png_obfuscation_incompatibility`: not all Minecraft versions are compatible
  with the techniques PackSquash uses to obfuscate PNG files. Currently, only
  versions from 1.14 onwards support obfuscated textures; other versions may
  display errors when the affected textures are loaded. This workaround disables
  obfuscation for any PNG files generated by PackSquash, allowing the pack to
  work across all Minecraft versions, at the cost of no obfuscation.

When
[`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
is set to `true`, PackSquash will use an automatically detected set of quirks no
matter what, ignoring the value of this option.

Example:

```toml
work_around_minecraft_quirks = ['grayscale_images_gamma_miscorrection', 'restrictive_banner_layer_texture_format_check', 'bad_entity_eye_layer_texture_transparency_blending', 'java8_zip_parsing', 'ogg_obfuscation_incompatibility', 'png_obfuscation_incompatibility']
```

### `automatic_asset_types_mask_detection`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

By default, PackSquash will attempt to automatically deduce the appropriate set
of pack files to include in the generated ZIP by checking what Minecraft
versions it targets, according to the pack format version. This works fine in
most circumstances and saves space if the pack contains legacy or too new files
for the targeted Minecraft version, but it might be undesirable sometimes.

If you want PackSquash to include every pack file it recognizes and is enabled
in [`allow_mods`](#allow_mods) no matter what, set this option to `false`.
Otherwise, leave it set to `true` to let it exclude files that are known to be
not relevant.

When this option is set to `true`, the `pack.mcmeta` file may be read and
validated, even if [`validate_pack_metadata_file`](#validate_pack_metadata_file)
and
[`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
are set to `false`. To guarantee that file is not read no matter what, these
options should be all set to `false`.

Example:

```toml
automatic_asset_types_mask_detection = true
```

### `allow_mods`

**Type**: [Array](https://toml.io/en/v1.0.0#array) of
[String](https://toml.io/en/v1.0.0#string)

**Default value**: `[]` (empty array; skip all mod-specific files)

PackSquash supports pack files that are only consumed by certain Minecraft mods,
but, in the interest of optimizing packs as much as possible, it assumes that
mod-specific files will not be used by the game by default and discards (skips)
them from the generated ZIP file. This will break your pack unless you tell
PackSquash about the involved mods via this option, whose value is a
comma-separated list of mod identifiers. The following mods are supported:

- `OptiFine`: adds support for Java properties files used by several of its
  features (`.properties`) and Custom Entity Model files (`.jem`, `.jemc`,
  `.jpm`, and `.jpmc`). It also accepts and optimizes vanilla models in the
  custom item feature files directory.
- `Minecraft Transit Railway 3`: adds support for Blockbench modded entity model
  projects for custom train models (`.bbmodel` and `.bbmodelc`) in the `mtr`
  asset namespace.

Example:

```toml
allow_mods = ['OptiFine', 'Minecraft Transit Railway 3']
```

<sub><sup>This option is only supported if PackSquash was compiled with support
for mods, which always is the case if you use a PackSquash build downloaded from
this repository.</sup></sub>

### `skip_pack_icon`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

Under some circumstances, the `pack.png` pack icon won't be shown in the
Minecraft UI, even if it is present. Therefore, skipping it will save space
without any side effects if the pack is to be used only under these
circumstances. If this option is set to `true`, the `pack.png` file that
contains the pack icon will not be added to the generated ZIP file.

In most older Minecraft versions, which include 1.16.3 and 1.17.1, pack icons
are not shown for server resource packs.

Example:

```toml
skip_pack_icon = true
```

### `validate_pack_metadata_file`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

This option controls whether the pack metadata file, `pack.mcmeta`, will be
validated (`true`) or not (`false`). Validating this file is a good thing in
most circumstances, and you should only disable this option if you are extremely
concerned about performance, need to add a `pack.mcmeta` file to the generated
ZIP file later, want to use PackSquash with folders that are not a Minecraft
pack, or similar reasons.

Even if this option is set to `false`, the `pack.mcmeta` may still be validated
if
[`automatic_minecraft_quirks_detection`](#automatic_minecraft_quirks_detection)
or
[`automatic_asset_types_mask_detection`](#automatic_asset_types_mask_detection)
are enabled. To guarantee that the pack metadata file is not validated no matter
what, both options should be set to `false`.

Example:

```toml
validate_pack_metadata_file = true
```

### `ignore_system_and_hidden_files`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

This option controls whether PackSquash will ignore system (i.e., whose name
signals they were generated by a ubiquitous and well-known program that is not
related to a Minecraft pack) and hidden files (i.e., whose name starts with a
dot), not printing status messages about them, nor even trying to process them.
If `true`, these files will be ignored. If `false`, these files will not be
treated specially and will be processed as normal. Ignoring these files is
usually a good thing to do unless your pack really contains files that are
filtered out by this option.

Example:

```toml
ignore_system_and_hidden_files = false
```

### `zip_spec_conformance_level`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `pedantic`

PackSquash uses a custom ZIP compressor that is able to balance ZIP file
specification conformance and interoperability with increased performance, space
savings, compressibility, and protection against external programs being able to
extract files from it. This option lets you choose the ZIP specification
conformance level that is most suitable to your pack and situation. The
following levels are available:

- `pedantic`: the generated ZIP files will follow the ZIP file specification to
  the letter, with the objective that they can be used with virtually any ZIP
  file manipulation program. This is a safe and user-friendly default, but it
  has a big downside: PackSquash can't do anything that may render the ZIP files
  it generates unconventional. This means that identical files after file
  type-specific optimizations will not be stored only once (i.e., deduplicated),
  the generated ZIP files will not contain the metadata needed to reuse them in
  future runs to speed up execution, files will be able to be extracted from
  them as normal, and compressibility of the ZIP file internal structures will
  not be improved. Anyway, you still get file type-specific optimizations,
  Zopfli compression, and metadata removal in both the generated ZIP and the
  pack file, which usually have a big impact on pack sizes.
- `high`: similar to `pedantic`, but it allows storing the metadata needed to
  reuse the generated ZIP files in future PackSquash runs. This metadata is
  stored in a way that is compatible with the vast majority of ZIP file
  manipulation programs, although it technically does not conform to the ZIP
  file specification, so, while it is technically possible to find a program
  that rejects the file, in practice that is highly unlikely.
- `balanced`: like `high`, but enables deduplication of identical files in the
  generated ZIP file. This yields significant space savings if somewhat big
  files are repeated in your pack, like textures or sounds, although it also
  helps with smaller files. Some ZIP file manipulation programs will still
  properly work when files are deduplicated, while others will not, so while
  this has a significant impact on interoperability, how that matters to you
  depends on what programs you expect to use with the ZIP file.
- `disregard`: PackSquash will use every trick up its sleeve to give you every
  feature it offers, including extraction protection and improved internal ZIP
  file structures compressibility, without any consideration for
  interoperability whatsoever. The only constraint is that the pack works as
  usual within Minecraft.

The following table summarizes and compares what each available conformance
level offers.

|                                                                          |   `pedantic`  |       `high`                  |      `balanced`     | `disregard` |
| :----------------------------------------------------------------------: | :-----------: | :---------------------------: | :-----------------: | :---------: |
| File type-specific optimizations                                         |       ✔       |               ✔               |          ✔          |      ✔      |
| Zopfli compression                                                       |       ✔       |               ✔               |          ✔          |      ✔      |
| Metadata removal<sup>*1</sup>                                            |       ✔       |               ✔               |          ✔          |      ✔      |
| Identical file deduplication                                             |       ✘       |               ✘               |          ✔          |      ✔      |
| Extraction protection<sup>*2</sup>                                       |       ✘       |               ✘               |          ✘          |      ✔      |
| Improved internal ZIP file structures compressibility<sup>*3</sup>       |       ✘       |               ✘               |          ✘          |      ✔      |
| Programs that can safely manipulate the output ZIP file                  |      Any      | The vast majority of programs |   Select programs   | As few as possible |
| Potential distribution and storage issues<sup>*4</sup>                   |      None     |              None             |         Some        |  Some more  |
| Levels whose output ZIP files can be reused with this level<sup>*5</sup> |       —       | `high`, `balanced`  | `high`, `balanced`  | `disregard` |
| Appropriate for | Using PackSquash for the first time, pack archival and backup, permissively licensed packs, trusting the users of your pack, fostering collaboration and innovation | Similar use cases as `pedantic`, but you want to take advantage of reusing previous ZIP files | Similar use cases as `high`, but you want file deduplication too | Proprietary packs whose usage you want to limit, especially for public server resource packs; just getting the best optimization possible |

<sup>*1</sup> Metadata removal includes non-critical PNG headers, Vorbis
comments and ID3 tags, and filesystem metadata such as creation time,
modification time, permissions, and proprietary user. This improves privacy and
reduces file sizes.

<sup>*2</sup>
> [!IMPORTANT]
> Due to Minecraft limitations, extraction protection is done using techniques
> that are not considered secure by modern information security standards (more
> specifically, they do not hold the [Kerckhoffs's
> principle](https://en.wikipedia.org/wiki/Kerckhoffs%27s_principle)). In order
> words, it is not reasonable to expect strong security guarantees from this
> protection.

<sup>*3</sup> Compressibility improvements do not reduce the actual size of the
generated ZIP files. However, they may allow for higher savings if the generated
ZIP files are compressed again. This is useful when serving packs over an HTTP
server with static compression enabled because it reduces bandwidth
requirements, transparently compressing and decompressing the pack while it is
being downloaded.

<sup>*4</sup>
> [!WARNING]
> *While using progressively higher levels of ZIP specification non-conformance
> can be effective for optimizing a pack's size and protection, the possibly
> desirable fact that such generated files are not as easily readable by other
> programs can backfire in several ways*.
>
> Some ways that happened to users are outlined below:
>
> - Some hosting services attempt to read uploaded ZIP files for validation, and
>   if they cannot do so because the ZIP file is unreadable, the pack may be
>   rejected. For instance, [mc-packs.net](http://mc-packs.net) is a
>   known-affected hosting service, whereas Dropbox, AWS S3, Azure Blob Storage,
>   Cloudflare R2, and most other generic web and file hosting services are
>   unaffected.
> - Security software that scans ZIP files may flag such packs as suspicious
>   because the protection techniques used by PackSquash can also be exploited
>   by malicious actors in other programs to bypass security controls. Depending
>   on the security software configuration, the pack file may be deleted or made
>   unreadable, causing issues with pack transfer and/or loading. While this
>   typically isn't a problem for most users, who generally connect to the
>   Internet through residential ISP gateways and at most run Windows Defender,
>   some users may have stricter antivirus software, be connected to networks
>   with enhanced security measures (such as those in academic or corporate
>   environments, where DPI firewalls, proxies, and IDS/IPS systems are common),
>   or use services that perform these checks (e.g., attach the pack to an email
>   whose mail servers scan attachments with an affected antivirus solution).
>   See also [issue
>   #317](https://github.com/ComunidadAylas/PackSquash/issues/317) for more on
>   this.
> - Different Minecraft clients handle ZIP specification non-conformances
>   differently, meaning a pack that works fine on one client may be rejected by
>   another that has different mods, is configured differently, or is of a
>   different game version. While PackSquash usually hides such differences
>   effectively, and no known Minecraft mods alter the game decompression
>   routines, there's no guarantee that this will remain the case in the future.
> - If the original pack files are lost and no backups are available, recovering
>   the optimized files may be more challenging due to the difficulty in
>   extracting them reliably.
>
> In light of these potential drawbacks, we recommend thoroughly testing and
> analyzing how lower ZIP specification conformance levels might affect you and
> your users before deploying packs to production or a wider audience. It's also
> important to ensure you have the capacity to troubleshoot and address the
> consequences of the decision you make. For what it's worth, the authors of
> PackSquash have not found evidence of these negative effects causing
> widespread problems on several established servers.

<sup>*5</sup>
> [!IMPORTANT]
> In general, *you should be careful when you try to reuse a generated ZIP file
> to speed up the optimization of a pack if you do any modification to the
> options file, update PackSquash, move that file between devices, modify it
> outside of PackSquash, or the set of Minecraft quirks to work around changes*.
>
> Failure to follow this advice may lead to the generation of incorrect ZIP
> files in ways that may not be immediately obvious. Just changing the
> conformance level to another that is compatible with the level used in the
> previous run is fine, however. It is also okay to reuse a generated ZIP file
> as many times as desired if you don't change anything.
>
> These are the catches you should keep in mind:
> - First of all, you may set the
>   [`never_store_squash_times`](#never_store_squash_times) option to a value
>   that does not save the metadata needed to reuse ZIP files, independently of
>   the conformance level. You should not reuse ZIP files that were generated
>   with this option enabled (i.e., set to `true`) after you disable it (i.e.,
>   set it to `false`). Doing the opposite thing (i.e., from `false` to `true`)
>   is fine, but that will end up not reusing the ZIP file.
> - Any change to an option that affects how files are compressed or optimized
>   will not be applied for non-modified files because they will not be
>   processed again.
> - If the set of [Minecraft quirks](#work_around_minecraft_quirks) to work
>   around changes, either because PackSquash detects that the pack was upgraded
>   or downgraded to work with another Minecraft version or you've explicitly
>   set them to a different value, the change will not be applied for
>   non-modified files.
> - PackSquash quickly detects whether a file was modified or not by looking at
>   the modification timestamp provided by the filesystem and comparing it with
>   an encrypted timestamp stored in the ZIP file. The encryption key used is
>   device-specific (see the documentation on [system
>   identifiers](https://github.com/ComunidadAylas/PackSquash/wiki/Generated-ZIP-file-reuse-feature-design#system-identifiers)
>   for more information). If modification timestamps are not available or
>   reliable, this detection may not work as expected (this usually is not the
>   case unless you copy files between partitions or devices, though).
> - You should not modify otherwise reusable generated ZIP files outside of
>   PackSquash. Doing so may change the file structure or timestamp metadata in
>   ways that PackSquash doesn't expect. You can copy the generated file around
>   and read or extract files from it.
> - Some effort is made to make ZIP files generated in the current version of
>   PackSquash compatible with future versions, but this compatibility is by no
>   means guaranteed. It is best to start from scratch after updating PackSquash
>   unless you validate that the versions are compatible.
> - Reusing ZIP files that were generated with
>   [`size_increasing_zip_obfuscation`](#size_increasing_zip_obfuscation) set to
>   `false` after it is changed to `true`, and vice versa, is not safe. Trying
>   to do so will, in the best-case scenario, end up not reusing the ZIP file at
>   all, and in the worst-case scenario, corrupting data.

With these gotchas out of the way, to reuse a ZIP file that was previously
generated by PackSquash, it suffices to set
[`output_file_path`](#output_file_path) to the path of that file. The previous
version of the file will be overwritten after the pack is processed.

Example:

```toml
zip_spec_conformance_level = 'high'
```

### `size_increasing_zip_obfuscation`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If [`zip_spec_conformance_level`](#zip_spec_conformance_level) is set to
`disregard`, enabling this option will add more protections against inspecting,
extracting, or tampering with the generated ZIP file that will slightly increase
its size. This option does not affect whether protections that do not increase
the file size are added or not and does not have any effect if the conformance
level does not feature protection.

Example:

```toml
size_increasing_zip_obfuscation = true
```

### `percentage_of_zip_structures_tuned_for_obfuscation_discretion`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) in the [0, 100] interval

**Default value**: `0`

If [`zip_spec_conformance_level`](#zip_spec_conformance_level) is set to
`disregard`, this option sets the approximate probability for each internal
generated ZIP file structure to be stored in a way that favors additional
discretion of the fact that protection techniques were used, as opposed to a way
that favors increased compressibility of the result ZIP file. This option is
ignored for other conformance levels.

When this option is set to 0 (minimum value), every ZIP record will be stored
favoring increased compressibility. Conversely, when it is set to 100 (maximum
value), every ZIP record will be stored favoring increased discretion. Other
values combine increased discretion and compressibility.

Example:

```toml
percentage_of_zip_structures_tuned_for_obfuscation_discretion = 100
```

### `never_store_squash_times`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

This option controls whether PackSquash will refuse to store the metadata needed
to reuse previously generated ZIP files, and likewise not expect such data if
the output ZIP file already exists, thus not reusing its contents to speed up
the process in any way, no matter what the
[`zip_spec_conformance_level`](#zip_spec_conformance_level) is.

You might want to set this to `true` if you are concerned about the presence of
encrypted metadata in the generated ZIP files, do not care about potential
speedups in later runs, file modification timestamps are unreliable for some
reason, or do not want PackSquash to get and use a system ID in any way. In
fact, if PackSquash will not be run anymore on this pack, it is a good idea to
set this to `true`, as this improves compressibility a bit and removes the now
unnecessary metadata.

Example:

```toml
never_store_squash_times = true
```

### `threads`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) greater than zero

**Default value**: number of available physical CPU threads

The maximum number of concurrent threads that PackSquash will use to process the
pack files. Higher numbers can spawn more threads, so if your computer has
enough physical CPU threads, several files can be processed at once, improving
the speed substantially. However, you might want to use a lower number of
threads due to memory, power consumption, open file limitations or CPU time
limitation concerns. This number is tentative, meaning that PackSquash may spawn
extra threads for internal purposes.

Example:

```toml
threads = 4
```

### `spooling_buffers_size`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) greater than or equal to
zero

**Default value**: half of the available main memory reported by the operating
system / (number of available physical CPU threads + 1)

The maximum size of the in-memory buffers that temporarily hold data to be
written to the generated ZIP file, in MiB. Ideally, if the buffers are big
enough to hold the entire ZIP file and any additional scratch data, PackSquash
will work almost entirely in memory and not do any disk operation, which is
pretty fast. However, if some buffer grows bigger than this size threshold, it
has to be rolled over to disk, which usually is much slower to operate with than
main memory, because otherwise PackSquash could run out of available memory and
be forced to abort its execution, which is a bad thing. The default value is
meant to be an educated guess of the optimum value, taking into account the
installed physical memory (RAM), the size of the pagination file or swap, the
amount of memory currently used by other applications, that other applications
may be launched or increase their memory demands while PackSquash executes, and
the fact that PackSquash uses a buffer for each thread that processes packs +
one for the generated ZIP file.

If you run into out-of-memory errors while executing PackSquash, try
decrementing this value to be able to use it without such problems. Conversely,
if you observe that PackSquash disk usage suddenly rises notably and there is
enough available memory to spare, try incrementing this value for maximum
performance.

Example:

```toml
spooling_buffers_size = 128
```

### `zip_comment`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `''` (empty string; no comment)

The comment string that will be attached to the output ZIP file, which is
displayed by some ZIP file manipulation programs when examining the archive.
This string is limited to 65535 US-ASCII characters in size, must not contain
some special character sequences that are internally used by the ZIP format to
delimit its structures, and is guaranteed to be placed at the end of the output
ZIP file.

While it is also possible to attach text notes to a ZIP file by adding a file
with a well-known name to it, and doing so is required for non-text or complex
data that takes more than 65535 characters, comment strings are usually
displayed more prominently in user interfaces and more convenient for programs
to read, rendering them more suitable for purposes like storing important
user-facing notices and file tracking metadata.

Example:

```toml
zip_comment = 'Created with PackSquash'
```

## Per-file options

PackSquash supports customizing how several pack file types are compressed, on a
per-file basis, via [tables](https://toml.io/en/v1.0.0#table). Tables represent
a group of options that are applied to the files whose relative path matches a
[extended glob pattern
syntax](https://docs.rs/globset/0.4.8/globset/index.html#syntax) contained in
the table name.

For matching, the path component separator is normalized to a forward slash (/),
so the configuration files are operating system agnostic. Also, the `*` and `?`
metacharacters can never match a path separator. The backslash character may be
used to escape special characters.

For example, you can match any files inside a "music" or "ambience" folder that
have a non-empty name and an audio file extension with the following pattern:

```glob
**/{music,ambience}/?*.{og[ga],mp3,wav,flac}
```

Another example that matches the same files as before, but only when they are in
the "music" or "ambience" folders of a resource pack assets folder is:

```glob
assets/*/sounds/{music,ambience}/?*.{og[ga],mp3,wav,flac}
```

Keep in mind that if your pattern contains a dot or characters that are not
ASCII letters, ASCII digits, underscores, and dashes (A-Za-z0-9_-), you will
need to put them in a string (i.e., between single quotes, like `'this'`) when
writing the table name in the options file.

Of course, different file types require different options. PackSquash will
detect on the fly the file type the configuration you write is intended for. If
several patterns match a single file, PackSquash will use the first one that
customizes options appropriate for the file type, and if no pattern is
appropriate or no pattern matches, use default options. There is a list of
options you can change per file type below.

### Audio files

You can customize how PackSquash transcodes the audio files of a pack via the
following [key and value pairs](https://toml.io/en/v1.0.0#keyvalue-pair).

#### `transcode_ogg`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

When `true`, Ogg files will be reencoded again to apply resampling, channel
mixing, pitch shifting, and bitrate reduction, which may degrade their quality,
but commonly saves quite a bit of space. If you change it to `false`, Ogg files
will be added to the generated ZIP file without being reencoded or modified.
Non-Ogg audio files will be reencoded no matter the value of this option.

Example:

```toml
transcode_ogg = false
```

#### `two_pass_vorbis_optimization_and_validation`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

When `true`, an additional fast two-pass optimization and validation step will
be performed on the generated Ogg Vorbis file before it is added to the pack,
regardless of whether it has been transcoded. This enables PackSquash to ensure
that the audio file will work fine in Minecraft, losslessly reduce its size by
an average of 5%, and optionally obfuscate it to thwart its playback outside of
Minecraft (see also [`ogg_obfuscation`](#ogg_obfuscation)).

Due to how fast and unobtrusive this step is, it's usually best to leave it
enabled. Good reasons to disable it include troubleshooting and wanting a
slightly faster execution at the cost of missing out on the features described
above.

Example:

```toml
two_pass_vorbis_optimization_and_validation = true
```

#### `channels`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) greater than zero

**Default value**: number of channels of the input audio data

Specifies the number of audio channels that the processed audio file will have
in the generated ZIP file. Values different to 1 (mono) or 2 (stereo) make
little sense to use with current versions of Minecraft and are not allowed. As
per [MC-146721](https://bugs.mojang.com/browse/MC-146721), Minecraft computes
positional sound effects depending on whether sounds are mono or stereo, so even
though mono sounds are more space-efficient (because they contain half the
samples), downmixing stereo sounds to mono or upmixing mono sounds to stereo has
side effects.

It should be noted that, although mono files contain half the audio data than
stereo ones, this does not necessarily translate to half the space savings. The
Vorbis codec used in Ogg files employs [joint
encoding](https://en.wikipedia.org/wiki/Joint_encoding), which is pretty space-efficient for common sounds.

Example:

```toml
channels = 2
```

#### `sampling_frequency`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) greater than zero

**Default value**: `40050` (40.05 kHz) for stereo audio, `32000` (32 kHz) for
mono audio

Specifies the sampling frequency (i.e., number of samples per second) to which
the input audio file will be resampled (in Hertz, Hz). If this frequency is
higher than the sampling frequency of the input audio file, the resampling will
be skipped to avoid wasting space with redundant samples.

As per the [Nyquist-Shannon
theorem](https://en.wikipedia.org/wiki/Nyquist%E2%80%93Shannon_sampling_theorem),
for a given sampling frequency of 𝑥 Hz, only frequencies up to 𝑥 ÷ 2 Hz can be
recreated without aliasing artifacts, in general. Human speech typically employs
frequencies up to 6 kHz, so a sampling frequency of 12 kHz saves space while
still providing acceptable audio quality. However, other sounds (e.g., music)
have a broader frequency spectrum, up to 20 kHz (the generally accepted upper
limit of the human hearing range). Therefore, in any case, a frequency greater
than 40 kHz is wasteful for encoding audio that will be heard by humans and is
not meant to be edited any further. The default value is meant to sound faithful
to vanilla sounds that have a wide frequency spectrum while still providing
significant savings.

Example:

```toml
sampling_frequency = 44100
```

#### `empty_audio_optimization`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, empty audio files (i.e., with no audio data, or full of complete
silence) will be replaced with a special empty audio file that is optimized for
size and contains no audio data. This kind of file works fine in Minecraft and
most media players, but some may consider the lack of audio data an error.

This option is only honored if the audio file is being transcoded, which is
always the case when the `transcode_ogg` option is set to `true`.

Example:

```toml
empty_audio_optimization = false
```

#### `bitrate_control_mode`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `CQF`

The bitrate control mode used during transcoding. Different bitrate control
modes have different tradeoffs between audio quality, file size, bandwidth
predictability, and encoding speed. They also affect how the
[`target_bitrate_control_metric`](#target_bitrate_control_metric) is
interpreted, if specified. The available bitrate control modes are:

- `CQF` (Constant Quality Factor): the encoder will interpret the target metric
  as a quality factor and will try to keep the perceived subjective quality
  constant at all times. The encoder will have no hard pressures to limit the
  bitrate in any way, although the quality metric tends to be strongly
  correlated with an average bitrate for typical signals.
  - When in doubt, use this mode, as it provides an effective balance for most
    situations.
  - The quality factor (i.e., the target metric) is expected to be in the range
    [-2, 10], where -2 is the worst audio quality, and 10 is the best.
  - Some advantages of this bitrate control mode include:
    - It adapts well to different sampling frequencies and channel counts: the
      encoder knows that it needs fewer bits to encode mono signals than stereo
      signals of the same quality level, for example.
    - Unlike with bitrates, it's not possible to ask for unsupported quality
      levels.
    - Easy-to-encode audio segments are stored in minimal space, with consistent
      quality: there is no pressure to meet an average bitrate.
    - Performance is significantly higher than when using ABR or CABR modes,
      because the encoder bitrate management engine is not involved.
  - Some disadvantages of this bitrate control mode include:
    - The relationship between the quality factor and the actual average bitrate
      is difficult to predict accurately.
    - There are no guarantees against difficult to encode segments significantly
      bumping the average bitrate.
- `VBR` (Variable BitRate): the encoder will interpret the target metric as an
  approximate bitrate in kbit/s, internally translating it to a quality factor.
  Therefore, this mode is equivalent to CQF, but with the quality factor
  selected in a different way.
  - Some advantages of this bitrate control mode over CQF include:
    - The relationship between quality factor and actual average bitrate is
      easier to predict.
  - Some disadvantages of this bitrate control mode over CQF include:
    - The same bitrate may yield different quality levels for different audio
      signals, or be too high or too low for the quality factors that apply to
      the signal.
- `ABR` (Average BitRate): the encoder will interpret the target metric as an
  average bitrate in kbit/s, and will be coerced to maintain that bitrate for
  the entire audio signal by using a bitrate management engine. No specific
  subjective quality level will be targeted.
  - Some advantages of this bitrate control mode over CQF and VBR include:
    - The actual average stream bitrate is guaranteed to be very close to the
      specified average bitrate. Therefore, the resulting file sizes are more
      predictable.
    - The maximum instantaneous bitrate for an audio segment can be higher than
      the average for a small time window, as long as it doesn't affect the
      long-term average.
  - Some disadvantages of this bitrate control mode over CQF and VBR include:
    - Setting too low bitrates for the input signal may severely degrade audio
      quality, while setting too high bitrates may waste space on padding the
      data to maintain the average.
    - Easy-to-encode audio segments may be stored with more bits than necessary
      for a given quality level in order to maintain the average bitrate.
      Conversely, harder-to-encode segments may sound worse when the encoder is
      already outputting at a high bitrate, as it will be deprived of bits to
      devote to them. The resulting subjective quality will be more
      inconsistent.
    - Performance is significantly worse than when using CQF or VBR due to the
      bitrate management engine being engaged.
- `CABR` (Constrained Average BitRate): the encoder will interpret the target
  metric as a hard maximum bitrate and internally select a slightly lower
  average bitrate than the maximum to maintain. This mode is similar to ABR, but
  with the addition of a maximum bitrate.
  - Some advantages of this bitrate control mode over ABR include:
    - The actual average bitrate is guaranteed to never exceed the specified
      maximum bitrate, which limits the maximum file size with certainty.
  - Some disadvantages of this bitrate control mode over ABR include:
    - To ensure that the hard maximum bitrate is never exceeded, a lower average
      bitrate will be targeted, which provides headroom for hard-to-encode
      segments, but usually results in inferior quality.

Because the default value of `target_bitrate_control_metric` is a quality
factor, specifying it when selecting a bitrate control mode other than `CQF` is
required.

Example:

```toml
bitrate_control_mode = 'VBR'
```

#### `target_bitrate_control_metric`

**Type**: [Float](https://toml.io/en/v1.0.0#float)

**Default value**: `0.25` (quality factor, ≈68 kbit/s at 44.1 kHz) for stereo
audio, `0.0` (quality factor) for mono audio

The metric to use as a target for the specified bitrate control mode when
transcoding. Depending on the [selected bitrate control
mode](#bitrate_control_mode), this will be interpreted as a quality factor,
average bitrate, approximate bitrate, or maximum bitrate.

Example:

```toml
target_bitrate_control_metric = 48 # To be interpreted as a bitrate in kbit/s
```

#### `ogg_obfuscation`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If `true`, the generated Ogg Vorbis files will be mangled so that they will be
harder to play outside of Minecraft. The obfuscation technique used by
PackSquash is not robust against some scenarios or expert knowledge, but it does
not increase file size.

This option is only taken into account if
[`two_pass_vorbis_optimization_and_validation`](#two_pass_vorbis_optimization_and_validation)
is set to `true` and the
[`ogg_obfuscation_incompatibility`](#work_around_minecraft_quirks) quirk is not
being worked around.

Example:

```toml
ogg_obfuscation = true
```

#### `target_pitch`

**Type**: [Float](https://toml.io/en/v1.0.0#float)

**Default value**: `1.0`

Sets the in-game pitch shift coefficient that will result in the audio being
played back at its original speed, affecting the perceived pitch and tempo. This
coefficient is specified as an argument to the
[`playsound`](https://minecraft.gamepedia.com/Commands/playsound#Arguments)
command and other places where Minecraft accepts a sound pitch number. For
values smaller than 1, the audio stored in the generated ZIP file will be sped
up, so it has less duration, and the generated file is smaller. Conversely, for
values larger than 1, the audio file will be slowed down, be longer, and thus
bigger. A value that exactly is one does not modify the audio.

Changing the sampling frequency is a better way to save space than using this
option. The effect on audio quality and file size that this option has is
similar to lowering or increasing the sampling rate by the same factor. However,
because this option enforces the sound playback to be pitch-shifted for it to
sound right, this coefficient can be used as a shared secret, slowing down
ripping attempts. Nevertheless, the offered protection is weak: there is a
theoretical maximum of 2<sup>30</sup> keys (i.e., 32-bit IEEE 754 floating-point
numbers in (0, 2], the interval accepted by Minecraft), which by itself is not
difficult to brute force, and further practical considerations reduce the
keyspace significantly.

Example:

```toml
target_pitch = 1.5
```

### JSON files

You can customize how PackSquash optimizes the `.json`, `.jsonc` (JSON with
comments), `.mcmeta` and `.mcmetac` files of a pack with the following [key and
value pairs](https://toml.io/en/v1.0.0#keyvalue-pair). If
[`allow_mods`](#allow_mods) includes `OptiFine`, these options also apply to
`.jem`, `.jemc`, `.jpm`, and `.jpmc` files. If [`allow_mods`](#allow_mods)
includes `Minecraft Transit Railway 3`, these options also apply to custom model
files with `.bbmodel` and `.bbmodelc` extensions.

#### `minify_json`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

When `true`, the JSON data will be minified, removing comments and unnecessary
whitespace to improve space savings. If this is changed to `false`, the JSON
will be prettified instead, reindenting it and adding spaces that make the
content easier to read by humans, which takes more space but ensures consistent
formatting. In any case, PackSquash will validate the JSON file.

Example:

```toml
minify_json = false
```

#### `delete_bloat_keys`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If this option is set to `true`, PackSquash will delete known-superfluous keys
from JSON files, like credits or metadata added by pack authoring tools, that
are completely ignored by Minecraft. This improves privacy and space savings. On
the contrary, when set to `false`, those keys will be left as-is in the JSON
file.

Example:

```toml
delete_bloat_keys = false
```

#### `always_allow_json_comments`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, PackSquash will allow comments in JSON files whose usual extension
does not end with an extra `c` letter, which explicitly marks the file as having
an extended JSON format that may contain comments. If `false`, comments will
only be allowed in JSON files with those specific extensions: `.jsonc`,
`.mcmetac`, etc.

Example:

```toml
always_allow_json_comments = false
```

#### `sort_json_object_keys`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, PackSquash will recursively sort JSON object keys by their
lexicographic order. This improves consistency by ensuring equivalent key sets
are always enumerated in the same order, and thus also compressibility. If
`false`, the original key order is preserved, which may be preferable when
working with mods that improperly rely on key ordering, or in cases where key
sorting turns out to be a wrong heuristic for compressibility.

Note that PackSquash may still not sort the keys of JSON objects no matter the
value of this option if it detects that doing so would be inappropriate, due to
excessive resource usage or other reasons.

Example:

```toml
sort_json_object_keys = false
```

### PNG files

You can customize how PackSquash optimizes the PNG files of a pack with the
following [key and value pairs](https://toml.io/en/v1.0.0#keyvalue-pair).

#### `image_data_compression_iterations`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) in the [0, 255] interval

**Default value**: `5`

Sets the number of Zopfli compression iterations that PackSquash will do to
compress raw pixel data that amounts to a magnitude of 1 MiB. This option is
similar to [`zip_compression_iterations`](#zip_compression_iterations) and is
used to feed the same linear model but with parameters better suited for image
compression.

When the number of compression iterations drops to zero, which happens when this
option is set to zero or the texture is pretty big, a much faster DEFLATE
compression algorithm is used instead of Zopfli. This extra performance may come
at the cost of file size. On the other side, the number of iterations is limited
to a maximum of 15. Values greater than 15 are still useful for this setting
because they change the threshold where iterations start being reduced in order
to keep acceptable performance levels.

Example:

```toml
image_data_compression_iterations = 15
```

#### `color_quantization_target`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `auto`

Sets the color quantization target for an image, which affects whether the lossy
color quantization process is performed and how. Quantization is useful because
it replaces the colors in the image with those in a palette that can fit in a
certain bit depth, which helps save space considerably. Higher bit depths can
represent more color variety but take more space. Lower bit depths are more
space-efficient but may reduce color fidelity because fewer different colors can
be represented. The supported color quantization targets are as follows:

- `none`: no color quantization will be done at all. This means that PackSquash
  will only apply lossless optimizations to the image, not changing any visible
  colors in any way. Color quantization must be disabled on textures that are
  used as input data for custom shaders.
- `auto`: like `eight_bit_depth`, but the image will not be color-quantized if
  PackSquash determines that not doing so is more space-efficient.
- `eight_bit_depth`: the image will be reduced to at most 256 colors, which fit
  in 8 bits. This can only effectively be a lossy process if the input texture
  is more than 16x16 pixels because 16x16 pixels images contain at most 256
  colors.
- `four_bit_depth`: the image will be reduced to at most 16 colors, which fit in
  4 bits. This halves the per-color storage requirements compared with 8 bits.
  However, there is only so much you can do with just 16 colors. Textures that
  are bigger than 4x4 pixels and have color variety will be susceptible to
  quality loss.
- `two_bit_depth`: the image will be reduced to at most four colors, which fit
  in 2 bits. This halves the per-color storage requirements compared with 4
  bits. This quantization target is only practical for extremely simple textures
  or if you find posterization effects artistically fitting.
- `one_bit_depth`: the image will be reduced to at most two colors, which fit in
  1 bit. This quantization target is only practical for extremely simple
  textures or if you find posterization effects artistically fitting.

With the exception of `none`, all of the targets above use an image-specific
palette, and a configurable amount of
[dithering](https://en.wikipedia.org/wiki/Dither) (see the
[`color_quantization_dithering_level`](#color_quantization_dithering_level)
option). `auto` and `eight_bit_depth` provide a reasonably good quality while
yielding high space savings, even for big textures with plenty of color variety.

Example:

```toml
color_quantization_target = 'none'
```

#### `color_quantization_dithering_level`

**Type**: [Float](https://toml.io/en/v1.0.0#float) in the [0, 1] interval

**Default value**: `0.85`

The level of dithering that will be applied when quantizing colors. This option
has no effect if `color_quantization_target` is set to not perform color
quantization.

Dithering is a technique that improves the perceived color depth of
color-quantized images by diffusing the reduced number of available colors in
areas that lost color information. The dithered areas appear more as if they had
their original colors preserved, reducing color banding artifacts. However,
dithering can introduce noisy, hard-to-compress diffusion patterns.

In most images, especially natural-looking ones, color quantization saves enough
space to compensate for the decreased dithered areas compressibility, so lots of
dithering is a good idea because it gives better-looking results and still
reduces file sizes. Extreme counterexamples are big images composed of big
constant color blocks whose color is not in the quantized palette: here,
dithering would reduce compressibility a lot while yielding a worse look than
just completely replacing the color of the blocks, so reducing the dithering
level is warranted.

```toml
color_quantization_dithering_level = 1
```

#### `maximum_width_and_height`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) greater than or equal to
zero

**Default value**: `8192`

Sets the maximum width and height of the images that PackSquash will accept
without throwing an error. The rationale behind this option is that limiting the
size of the images that PackSquash can deal with sets a high bound of memory
usage and image processing time, helping authoring efficient packs, and
improving the compatibility of the pack with GPUs that only support smaller
texture sizes.

In relation to the last point, Minecraft internally builds texture atlases with
textures of resource packs, for efficiency reasons (a texture atlas is generated
by stitching individual textures together so that they become part of a single
larger texture). Assuming a maximum texture size of 65536x65536, 256 individual
textures of 4096x4096 would fit in such an atlas, which is a fairly low number
and not enough to hold all vanilla textures. Resource packs that replace only a
few atlased textures may get away with textures that are even bigger than
4096x4096, but that might only happen on "beefy" GPUs, and, in general, it does
not make much sense to mix small textures with such big textures.

It only makes sense to increase the default value if you know what you are
doing, the affected images contain several animation frames or are texture
atlases themselves, or use mods that somehow relax the need for GPUs to support
big textures. If you do not plan on pushing Minecraft to its limits or making
high-definition resource packs, setting this to an even lower value than the
default is a good thing.

Example:

```toml
maximum_width_and_height = 4096
```

#### `skip_alpha_optimizations`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If set to `true`, this option prevents the color values of completely
transparent pixels from being changed to achieve better compression. This
optimization is visually lossless: completely transparent pixels are invisible
no matter their color, and images that lack an alpha channel are not affected.
However, if the image is meant to be used as data by custom shaders, edited
further or contains steganographic data, this optimization may have undesirable
side effects. Disabling alpha optimizations also reduces the time it takes to
optimize an image, at the cost of a maybe increased file size.

Example:

```toml
skip_alpha_optimizations = true
```

#### `downsize_if_single_color`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If `true`, single-color textures that are estimated to be safe to resize will be
downsized to the minimum resolution that maintains the maximum mipmap level.
This usually provides significant space savings without side effects for this
kind of textures, but in some cases, such as when using some modifications,
custom shaders, animated textures, or custom fonts, issues may occur.

Example:

```toml
downsize_if_single_color = true
```

#### `png_obfuscation`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If `true`, the generated PNG files will be mangled in a way that renders them
harder to view outside of Minecraft. The obfuscation technique used by
PackSquash is not robust against some scenarios or expert knowledge, but it does
not increase file size.

This option is only taken into account if the
[`png_obfuscation_incompatibility`](#work_around_minecraft_quirks) quirk is not
being worked around.

Example:

```toml
png_obfuscation = true
```

### Shader files

You can customize how PackSquash optimizes the `.vsh`, `.fsh`, and `.glsl` files
of a pack with the following [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair).

#### `shader_source_transformation_strategy`

**Type**: [String](https://toml.io/en/v1.0.0#string)

**Default value**: `minify`

Defines how the GLSL source code of shaders will be transformed to a more
optimized but semantically equivalent representation.

Please note that PackSquash may not be able to transform some shaders due to
technical limitations. When this happens, the optimization strategy text for
affected shaders will highlight that situation, and PackSquash will act as if
the `keep_as_is` strategy was selected. Some of these limitations might be
removed in the future, rendering PackSquash capable of transforming more shaders
according to the selected strategy.

The available source transformation strategies are:

- `minify`: minifies the shader source code (i.e., removes unnecessary
  whitespace, line breaks, comments and preprocessor directives) to save space
  and improve parsing performance.
- `prettify`: prettifies the shader (i.e., formats its source in an indented,
  human-readable form), while expanding and removing preprocessor directives and
  comments.
- `keep_as_is`: adds the source code as-is to the generated ZIP file, without
  removing comments or expanding preprocessor directives.

Example:

```toml
shader_source_transformation_strategy = 'keep_as_is'
```

#### `is_top_level_shader`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true` for every vertex and fragment shader, `false` for
include shaders

If `true`, PackSquash will consider this shader to be a top-level one (i.e.,
parsed by Minecraft as a translation unit and never included in other shaders).
When `false`, this shader will not be considered top-level.

The value of this option is only honored for vertex and fragment shaders. By
definition, include shaders never are top-level, so they can't be marked as
top-level via this option.

In the vast majority of scenarios, vertex and fragment shaders are not meant to
be included in other shaders (i.e., they are top-level). However, Minecraft
technically allows such inclusion to happen, and it is exceedingly difficult for
PackSquash to determine whether a vertex or fragment shader is being included by
other shader. PackSquash needs to know whether a shader is top-level to decide
whether it is appropriate to expand and remove preprocessor directives: if it is
not top-level but PackSquash thinks it is, removing preprocessor directives may
change the preprocessor behavior in the top-level shader, yielding potentially
different GLSL source code.

It is strongly recommended to only explicitly set this option to `false` for the
exceedingly rare cases where the default behavior generates semantically altered
GLSL code, as doing so will make it harder for PackSquash to optimize the
affected shaders. Otherwise, omitting it is the most appropriate course of
action.

Example:

```toml
is_top_level_shader = false
```

### Legacy language files

You can customize how PackSquash optimizes the `.lang` files used in older
resource packs with the following [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair). Please note that these files
are only optimized on resource packs that target Minecraft 1.12.2 or older
versions unless you set
[`automatic_asset_types_mask_detection`](#automatic_asset_types_mask_detection)
to `false`.

#### `minify_legacy_language`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, the legacy language file will be minified: empty lines and comments
will be removed. This saves space and improves parsing performance. If `false`,
the file will still be validated for errors but left as-is. Line endings are
normalized to Unix style (using a single LF character) no matter what.

Example:

```toml
minify_legacy_language = false
```

#### `strip_legacy_language_bom`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, the BOM in the first line of the file will be stripped. This usually
saves space and avoids user confusion, as the BOM is normally introduced
inadvertently, and Minecraft interprets the BOM as a part of the line.
Therefore, the BOM may undesirably become part of the key of the first language
string, causing it not to work or prevent a comment from being parsed as such.

However, if a pack relies on the BOM being there because it refers to the first
language string with a BOM, this behavior might have undesirable consequences.
In such cases, set this option to `false` to leave the BOM alone.

Example:

```toml
strip_legacy_language_bom = false
```

### Command function files

You can customize how PackSquash optimizes the `.mcfunction` files used in data
packs with the following [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair).

#### `minify_command_function`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

If `true`, the command function file will be minified: empty lines and comments
will be removed. This saves space and improves parsing performance. If `false`,
the file will still be validated for errors but left as-is. Line endings are
normalized to Unix style (using a single LF character) no matter what.

Example:

```toml
minify_command_function = false
```

### Properties files

You can customize how PackSquash optimizes the `.properties` files of a pack
with the following [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair). Please note that these files
only are processed if [`allow_mods`](#allow_mods) includes `OptiFine`.

<sub><sup>These options are only supported if PackSquash was compiled with
support for the OptiFine mod, which always is the case if you use a PackSquash
build downloaded from this repository.</sup></sub>

#### `minify_properties`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `true`

When `true`, the properties will be minified, which removes comments and
unnecessary whitespace to improve space savings. If you change this option to
`false`, the properties will not be changed at all, but they will still be
validated by PackSquash.

Example:

```toml
minify_properties = false
```

### Compressed compound NBT tag files

You can customize how PackSquash optimizes compressed compound NBT tag files,
such as [structure `.nbt` files](https://minecraft.wiki/w/Structure_file) in
data packs, with the following [key and value
pairs](https://toml.io/en/v1.0.0#keyvalue-pair).

#### `nbt_compression_iterations`

**Type**: [Integer](https://toml.io/en/v1.0.0#integer) in the [0, 255] interval

**Default value**: `15`

Sets the number of Zopfli compression iterations that PackSquash will do to
compress raw NBT data that amounts to a magnitude of 1 MiB. This option is
similar to [`zip_compression_iterations`](#zip_compression_iterations) and is
used to feed the same linear model but with parameters better suited for NBT
data compression.

When the number of compression iterations drops to zero, which happens when this
option is set to zero or the NBT file is pretty big, a faster Gzip compression
algorithm that is likely to generate bigger files will be used. On the other
side, the number of iterations is limited to a maximum of 20. Values greater
than 20 are still useful for this setting because they change the threshold
where iterations start being reduced in order to keep acceptable performance
levels.

Example:

```toml
nbt_compression_iterations = 20
```

### Custom files

Any unknown pack file can be marked as a custom file with the following [key and
value pairs](https://toml.io/en/v1.0.0#keyvalue-pair), making it possible to
customize how PackSquash deals with files that would otherwise be inevitably
skipped. Custom file options are ignored for files that belong to any known
type.

> [!NOTE]
> Be mindful that *this feature does not substitute proper support for files
> that are read by the game*: if you are using it to work around such a missing
> feature, please let us know by opening an issue or getting in touch over
> Discord.

#### `force_include`

**Type**: [Boolean](https://toml.io/en/v1.0.0#boolean)

**Default value**: `false`

If `true`, the custom file will be copied to the generated ZIP file as-is,
without any specific optimizations. A `false` value explicitly asks for the
default behavior of skipping the file.

Example:

```toml
force_include = true
```

## Examples

You can run the examples proposed in this section by copying their contents to a
text file, saving it wherever you like, and then launching PackSquash with the
path to your options file as the first parameter. To do so, you can use a
command prompt or terminal.

On Windows, you can open a command prompt by following [these
instructions](https://www.wikihow.com/Open-the-Command-Prompt-in-Windows), and
then type a command like this (please remember to replace the example paths with
the appropriate ones):

```
"C:\path\to\your\packsquash\executable\packsquash.exe" "C:\path\to\your\settings\file.toml"
```

Alternatively, if you don't like to type commands in the Windows command prompt,
you can create a shortcut that does this for work for you or drag-and-drop the
file to the executable. Keep in mind that doing so might complicate
troubleshooting, as the PackSquash window will be immediately closed if
something goes wrong, and you won't be able to see the error messages that
PackSquash has shown.

Of course, you are free to modify these examples to suit your needs better or
even create your options files from scratch.

### Basic options file

This is a minimal valid options file. It only specifies the pack directory and
leaves the rest of the options set to their defaults. The generated ZIP file
will be saved at `pack.zip` in the current directory.

> [!TIP]
> We recommend starting with a minimal options file like this and adding options
> to it on an as-needed basis to keep the file smaller, more readable, and
> easier to upgrade to future PackSquash versions.

```toml
pack_directory = 'C:\path\to\pack'
```

### Basic options file but tuned for extreme compression

By default, PackSquash is designed to create quite small ZIP files using a
reasonable amount of computing power. However, it is possible to squeeze out
even more savings by changing some settings to be on the extreme side:

```toml
pack_directory = 'C:\path\to\pack'

recompress_compressed_files = true
zip_compression_iterations = 30
zip_spec_conformance_level = 'balanced'
skip_pack_icon = true

['**/*?.png']
image_data_compression_iterations = 15

['**/*?.nbt']
nbt_compression_iterations = 20
```

### Basic options file with extraction protection

This example is like [Basic options file](#basic-options-file), but it enables
PackSquash to do more aggressive ZIP file optimizations, including extraction
protection, described in the [`zip_spec_conformance_level` option
documentation](#zip_spec_conformance_level). However, it might be a bad idea to
do these optimizations blindly: they make the ZIP file more cumbersome to work
with, the extraction protection is not unbreakable, and they change how
PackSquash reuses previously generated ZIP files. Check out the mentioned option
documentation for more information.

```toml
pack_directory = 'C:\path\to\pack'
zip_spec_conformance_level = 'disregard'
```

### Complex options file

This example options file changes every option that PackSquash offers, just for
the sake of doing so. You will probably not need to do this in practice, and
some values may be outright inappropriate for you, but this is useful to see how
everything is grouped together.

> [!CAUTION]
> **Carelessly copying and pasting this example is a bad idea** and makes
> troubleshooting harder than you may realize at first sight: newer versions of
> PackSquash may adjust default values to be more optimal or introduce breaking
> changes to the available options, so by explicitly setting options without a
> valid reason, you increase the workload required to verify their relevance
> when switching PackSquash versions for no benefit. Please use this example
> only if you have a clear and justified reason to do so.

<details>
<summary>Click here to view this example</summary>

```toml
# Lines that start with # are comments, and PackSquash ignores them.
# You can also start a comment in the middle of a line with #, that
# spans until the end of the line

# Global options
pack_directory = 'C:\path\to\pack'
output_file_path = 'C:\path\to\result\pack\zip\file\my_pack.zip'
recompress_compressed_files = true
zip_compression_iterations = 5
automatic_minecraft_quirks_detection = true
# The value of this option is ignored due to automatic quirk detection
# being enabled, but this usually does not matter
work_around_minecraft_quirks = ['grayscale_images_gamma_miscorrection', 'restrictive_banner_layer_texture_format_check', 'bad_entity_eye_layer_texture_transparency_blending', 'java8_zip_parsing', 'ogg_obfuscation_incompatibility', 'png_obfuscation_incompatibility']
automatic_asset_types_mask_detection = true
allow_mods = ['OptiFine', 'Minecraft Transit Railway 3']
skip_pack_icon = true
validate_pack_metadata_file = true
ignore_system_and_hidden_files = false
zip_spec_conformance_level = 'high'
# These two are actually ignored due to the ZIP spec conformance level
size_increasing_zip_obfuscation = true
percentage_of_zip_structures_tuned_for_obfuscation_discretion = 100
never_store_squash_times = true
# System dependent values. PackSquash automatically uses appropriate
# defaults for your system, so usually you should not need to set
# these
threads = 4
spooling_buffers_size = 128 # MiB
zip_comment = 'Created with PackSquash'

# Per-file options below

# A special silence file that for some reason must be kept exactly as-is
# (usually not the case)
['assets/craftmine/sounds/special_silence.ogg']
empty_audio_optimization = false

# Audio several hours long that would take a lot of time to transcode,
# optimize and validate
['assets/craftmine/sounds/forest_soundscape.ogg']
transcode_ogg = false
two_pass_vorbis_optimization_and_validation = false

# Other Ogg files do not get transcoded
['**/*?.ogg']
transcode_ogg = false

# Lossless music files get pitch shifted, compressed with good quality,
# and obfuscated if allowed by the target Minecraft version
['**/*?.{flac,wav}']
ogg_obfuscation = true
channels = 2
sampling_frequency = 44100
target_pitch = 1.5
bitrate_control_mode = 'VBR'
target_bitrate_control_metric = 128

# JSON files with comments get prettified and nothing is removed from them
['**/*?.jsonc']
minify_json = false
delete_bloat_keys = false
sort_json_object_keys = false

# Do not allow comments in any JSON file.
# Comments may be an useful JSON extension for documentation purposes, so
# please avoid blindly copying and pasting this unless you really want to
# limit their usage to .jsonc, .jemc, .jpmc, .mcmetac and .bbmodelc files!
['**/*?.{json,jem,jpm,mcmeta,bbmodel}']
always_allow_json_comments = false

# Quantize big natural-looking image, doing the highest quality dither
['assets/craftmine/textures/landscape.png']
color_quantization_target = 'eight_bit_depth'
color_quantization_dithering_level = 1

# Compress other textures losslessly no matter what. Keep them small,
# but don't downsize them, and obfuscate them if the target Minecraft
# version supports it
['**/*?.png']
image_data_compression_iterations = 15
color_quantization_target = 'none'
maximum_width_and_height = 2048
skip_alpha_optimizations = true
downsize_if_single_color = false
png_obfuscation = true

# Don't minify shaders
['**/*?.{fsh,vsh,glsl}']
shader_source_transformation_strategy = 'keep_as_is'
# It usually is a bad idea to explicitly configure this option. Read
# its documentation for more details
#is_top_level_shader = false

# Don't touch Minecraft 1.12.2 or older language files
['**/*?.lang']
minify_legacy_language = false
strip_legacy_language_bom = false

# Don't minify properties files
['**/*?.properties']
minify_properties = false

# Don't minify an example command function.
# Note that command functions are only accepted in data packs, and
# data packs do not contain other file types shown throughout this
# example file!
['data/craftmine/functions/main.mcfunction']
minify_command_function = false

# Compress NBT data with more iterations for better compression
['**/*?.nbt']
nbt_compression_iterations = 20

# Include copyright and authorship information files in any directory.
# More information:
# https://www.gnu.org/prep/maintain/html_node/License-Notices.html
# https://www.gnu.org/prep/maintain/html_node/Recording-Contributors.html
['**/{COPYING,AUTHORS}']
force_include = true
```

</details>
