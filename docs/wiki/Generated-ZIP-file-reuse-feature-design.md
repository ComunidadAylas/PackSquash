> [!WARNING]
> 🔧 **This wiki article contains design specifications intended for a technical
> audience** 🔧
>
> If you are just an end user, please consider that you probably are better off
> reading other documentation that uses simpler terms unless you really want to
> read the nitty-gritty details or are looking for some very specific fact about
> the topic.
>
> Moreover, for PackSquash versioning, the details contained in this document
> are not considered a public contract in any way. They could change in any
> version in a breaking way without any notice.

<!-- omit from toc -->
## Table of contents

- [Introduction and software requirements](#introduction-and-software-requirements)
- [Design overview](#design-overview)
- [Description of key design elements](#description-of-key-design-elements)
  - [System identifiers](#system-identifiers)
  - [Application salt](#application-salt)
  - [Filesystem-provided file modification timestamps](#filesystem-provided-file-modification-timestamps)
  - [Squash Times](#squash-times)
  - [Format-preserving encryption](#format-preserving-encryption)

## Introduction and software requirements

PackSquash v0.3.0 introduced a feature called _append mode_, _previously
generated ZIP file reuse_ or _caching_, which allows it to detect what files
were changed since the last time it was executed on a pack with high accuracy
under most circumstances. When this feature is used, and a likely unchanged file
is detected, all processing is skipped for that pack file, so it is just added
and copied to the new version of the output ZIP file. This is much faster than
processing it again.

Obviously, for a feature like this to work, it is necessary to store data
somewhere to track the file changes in a way that can be efficiently read back
and compared with later. So, in addition to the functional requirement stated
above, the feature must satisfy the following non-functional requirements:

- It must be at least more time-efficient than processing the pack file again;
  otherwise, the feature would be useless.
- It must be privacy-first and storage-efficient, saving the lesser amount of
  data needed for the feature to work. The data must not be readable by any
  third party under reasonable threat models.
- It must not reduce PackSquash portability, require extra installation steps,
  or store files in some hardcoded, system, or user-dependent data store that
  may make it harder to do reproducible runs or support use cases involving
  running PackSquash by different users, operating systems, or machines. In
  other words, it must be as stateless as possible for the end-user.
- It must be a reasonably simple design, as simple to maintain and reason about
  as possible.
- Lastly, it must be as user-friendly as reasonably possible, considering the
  previous requirements. It is acceptable to exchange some user-friendliness if
  that demonstrably helps to fulfill any of the previous requirements, but it is
  not acceptable to overdo this point.

## Design overview

The design of the current PackSquash implementation for this feature revolves
around the following key functional blocks and concepts:

- System identifiers.
- Application salt.
- Filesystem-provided file modification timestamps.
- Squash Times.
- [Format-preserving
  encryption](https://en.wikipedia.org/wiki/Format-preserving_encryption) (FPE),
  also known as _sanitization_ in some technical literature.

The following flowchart shows how these items relate to each other in a
high-level overview of the pack file processing algorithm, focusing on the
aspects relevant to the feature at hand.

<p align="center"><img src="./assets/PackSquash design outline.svg" alt="Flowchart for the pack file processing algorithm"></p>

This design satisfies the requirements stated above for the following reasons:

- It is trivial to conclude that it allows skipping processing unchanged files.
- Getting the file modification time from the filesystem is an operation that
  can be done very quickly in most filesystems. This operation might be slower
  in distributed filesystems, but so might be every operation with them in
  general. It also can be done efficiently because file metadata is already
  retrieved to iterate through the pack directory, so checking the modification
  time field only adds a marginal time cost. Timestamps are also quick to
  compare and compute, unlike hashes, which require looking at the file
  contents.
- Modification timestamps are the bare minimum amount of data needed to check
  whether a file has plausibly changed or not. Also, we believe that the
  designed cryptosystem is effective to maintain timestamp and system ID
  confidentiality under ciphertext-only attacks and chosen-plaintext attacks.
  This claim is at least plausible due to the guarantees provided by the
  cryptographic primitives used, which are discussed in [NIST Special
  Publication 800-38G § Appendix
  B](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-38G.pdf).
- The timestamps are stored in the generated ZIP file using standard fields
  reserved for this purpose in the ZIP file records, so no additional data
  stores are required. Transferring the generated ZIP file also transfers the
  timestamps.
- The design is based on standard cryptographic primitives and simple timestamp
  comparisons, which are easy to maintain.
- Because the metadata is kept within the ZIP file, there is little room for
  user error, providing that the user is aware that some environment changes may
  render that metadata unable to be read back properly or obsolete. These
  changes are documented in the [options
  files](https://github.com/ComunidadAylas/PackSquash/wiki/Options-files#zip_spec_conformance_level)
  wiki article. Moreover, this design can work _automagically_ in the most
  common use cases without any configuration.

## Description of key design elements

The following sections present the information about key design elements needed
for a thorough understanding of the design.

### System identifiers

System identifiers are unsigned integers that are guaranteed to be more
repeatable across PackSquash executions than a randomly generated number, fairly
unpredictable, and unique. In practice, the way to get an integer that is
somewhat unique, unpredictable, and repeatable is to query the computer for
machine-specific hardware or software data that is expected to be long-lived, so
that is why the system identifiers are named as such. The reader may be more
familiar with other terms that refer to a similar (albeit not identical) concept,
like hardware ID (HWID), or machine ID.

The definition above is so generic because the ways to get system identifiers
vary across operating systems and hardware platforms. They may also vary within
the same environment (i.e., operating system install and hardware combination)
due to user intervention or other usually atypical factors. The user may also
spoof a system identifier, but this is not a concern in typical scenarios
because it is reasonable to assume that no malicious party is in control of the
computer that is used to generate ZIP files and that the end-user has no
interest in spoofing identifiers in a way that undermines the suitability of the
system identifier for the cryptosystem. Social engineering attacks are excluded
from the threat model because they can compromise every conceivable
cryptosystem. In this vein, it should be noted that **if any party succeeds in
making PackSquash use a weak (i.e., predictable, non-unique) system identifier,
the security of the whole cryptosystem would be compromised**.

Another consequence of the facts stated above is that system identifiers may be
of varying suitability for long-term cryptographic key generation. The impact
that this has on information security and usability is addressed by deriving the
required cryptographic key with the following algorithm:

1. Every candidate system identifier for the running environment is retrieved.
2. _Volatile_ system identifiers (i.e., that may change without any user
   intervention that may be reasonably expected to be targeted at changing the
   identifier) are excluded. If at least one non-volatile candidate system
   identifier remains, their concatenation is used as input to a [key derivation
   function (KDF)](https://en.wikipedia.org/wiki/Key_derivation_function) that
   produces a 256-bit, uniformly random key. If only volatile candidate system
   identifiers are available, their concatenation is used instead as input to
   the such KDF.
3. If the KDF had volatile candidate system identifiers as input, or the input
   candidate system identifiers provided a total of less than 14 bytes of data,
   warnings are emitted, so the user is notified of the fact that the identifier
   may not be suitable enough and can take corrective measures.

On Windows, PackSquash retrieves candidate system identifiers from the following
sources:

- The `MachineGuid` registry key located at
  `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography`.
- The SMBIOS DMI product UUID, read through the [`Win32_ComputerSystemProduct`
  WMI
  class](https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystemproduct),
  that is provided by the motherboard firmware on x86 hardware platforms.
- The serial number of the volume containing the system root directory, i.e.
  where Windows was installed.
- The last Windows service or feature pack installation date, stored in the
  registry at `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows
  NT\CurrentVersion\InstallDate`.

On Linux, PackSquash retrieves candidate system identifiers from the following
sources:

- [The D-Bus/systemd machine
  ID](https://www.freedesktop.org/software/systemd/man/machine-id.html).
- The SMBIOS DMI product UUID, read from the `/sys/class/dmi/id/product_uuid`
  file exposed by the DMI firmware driver.
- SMBIOS DMI serial numbers, as collected by the udev database. Unlike the
  sysfs-based method above, this usually does not require root access and offers
  more serial numbers.
- [The boot ID provided by the kernel via
  sysctl](https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html#random).
- The
  [`gethostid()`](https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostid.html)
  C function, defined by POSIX.1-2017 as an extension to the ISO C standard.

On macOS, PackSquash retrieves candidate system identifiers from the following
sources:

- The `IOPlatformSerialNumber` of the `IOPlatformExpertDevice` [I/O
  Kit](https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/Features/Features.html)
  device.
- The
  [`gethostid()`](https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostid.html)
  C function, defined by POSIX.1-2017 as an extension to the ISO C standard.

On every platform, the `PACKSQUASH_SYSTEM_ID` environment variable can be set to
an arbitrarily-long hexadecimal string (recommended) or [RFC4122 UUID
string](https://www.ietf.org/rfc/rfc4122.txt) that will be parsed as the system
identifier, ignoring any other candidates. This is meant to support use cases
where it is necessary to use a fixed system identifier, be it because PackSquash
is not capable of getting a suitable one or because it is expected to be used
across environments that may have different identifiers. Suitable values for
this environment variable can be generated [in this web
page](https://packsquash.aylas.org/tools/system-id-generator), which is provided
by the PackSquash authors as a convenient way to generate them.

Finally, it should be noted that PackSquash will not attempt to get a system ID
unless it is necessary. So, if the storage of Squash Times is disabled, no
system ID will ever be computed and used for anything.

### Application salt

The application salt is a hardcoded 256-bit number generated at build time using
a CSPRNG. Because it is hardcoded, it is guaranteed to remain constant for every
execution of a given PackSquash executable. Different builds may have a
different application salt.

### Filesystem-provided file modification timestamps

Explaining the details on how filesystems store file modification timestamps is
out of scope for this document.

It suffices to remark that different filesystems may store this time data with
varying precision, accuracy, and reliability. Some filesystems may not provide
suitable modification timestamps at all. In their default configuration, the
most common non-distributed filesystems (NTFS, ext2, ext3, ext4, AFS, HFS+,
etc.) store suitable modification timestamps, but it is advised to double-check
that this holds true if you use less common filesystems or configurations.

Another interesting remark is that PackSquash assumes that files with a
modification time before their Squash Time were not changed in any way. This is
usually a reasonable assumption, but it may not hold true if file modification
timestamps are modified.

### Squash Times

Squash Times are the count of half-seconds since Monday, 22 December 2014
0:00:00 (UTC), excluding leap seconds, until a file was optimized by PackSquash.

Mathematically, a Squash Time is defined as follows. Let $U$ be the [POSIX
time](https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16)
that represents the instant when a file was optimized and is to be converted to
a Squash Time, with millisecond precision (but not necessarily accuracy). Let
$E$ be the POSIX time value with millisecond precision for the Squash Time
epoch, which is a constant whose value is $1419206400000$. The Squash Time $T$
is:

$$T = \frac{U - E}{500}$$

Squash Times are always stored as unsigned 32-bit integers. However, the most
significant bit is a stick parity bit that is always set to zero, whose purpose
is to detect Squash Times that were tampered with or decrypted with unintended
cryptographic parameters. So the highest date that can be represented is
Wednesday, 30 December 2048 13:37:03 (UTC). The reasons for using Squash Times
over standard second precision POSIX times are:

- They avert [the year 2038
  problem](https://en.wikipedia.org/wiki/Year_2038_problem) while being twice as
  precise, thanks to the choice of a much more recent epoch.
- Because they always need 4 bytes to be stored, they fit in the 4 bytes that
  the ZIP file records dedicate to the modification time and date fields, where
  they are stored after encryption.

### Format-preserving encryption

Explaining the details of FPE is out of scope for this document. However, it is
interesting to note that the CRC32 of a pack file is used as a tweak for the FF1
mode of operation of the AES-256 block cipher, so two files with different CRC32
will have different ciphertexts for the same plaintext.
