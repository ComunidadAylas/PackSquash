# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

#### Compression

- The single-color texture downsizing optimization introduced in v0.4.0 is now
  disabled by default until the necessary features to better automatically
  determine its default value are implemented, as it caused issues with e.g.
  more common than expected single-color font textures.

#### Distribution

- The official binaries have been slimmed down by not including code to show
  stack backtraces on panic, which never worked because such binaries don't
  contain the necessary debug symbol data.
- [SLSA](https://slsa.dev/) v1.0 attestations conforming to the level 3 of the
  build track are now generated for the official PackSquash binary artifacts and
  containers.
  - These attestations allow security-conscious users to assert that they were
    generated on GitHub Actions runners with a well-defined build process, and
    that they have not been tampered with since they were generated.
  - The third-party
    [`slsa-verifier`](https://github.com/slsa-framework/slsa-verifier) and
    [Cosign](https://docs.sigstore.dev/verifying/attestation/) tools can be used
    to verify these attestations after downloading the attached provenance data.
  - Due to the numerous technical advantages of SLSA attestations, in addition
    to their decentralized and free nature, the PackSquash project will most
    likely not pursue signing binaries with code signing certificates.

#### User experience

- Some options file deserialization error messages have been shortened and made
  aware of the context in which they appear to suggest likely helpful corrective
  actions.

## [0.4.0] - 2023-06-25

### Added

#### Compression

- Added a single-color texture downsizing optimization to make textures composed
  of pixels of a single color as small as possible while maintaining their
  maximum mipmap level. (Thanks to @alumina6767 for bringing this idea to my
  attention!)
  - A new `downsize_if_single_color` file-specific option has been added to
    control whether this optimization is applied or not.
  - Downsizing single-color textures can have negative side effects if such
    textures are used with custom shaders or custom fonts. We are aware of such
    limitations and look forward to making PackSquash handle such cases better
    in the future, but feel free to let us know if you are affected by them: it
    can be useful to assess the impact of these limitations and discover new
    ones we were not aware of.
- The macOS resource fork ZIP folder (i.e., the `__MACOSX` folder) has been
  added to the set of files and folders ignored by PackSquash when
  `ignore_system_and_hidden_files` is set to `true` (the default value).

#### Distribution

- macOS binaries are now universal, containing native code for both Intel and
  Apple Sillicon based devices.
  ([#41](https://github.com/ComunidadAylas/PackSquash/issues/41))
  - As a result, newer macOS devices with Apple Sillicon (e.g., ARM) CPUs should
    run PackSquash faster.
  - The performance on Intel-based devices should not be affected by this
    change, as the universal binaries still contain optimized x86 code.
- Published PackSquash Docker images at the GitHub Container Registry.
  ([#111](https://github.com/ComunidadAylas/PackSquash/pull/111), thanks
  @realkarmakun for your PR!)

#### Protection

- Audio files can now be protected to make them harder to play outside of
  Minecraft via the new file-specific `ogg_obfuscation` option. This protection
  is independent of the already available ZIP layer protection, so it can be
  used alongside it or not, and can be applied to a subset of pack files.
  - This protection will not work for resource packs targeting Minecraft 1.13.2
    or older. By default, PackSquash will force it to be disabled for such
    versions via the new `ogg_obfuscation_incompatibility` quirk.

#### Documentation

- Included a contributors section in the project readme, following the [All
  Contributors
  specification](https://github.com/all-contributors/all-contributors).
- Created a changelog file.
- Added a repository pull request template.
- Added a GitHub Sponsors button to the repository.

#### Internal

- Configured [Renovate](https://docs.renovatebot.com/) for automated dependency
  updates.
- Added GitHub Codespaces configuration files to allow interested parties to
  quickly spin up an environment suitable for PackSquash development.

### Changed

#### Compression

- Reworked the audio file processing code to be much faster and more effective.
  This is a major change that included the following smaller changes:
  - Removed the dependency on GStreamer in favor of combining several
    specialized libraries to accomplish the necessary audio processing tasks.
    This greatly samplifies PackSquash distribution and installation, removes
    points of failure related to the availability of GStreamer components, and
    makes the audio processing code more efficient. It also enabled the use of
    the audio processing software mentioned below.
  - Integrated [OptiVorbis](https://github.com/OptiVorbis/OptiVorbis), a novel
    solution for lossless optimization and validation of the generated Ogg
    Vorbis files.
  - Migrated to [`vorbis_rs`](https://github.com/ComunidadAylas/vorbis-rs), a
    novel set of bindings for the best-in-breed encoder available, based on the
    reference Vorbis encoder implementation and the aoTuV and Lancer patchsets.
  - The default encoder bitrate management parameter selection has been
    optimized for much better performance and space efficiency without
    compromising audio quality.
  - Empty audio files (i.e., containing only complete silence) are now replaced
    with a minimal Ogg Vorbis file with no audio data, which significantly
    minimizes their size with no adverse effects for the vast majority of
    purposes. This optimization can be disabled if needed via the new
    `empty_audio_optimization` option.
  - The default encoder bitrate management parameters and sampling frequency are
    now different for mono and stereo sounds. The rationale for this change is
    that mono sounds tend to be shorter and mixed with other world sounds, so a
    drop in quality is less noticeable in them than in persistent, music-like
    stereo sounds.
  - PackSquash now falls back to outputting the input audio file if resampling
    did not help to reduce the file size, as long as channel mixing or pitch
    shifting are not requested. This input audio file will be optimized (and
    protected if requested) if the `two_pass_vorbis_optimization_and_validation`
    option is enabled (its default value).
- Revised shader processing code to partially fix long-standing preprocessor
  directive support issues. (Related issue:
  [#187](https://github.com/ComunidadAylas/PackSquash/issues/187))
  - Switched to a new GLSL preprocessor and parser, which is much faster (its
    author claims about 10x faster) and allows to accept files containing
    preprocessor directives outside of external declaration position by
    expanding them before parsing.
  - `minify_shader` has been superseeded by a new
    `shader_source_transformation_strategy` option, which now also allows
    prettifying shader source code.
  - Include shaders (i.e., shaders with a `.glsl` extension) can now be
    standalone statements and expressions.
  - Properly optimizing shaders in the context of Minecraft is an interesting
    and complex problem that we are working on. Even with this partial fix, you
    may find that some shaders still do not work as-is with PackSquash. For more
    information on the known shortcomings and current state of affairs, please
    read [this GitHub issue
    comment](https://github.com/ComunidadAylas/PackSquash/issues/187#issuecomment-1499365532).
- Some internal PNG compression settings have been tweaked for better
  performance and compression on average. In addition, PackSquash now uses the
  cutting-edge `oxipng` raw pixel API, which allows PackSquash to avoid encoding
  intermediate textures.

#### Fixed

- PackSquash no longer fails to process some audio files when using the default
  options. This is a consequence of the new default encoder bitrate management
  parameter selection.
- Audio files with more than two audio channels are no longer accepted, as they
  don't work with Minecraft.
- Fixed "write zero" I/O errors happening for every pack file when the spooling
  buffer size is set to zero. (Thanks to _darkchroma_ for reporting this issue
  on Discord)
- PackSquash now accepts legacy font `.bin` files anywhere in a resource pack,
  like Minecraft does.
- Fixed D-Bus machine ID fallback lookups not working as intended on Linux
  platforms.

#### Distribution

- AppImages have been dropped in favor of statically-linked binaries as a
  distribution method for Linux platforms. Due to the removal of the dependency
  on GStreamer, AppImages are no longer considered a better overall means of
  distro-agnostic distribution. We don't expect the PackSquash CLI to depend on
  external libraries and thus switch back to AppImages anytime soon, but the
  future GUI may benefit from them.

#### User experience

- The options file deserialization error messages have been made more verbose so
  that they are more useful for troubleshooting syntax errors.
- PNG files with unnecessary trailing bytes at the end are no longer rejected;
  instead, such bytes are now silently discarded. These files do not follow the
  PNG specification and may cause interoperability issues, but experience has
  shown that popular closed-source programs (e.g., some versions of Photoshop)
  generate these files, and users can have a cumbersome time identifying and
  fixing them.
  - To achieve a better balance between problem linting capabilities and
    usability, future versions of PackSquash will display a non-fatal warning
    for the troublesome files, so that pack authors can improve the technical
    quality of their pack if they want to.

#### Documentation

- The PackSquash logo has been redesigned to give it a more modern, cleaner
  look. (Thanks to @MiguelDreamer for your work on this!)
- The repository README and other pieces of documentation have been updated and
  proofread.
  - This included updating the Discord username of the main PackSquash author,
    since Discord is phasing out username discriminants.
- Revamped the repository issue templates.

#### Internal

- Slightly optimized the space efficiency of the repeated entry lookup in legacy
  language files by using [radix
  trees](https://en.wikipedia.org/wiki/Radix_tree) instead of hashmaps.
- Renamed the `packsquash-cli` package to `packsquash_cli` to better align with
  [documented Rust crate naming
  conventions](https://github.com/rust-lang/api-guidelines/discussions/29).
- Lots of third-party dependency updates, including prominent libraries such as
  `imagequant`, `oxipng` and `zopfli`. In particular, several `oxipng` updates
  brought significant performance and compression improvements.
  - Some dependency changes addressed minor public security advisories.
- Replaced some unmaintained CI step actions with faster, more up-to-date
  equivalents.
- The official PackSquash web presence has been moved to the `aylas.org` domain,
  under the easy to remember subdomain `packsquash.aylas.org`.
  - We were able to afford this domain, for which we pay 10.1 €/year, thanks to
    our financial contributors.

#### Options

- The following options were removed from options files:
  - `open_files_limit`
  - `minimum_bitrate` (superseeded by `target_bitrate_control_metric`)
  - `maximum_bitrate` (superseeded by `target_bitrate_control_metric`)
  - `minify_shader` (superseeded by `shader_source_transformation_strategy`)
- The following options were added to options files:
  - `two_pass_vorbis_optimization_and_validation`
  - `empty_audio_optimization`
  - `bitrate_control_mode`
  - `target_bitrate_control_metric`
  - `ogg_obfuscation`
  - `downsize_if_single_color`
  - `shader_source_transformation_strategy`
  - `is_top_level_shader`
- The `ogg_obfuscation_incompatibility` quirk has been added to the set of
  quirks accepted by the `work_around_minecraft_quirks` option.

### Removed

#### Performance

- The `open_files_limit` option has been removed because it was hard to use
  properly. PackSquash will instead automatically try to increase the open files
  limit when needed, and fall back to using less threads if the maximum
  attainable limit would not support the desired level of concurrency.

#### Internal

- Dropped build-time dependency on `vergen` in favor of directly gathering
  version metadata from Git during build scripts.
- Dropped the dependency on `atty` in favor of new Rust standard library
  methods.
- Removed pretty panic printing logic. With GStreamer out of the equation,
  panics should be much less likely to happen, so the significant binary size
  taken by this logic was no longer warranted.

[Unreleased]: https://github.com/ComunidadAylas/PackSquash/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/ComunidadAylas/PackSquash/compare/v0.3.1...v0.4.0
