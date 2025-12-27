PackSquash is officially available for a variety of platforms and is distributed
in several ways. This document explains how to get started with PackSquash on
the supported platforms.

<!-- omit from toc -->
## Table of contents

- [▶️ GitHub action](#️-github-action)
- [🪟 Windows](#-windows)
  - [Tutorial video](#tutorial-video)
- [🐧 Linux](#-linux)
  - [APT packages](#apt-packages)
  - [Static binaries](#static-binaries)
  - [Docker image](#docker-image)
- [🍎 macOS](#-macos)
- [🔨 Custom builds](#-custom-builds)
- [🧪 Unstable builds](#-unstable-builds)

## ▶️ GitHub action

If your pack lives on a GitHub repository, [the official
action](https://github.com/marketplace/actions/packsquash) makes it easier and
more convenient to validate, optimize, and deploy your pack: you can set up the
necessary [GitHub Actions](https://github.com/features/actions) workflow from
your web browser.

## 🪟 Windows

The official Windows executables are portable (i.e., require no installation or
system configuration) and target computers that run Windows 10 or newer on
modern x64 CPUs.

For compatibility purposes, the precise definition of a "modern x64 CPU" is any
CPU that supports the [x86-64-v2 microarchitecture
level](https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels). The vast
majority of CPUs released in the early 2010s or later support this
microarchitecture level. Windows computers sporting ARM CPUs, such as some
tablets and laptops, can run these builds at a performance cost through the
[transparent WOW64 x86 emulation
subsystem](https://learn.microsoft.com/en-us/windows/arm/apps-on-arm-x86-emulation
).

After downloading the Windows executable from the [releases
page](https://github.com/ComunidadAylas/PackSquash/releases) and extracting it
to a location of your choice, you **need to pass PackSquash some options for it
to be able to process your pack, which are usually contained in a separate text
file**. Please refer to the [options files
documentation](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files)
for more details, which includes helpful examples you can use as templates for
your configuration.

### Tutorial video

Do you find the above documentation too dauting, or do you just feel like a
video guide would get you up to speed faster? Good news: the Discord user
`@n0smoke` has kindly uploaded a tutorial on how to use PackSquash and its ZIP
file protection feature on Windows to YouTube. You can check it out by clicking
on the image below!

<div align="center">
<a href="https://www.youtube.com/watch?v=N7X0qX3Ci9Q" alt="@n0smoke's PackSquash tutorial YouTube video"><img src="https://github.com/ComunidadAylas/PackSquash/assets/7822554/42b49a58-41d8-4b40-a76b-d516a0894417" alt="@n0smoke's PackSquash tutorial YouTube video thumbnail"></a>
</div>

Please note that `@n0smoke` created this video independently and they are not
affiliated with the PackSquash project. We cannot vouch for the accuracy and
appropriateness of its content over time, but we recognize its helpfulness and
wish to visibilize it to those who may benefit from it.

## 🐧 Linux

On Linux, PackSquash officially targets systems with x86-64-v2 compatible CPUs
and AArch64/ARM64 CPUs, and is distributed in a bunch of binary formats. You can
choose between the formats described below, depending on the flavor of Linux
userspace you are using and your needs.

### APT packages

APT packages are the recommended way to install PackSquash on Debian-based
GNU/Linux distros because they integrate with the operating system and its
package management infrastructure, facilitating efficient installation,
uninstallation, and updates. However, the PackSquash APT packages are
distributed in a separate APT repository, which must first be added to the list
of package sources.

To add the PackSquash APT repository to the system list of package sources, run
the following composite command as root, which you can copy and paste in a root
shell:

```shell
mkdir -p /etc/apt/keyrings && \
wget -O /etc/apt/keyrings/packsquash.key https://deb.packages.packsquash.aylas.org/public.key && \
echo 'deb [signed-by=/etc/apt/keyrings/packsquash.key] https://deb.packages.packsquash.aylas.org/debian stable main' > /etc/apt/sources.list.d/packsquash.list
```

After the previous command completes successfully, installing PackSquash boils
down to running the following commands as root:

```shell
apt update
apt install packsquash
```

If all goes well, then PackSquash will be available at the `packsquash` command.
Otherwise, if some dependency can't be met, check that you are not using a too
old or incompatible distro. Currently, PackSquash APT packages target Debian
Bullseye, so they will work on any distro that is binary and package-compatible
with it, such as:

- Debian Bullseye (11) itself and later.
- Ubuntu Focal Fossa (20.04) and later.
- Linux Mint Ulyana (20) and later.
- Pop_OS! 20.04 and later.
- Raspberry Pi OS 2021-10-30 (Bullseye-based) and later.
- Armbian v20.02.2 and later.
- Other distros based on a compatible version of the listed distros (Devuan,
  Kubuntu, Xubuntu, Lubuntu, Trisquel, Kali Linux...).

The `packsquash` command will read options from its standard input stream or, if
a path to an options file was passed as a parameter, from that file. The
[options file
documentation](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files)
describes the format of such options, whose specification is necessary for
PackSquash to operate. You can get a list of supported command-line parameters
via the conventional `--help` switch.

### Static binaries

Static binaries are less optimized for specific environments than more
specialized distribution methods, but they will run on almost any Linux machine
with a compatible CPU with no setup required, making them an effective fallback
option. Unlike APT packages, they are guaranteed to work on non-Debian-based
environments with a recent enough kernel, such as:

- Arch Linux.
- Fedora.
- RedHat Enterprise Linux.
- Gentoo.
- openSUSE.
- NixOS.
- Android.

These binaries can be downloaded from the [releases
page](https://github.com/ComunidadAylas/PackSquash/releases). Note that on
Linux, you must explicitly give binaries permission to execute, which you can do
with the `chmod +x` command. Other than that, they behave the same as those
installed by the [APT packages](#apt-packages).

If you are interested in a package for a non-Debian-based distro and have the
required knowledge and time, feel free to package it and contact us. We can
update this guide and give your favorite distro some love!

### Docker image

PackSquash is also available as a lightweight, distroless Docker image on the
[GitHub Container
Registry](https://github.com/ComunidadAylas/PackSquash/pkgs/container/packsquash).
This container may come in handy for advanced use cases, such as integration
with GitLab build pipelines and professional deployments. There are several tags
you can pick from:

- `latest`: refers to the latest release of PackSquash at any given time.
- `edge`: refers to the latest unstable build of PackSquash at any given time.
- `X.X.X`: refers to the PackSquash `X.X.X` release.
- `X.X`: refers to the latest patch version of the PackSquash v`X.X` release
  track. This enables automatic updates to newer but compatible PackSquash
  versions as they are released.

The image has its entrypoint set to the PackSquash binary, so any arguments
specified in the container configuration will be passed to PackSquash. In other
words, you can use these images in much the same way as if you were using
PackSquash directly. For example, the following `docker run` command optimizes a
pack at `/tmp/pack` with the options file at `/tmp/pack/packsquash.toml`:

```shell
docker run -v /tmp/pack/:/pack \
    ghcr.io/comunidadaylas/packsquash:latest \
    /pack/packsquash.toml
```

It is possible to use these images with GitLab pipeline runners by leveraging
Docker-in-Docker, although the setup is somewhat inelegant. To do so, add the
following lines to the `[runners.docker]` section of your GitLab runner
configuration file:

```toml
privileged = true
volumes = ["/certs/client", "/cache"]
```

Then you can use the image in your GitLab jobs like this:

```yaml
build-job:
  stage: build
  image: docker:20.10.16
  variables:
    # Comes from dind service name (e.g., docker:20.10.16-dind).
    # If you rename it to something else DOCKER_HOST variable will need to be adjusted
    DOCKER_HOST: tcp://docker:2375
    DOCKER_TLS_CERTDIR: ""
  services:
    - docker:20.10.16-dind
  before_script:
    - docker info
  script: |
    docker run -v "$(pwd)":"$(pwd)" --workdir "$(pwd)" \
    ghcr.io/comunidadaylas/packsquash:latest \
    packsquash.toml
  artifacts:
    paths:
      - pack.zip
```

We recommend dedicating a separate runner to PackSquash to mitigate potential
security threats associated with Docker-in-Docker.

For more interesting information and context about the Docker images, please
check out [the PR that introduced
them](https://github.com/ComunidadAylas/PackSquash/pull/111) and [a follow-up
issue](https://github.com/ComunidadAylas/PackSquash/issues/135). Big props to
[@realkarmakun](https://github.com/realkarmakun) for providing examples for this
guide and their code contribution!

## 🍎 macOS

PackSquash also supports updated macOS environments via official universal
binaries (i.e., for both Apple Silicon and Intel-based Macs).

You can download the PackSquash macOS executable from the [releases
page](https://github.com/ComunidadAylas/PackSquash/releases). After downloading
and extracting it to a directory of your liking, make the file executable for
your user by running the `chmod +x` command on a terminal.

Since PackSquash binaries aren't notarized, you may also need to mark them as
safe to run, depending on the software you used to download them and your
[Gatekeeper](https://support.apple.com/en-us/102445) settings. To do this from a
terminal and without requiring administrator privileges use the `xattr -d
com.apple.quarantine` command on the executable.

As with the Windows and Linux ports, you have to pass options to PackSquash to
make it do something useful. How to do this is described on the [options
files](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files) page.

## 🔨 Custom builds

Generating an executable yourself for your system can be an adequate way to
tailor it to your needs or get it running in an unsupported environment, but
this is not for the fainthearted or unfamiliar with software development. The
[contributing
guidelines](https://github.com/ComunidadAylas/PackSquash/blob/master/CONTRIBUTING.md)
explain the technologies used by the project and how to build it.

## 🧪 Unstable builds

As dictated by good software engineering practices, a CI service builds each
revision of the PackSquash source code, generating distribution artifacts for
that revision. So, if you want to try out the latest changes without waiting for
a new release, you can go to the [GitHub Actions build workflow
page](https://github.com/ComunidadAylas/PackSquash/actions/workflows/ci.yml)
and download distribution artifacts for any recent commit. These artifacts are
drop-in replacements for those downloaded from the release page. Note that you
need to be logged in to GitHub to download these artifacts.

Please consider that the unstable PackSquash builds are so-called and not yet
released for a reason: they may contain undocumented, untested, experimental, or
breaking changes, unsuitable for production usage. However, they are great for
trying out the latest features, checking if an issue was fixed, and giving
feedback to developers.
