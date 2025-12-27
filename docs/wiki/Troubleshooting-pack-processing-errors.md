PackSquash strives to be correct and robust, handling every pack as it should
according to technical standards and conventions. However, errors may happen
while processing packs for various reasons. This wiki page describes pack
processing errors that, while not being caused by a defect in PackSquash, may
nonetheless happen to you, along with their meaning, probable causes, and
possible solutions.

Of course, if you find an error that you believe to be common and is not in this
list or a defect in how PackSquash deals with valid packs, please open an issue,
so this list may be updated or the bug fixed.

## Table of contents

- [Table of contents](#table-of-contents)
- [Types and characteristics of pack processing errors](#types-and-characteristics-of-pack-processing-errors)
- [Common pack processing errors](#common-pack-processing-errors)
  - [`PNG decode error: invalid signature`](#png-decode-error-invalid-signature)
    - [Meaning](#meaning)
    - [Probable causes](#probable-causes)
    - [Possible solutions](#possible-solutions)
  - [`PNG decode error: invalid image width`](#png-decode-error-invalid-image-width)
  - [`PNG decode error: invalid image height`](#png-decode-error-invalid-image-height)
  - [`PNG decode error: image width exceeds user limit`](#png-decode-error-image-width-exceeds-user-limit)
  - [`PNG decode error: image height exceeds user limit`](#png-decode-error-image-height-exceeds-user-limit)
  - [`Invalid PNG: The texture width or height exceeds the configured maximum size`](#invalid-png-the-texture-width-or-height-exceeds-the-configured-maximum-size)
    - [Meaning](#meaning-1)
    - [Probable causes](#probable-causes-1)
    - [Possible solutions](#possible-solutions-1)
  - [`Pack processing error: Error while performing a ZIP file operation: I/O error: Access is denied. (os error 5)`](#pack-processing-error-error-while-performing-a-zip-file-operation-io-error-access-is-denied-os-error-5)
    - [Meaning](#meaning-2)
    - [Probable causes](#probable-causes-2)
    - [Possible solutions](#possible-solutions-2)

## Types and characteristics of pack processing errors

From PackSquash v0.3.0 onwards, there are two types of pack processing errors:
those that occur while processing a pack file and those that occur while a pack
file is not being processed (usually, when the result ZIP file is being
generated). When any error happens, PackSquash prints error messages to the
standard error stream that start with an exclamation mark (!) and halts the
generation of the pack ZIP file (otherwise, PackSquash would risk outputting a
pack ZIP file in an inconsistent state).

When a pack file processing error happens, an error message like this is shown.
Note the relative pack file path at the beginning, followed by a textual
description of what went wrong when processing that file:

```
! assets/minecraft/textures/blocks/block.png: PNG decode error: invalid signature
```

The other type of pack processing errors output similar error messages, but they
substitute the path shown above with a textual description of the phase where
the error occurred.

## Common pack processing errors

### `PNG decode error: invalid signature`

#### Meaning

PackSquash couldn't find the signature bytes (i.e., data) that mark a PNG file
as being encoded in the PNG format.

#### Probable causes

- The file is not an actual PNG image. For instance, the file could be a JPEG
  image, but with its extension changed to `.png`. Although many image viewers
  will automatically detect the image type and display it without problems,
  PackSquash and Minecraft are **only** able to deal with PNG files. You can
  check whether an image is encoded in PNG by opening it with a PNG file
  manipulation program, using a website like
  [checkfiletype.com](https://checkfiletype.com/), running the Unix `file`
  command on it, or any other method you prefer.
- If the file is indeed a PNG image, it is corrupt or encoded in a very
  unconventional way.

#### Possible solutions

- Re-export the image to PNG with a program that generates known-good PNG files.

### `PNG decode error: invalid image width`
### `PNG decode error: invalid image height`
### `PNG decode error: image width exceeds user limit`
### `PNG decode error: image height exceeds user limit`
### `Invalid PNG: The texture width or height exceeds the configured maximum size`

#### Meaning

A PNG image has a resolution too wide or high for PackSquash, according to the
value of the
[`maximum_width_and_height`](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files#maximum_width_and_height)
option.

#### Probable causes

- The image exceeds the configured width and height limit.
- The image is corrupt.

#### Possible solutions

- Reduce the resolution of the affected image with a suitable image manipulation program so that its width and height fall under the limit. Minecraft avoids referencing raw pixel coordinates and offsets in resource pack files, but be careful to update files that do not and therefore might be sensitive to the resolution change, like a custom shader.
- Increase the value of the [`maximum_width_and_height`](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files#maximum_width_and_height) option for the affected file. However, you should be aware that doing so has some drawbacks, including potentially making your pack not work on certain devices. Please read the documentation of the referenced option for more information.
- Check that other image manipulation programs can open the file. If not, restore a known-good backup of the file.

### `Pack processing error: Error while performing a ZIP file operation: I/O error: Access is denied. (os error 5)`

#### Meaning

While writing the result ZIP file to the filesystem, a Windows API call returned a [`ERROR_ACCESS_DENIED` error code](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/18d8fbe8-a967-4f1c-ae50-99ca8e491d2d). The precise error message might change depending on the operating system version, regional configuration, and whether an external program is performing [DLL injection](https://en.wikipedia.org/wiki/DLL_injection) on the PackSquash process to [hook](https://en.wikipedia.org/wiki/Hooking) the API calls it uses (this is usually done by antivirus, DRM and anti-cheat software, and by malware).

On POSIX-like operating systems (for example, Linux and macOS) the equivalent error has a [different numeric code](https://github.com/torvalds/linux/blob/d999ade1cc86cd2951d41c11ea769cb4452c8811/include/uapi/asm-generic/errno-base.h#L17), and software that intrusively modifies the operation of other software is different and much less prevalent. On these platforms, the last components of the error message will be radically different.

#### Probable causes

- Your operating system user does not have enough permission in the folder where the output ZIP file would be to write and modify the folder itself and the files contained within it.
- External software is modifying the usual operation of the OS API calls used. As mentioned above, the most likely culprits are antivirus, DRM and anti-cheat software, and malware.

#### Possible solutions

- Make sure that your operating system user has enough permissions in the folder where you want to store the ZIP file generated by PackSquash.
- Repeatedly change the value of the output file path option to another folder until you find one that works.
- Remove any malware from your device.
- Temporarily disable or uninstall any antivirus, DRM, and/or anti-cheat software. If you manage to pinpoint the problem to a particular piece of external software, please contact its authors so they can fix this likely undesired effect of their hooks. Alternatively, you can also contact the PackSquash developers, so they can initiate the corresponding due process for you and at least document the problems with such software (but keep in mind that PackSquash will not fix other programs).
  - An user reported that a particular version of the Avast antivirus caused this error.
