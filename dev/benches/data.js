window.BENCHMARK_DATA = {
  "lastUpdate": 1655124347783,
  "repoUrl": "https://github.com/ComunidadAylas/PackSquash",
  "entries": {
    "PackSquash library quick benchmarks": [
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "364784dca3d7235bc4cfd22e3204b2ba92359132",
          "message": "chore(deps): bump actions/upload-artifact from 2 to 3 (#95)\n\nBumps [actions/upload-artifact](https://github.com/actions/upload-artifact) from 2 to 3.\r\n- [Release notes](https://github.com/actions/upload-artifact/releases)\r\n- [Commits](https://github.com/actions/upload-artifact/compare/v2...v3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/upload-artifact\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-11T10:04:29+02:00",
          "tree_id": "5501f02b39e1c9a1a9e9ae92a286017b0df4eb72",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/364784dca3d7235bc4cfd22e3204b2ba92359132"
        },
        "date": 1649666350972,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14632623,
            "range": "± 781773",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 804648941,
            "range": "± 16939856",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13372598681,
            "range": "± 156806944",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2683923517,
            "range": "± 40949778",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "bafe0a9c3e1eb4706fb82a98aedd650c4eacd792",
          "message": "tweak(png_file): improve error message for PNG files with trailing bytes\n\nThe PNG validation and chunk stripping code we introduced for v0.3.1 had\nthe technically nice side effect of being stricter with trailing bytes\nat the end of files, as previously the decoders we used just ignored\nthem, even if the PNG standard explicitly says that such PNG files are\nnon-conforming. I expected such an error condition to be very rare, so I\ndidn't add a specific error message for it.\n\nHowever, @sya-ri stumbled upon a PNG file that had trailing bytes, and\nwas puzzled about it, with reason, as the error message pointed out that\nthe PNG file is \"too small\". To address that situation, update the wiki\nand tweak the error message for future releases.",
          "timestamp": "2022-04-12T22:13:52+02:00",
          "tree_id": "ab388b26bc4d3d3f75cfc7a7684ce390278290f8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bafe0a9c3e1eb4706fb82a98aedd650c4eacd792"
        },
        "date": 1649796822592,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14491745,
            "range": "± 984818",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 760119122,
            "range": "± 21361903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12946192574,
            "range": "± 214389182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2646146379,
            "range": "± 64109448",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "5381ceffabd1642e93bb911f21f4d5f80636a114",
          "message": "ci(appimage): bump appimage-builder version used by the action in fork\n\nThe appimage-builder action was using an ancient appimage-builder\nversion, 0.8.2, which is lacking some important fixes for us. Let's\nupdate it to something much more modern ourselves.",
          "timestamp": "2022-04-14T13:01:42+02:00",
          "tree_id": "067d86075a70758ffec6caf7a760d752a8ae8f97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5381ceffabd1642e93bb911f21f4d5f80636a114"
        },
        "date": 1649936568385,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15422109,
            "range": "± 800412",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 823070658,
            "range": "± 9358165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13880916604,
            "range": "± 82000403",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2794591778,
            "range": "± 20535365",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "a4e086761288dfdcf8b9cc3ea192441026c98487",
          "message": "dist: bump back to v0.3.1 for re-release with fixed appimage-builder\n\nAfter releasing v0.3.1 two unexpected and somewhat critical errors were\nreported by users:\n\n- The checks we introduced for trailing PNG bytes were more frequent\n  than anticipated, but the error message shown for them was bad and\n  confusing for users. This warranted improving the error messages at\n  least.\n- The new Linux AppImages were effectively broken due to the usage of a\n  very old appimage-builder version in CI, v0.8.2, while a newer\n  appimage-builder version of v0.9.2 was used for testing by developers\n  in their computers. That older appimage-builder version, pulled by its\n  corresponding workflow action, did not handle GStreamer plugins\n  properly, but support has improved a lot ever since, especially since\n  v1.0.0-alpha.1. The alpha version even contain some nice changes that\n  generate the GStreamer plugin registry at build time, which speeds up\n  execution and avoids some ugly, non-hideable warnings about loading\n  plugins that are not bundled in the AppImage and not necessary.\n\nAs we've addressed these two issues, prepare the repository so that the\nnext workflow run will generate artifacts suitable for re-releasing\nv0.3.1. These release artifacts will silently replace the current v0.3.1\nrelease artifacts. For traceability, PackSquash will show a slightly\ndifferent version when run.",
          "timestamp": "2022-04-14T15:32:07+02:00",
          "tree_id": "ec1b474e5ef59fa63ef5c7d8b55162f1cdca9c91",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a4e086761288dfdcf8b9cc3ea192441026c98487"
        },
        "date": 1649951603715,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14430411,
            "range": "± 1073194",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 799067299,
            "range": "± 20518533",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13410498205,
            "range": "± 393865335",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2700485986,
            "range": "± 110030820",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "a4e086761288dfdcf8b9cc3ea192441026c98487",
          "message": "dist: bump back to v0.3.1 for re-release with fixed appimage-builder\n\nAfter releasing v0.3.1 two unexpected and somewhat critical errors were\nreported by users:\n\n- The checks we introduced for trailing PNG bytes were more frequent\n  than anticipated, but the error message shown for them was bad and\n  confusing for users. This warranted improving the error messages at\n  least.\n- The new Linux AppImages were effectively broken due to the usage of a\n  very old appimage-builder version in CI, v0.8.2, while a newer\n  appimage-builder version of v0.9.2 was used for testing by developers\n  in their computers. That older appimage-builder version, pulled by its\n  corresponding workflow action, did not handle GStreamer plugins\n  properly, but support has improved a lot ever since, especially since\n  v1.0.0-alpha.1. The alpha version even contain some nice changes that\n  generate the GStreamer plugin registry at build time, which speeds up\n  execution and avoids some ugly, non-hideable warnings about loading\n  plugins that are not bundled in the AppImage and not necessary.\n\nAs we've addressed these two issues, prepare the repository so that the\nnext workflow run will generate artifacts suitable for re-releasing\nv0.3.1. These release artifacts will silently replace the current v0.3.1\nrelease artifacts. For traceability, PackSquash will show a slightly\ndifferent version when run.",
          "timestamp": "2022-04-14T15:32:07+02:00",
          "tree_id": "ec1b474e5ef59fa63ef5c7d8b55162f1cdca9c91",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a4e086761288dfdcf8b9cc3ea192441026c98487"
        },
        "date": 1649956724443,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13273902,
            "range": "± 806734",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 651588201,
            "range": "± 18838347",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11579921713,
            "range": "± 626931260",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2468342873,
            "range": "± 135362701",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "349eaf031897ad327060eb840f308f16d801bcb5",
          "message": "ci(linux): ditch build-appimage action, use appimage-builder directly\n\nThe action was terribly obsolete, to begin with. We updated it to a more\nrecent version, and it worked, but then we wanted to get rid of some\nbenign GStreamer warnings, and for this the latest appimage-builder\nversion needs to run gst-launch-1.0 during build time. So we tried to\nput the package that contains that command in the Docker image too, but\nthen no plugins could be loaded in that environment for some reason.\nUsing appimage-builder locally, with gst-launch-1.0, worked fine, at\nleast in a non cross-compilation situation.\n\nWe maybe could spend 3 days debugging this and waiting for the Docker\nimages to upload at a terribly slow speed that reminds me of the dial-up\ndays, but by the looks of it, not much people cares about the Docker\ncontainer anyway, so it's fate is to become obsolete again. Let's\nsidestep all of that by not using the action.",
          "timestamp": "2022-04-14T21:55:24+02:00",
          "tree_id": "deaca031e4c84965051b9b88e8237b1d3a2afed7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/349eaf031897ad327060eb840f308f16d801bcb5"
        },
        "date": 1649968726154,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17250590,
            "range": "± 691395",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 821778515,
            "range": "± 18371560",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15129503870,
            "range": "± 284792458",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2893544360,
            "range": "± 76262096",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b3f5df32340aba2e28a2d116ac483cff22e08b14",
          "message": "chore(deps): bump toml from 0.5.8 to 0.5.9 (#98)\n\nBumps [toml](https://github.com/alexcrichton/toml-rs) from 0.5.8 to 0.5.9.\r\n- [Release notes](https://github.com/alexcrichton/toml-rs/releases)\r\n- [Commits](https://github.com/alexcrichton/toml-rs/compare/0.5.8...0.5.9)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: toml\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-18T10:58:30+02:00",
          "tree_id": "95846f53b5b0288c13a27ea9fc2a0caebeaf3a2c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b3f5df32340aba2e28a2d116ac483cff22e08b14"
        },
        "date": 1650274320258,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13962140,
            "range": "± 704537",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 777459789,
            "range": "± 20434507",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13341389873,
            "range": "± 270704483",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2662303930,
            "range": "± 52906102",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5fe7054cd1687719560804f2164a9a545e105c96",
          "message": "ci(linux/aarch64): install missing package dependency",
          "timestamp": "2022-04-18T10:54:25+02:00",
          "tree_id": "14aa64e56c445d6e68612c0224462aa0ff81ea3a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5fe7054cd1687719560804f2164a9a545e105c96"
        },
        "date": 1650275214815,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12462296,
            "range": "± 307585",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 690250069,
            "range": "± 5376980",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12885206326,
            "range": "± 223230235",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2456364485,
            "range": "± 81177265",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b44439f4f6bde979dbe5c7be731cd1d65253d4a7",
          "message": "ci(linux/aarch64): fix typo",
          "timestamp": "2022-04-18T12:28:32+02:00",
          "tree_id": "0f05d8d308fb2b4bab569b5f3b748d5fe8a1b00a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b44439f4f6bde979dbe5c7be731cd1d65253d4a7"
        },
        "date": 1650279172036,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12539107,
            "range": "± 199971",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 690414979,
            "range": "± 4940540",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12748473254,
            "range": "± 80237151",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2460240148,
            "range": "± 50501284",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "d2da6012783b270609882060e051e7abb5324e3d",
          "message": "chore(Cargo.toml): remove Cargo option stabilized in 1.59",
          "timestamp": "2022-04-24T20:06:11+02:00",
          "tree_id": "f2c3cb82f050e6c54be95435a769d8924df524a9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d2da6012783b270609882060e051e7abb5324e3d"
        },
        "date": 1650825631732,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13510193,
            "range": "± 815942",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 693790336,
            "range": "± 20523164",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13798561759,
            "range": "± 54345426",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2490247733,
            "range": "± 1955195960",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "id": "d2da6012783b270609882060e051e7abb5324e3d",
          "message": "chore(Cargo.toml): remove Cargo option stabilized in 1.59",
          "timestamp": "2022-04-24T18:05:39Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d2da6012783b270609882060e051e7abb5324e3d"
        },
        "date": 1650850439749,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14499971,
            "range": "± 206132",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 702454231,
            "range": "± 5861142",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14014202238,
            "range": "± 85166632",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2509806831,
            "range": "± 14860362",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2d3215edc81ed15a3dd649225179281408f7b192",
          "message": "chore(deps): bump uuid from 0.8.2 to 1.0.0 (#99)\n\nBumps [uuid](https://github.com/uuid-rs/uuid) from 0.8.2 to 1.0.0.\r\n- [Release notes](https://github.com/uuid-rs/uuid/releases)\r\n- [Commits](https://github.com/uuid-rs/uuid/compare/0.8.2...1.0.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uuid\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-25T10:29:42+02:00",
          "tree_id": "2fd1239e17cf34441722e87cf5ba89206ab5cb00",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2d3215edc81ed15a3dd649225179281408f7b192"
        },
        "date": 1650878158437,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16616622,
            "range": "± 940551",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 774891607,
            "range": "± 35689491",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15125785947,
            "range": "± 617750340",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2724956311,
            "range": "± 71623062",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "da0230ff84cf1bfad79bf91c067a66ab6eca5b5d",
          "message": "ci(issue templates): add link to the Discord server in the template chooser\n\nAlso disable creating empty issues.",
          "timestamp": "2022-04-28T13:11:45Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/da0230ff84cf1bfad79bf91c067a66ab6eca5b5d"
        },
        "date": 1651455790659,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16353934,
            "range": "± 1088856",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 911527425,
            "range": "± 24069030",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15032638259,
            "range": "± 387908368",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3498013631,
            "range": "± 184932198",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d1aebda4f83fe5fd885a07101428dbed90f026e",
          "message": "chore(deps): bump tokio from 1.17.0 to 1.18.0 (#102)\n\nBumps [tokio](https://github.com/tokio-rs/tokio) from 1.17.0 to 1.18.0.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.17.0...tokio-1.18.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-02T10:52:56+02:00",
          "tree_id": "8d3681a7cf8ce206e070e9a880fefbdd5c536f36",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0d1aebda4f83fe5fd885a07101428dbed90f026e"
        },
        "date": 1651483162980,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11744072,
            "range": "± 362694",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 728877458,
            "range": "± 9553411",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11937917957,
            "range": "± 790177610",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2424382609,
            "range": "± 98503994",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b3cc3148484bec11528c92330ac16b1ab7c145d1",
          "message": "chore(deps): bump serde_json from 1.0.79 to 1.0.80 (#106)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.79 to 1.0.80.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.79...v1.0.80)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-02T10:55:16+02:00",
          "tree_id": "e47775cd664f69ca1694d3bf18936cfaa7005f1f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b3cc3148484bec11528c92330ac16b1ab7c145d1"
        },
        "date": 1651483522383,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15813902,
            "range": "± 1191829",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 888895706,
            "range": "± 22773331",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11695981113,
            "range": "± 205187593",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3108644214,
            "range": "± 83096794",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "63c01bf3d310dcbb73fba086b22b2ba96cf44cac",
          "message": "chore(deps): bump gstreamer from 0.18.7 to 0.18.8 (#103)\n\nBumps gstreamer from 0.18.7 to 0.18.8.\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: gstreamer\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-04T01:17:06+02:00",
          "tree_id": "b77513d42f515a199409dedf1aba116e11576299",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63c01bf3d310dcbb73fba086b22b2ba96cf44cac"
        },
        "date": 1651621998909,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14207567,
            "range": "± 989821",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 742333428,
            "range": "± 16353670",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10528425967,
            "range": "± 679654353",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2594400619,
            "range": "± 101883685",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c45ed289f32a181849e5b8697bfa57a235a8f0d2",
          "message": "chore(deps): bump thiserror from 1.0.30 to 1.0.31 (#107)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.30 to 1.0.31.\r\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\r\n- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.30...1.0.31)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: thiserror\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-04T01:18:08+02:00",
          "tree_id": "25b371e2f500cd15b5b03816bd7facbbdc462668",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c45ed289f32a181849e5b8697bfa57a235a8f0d2"
        },
        "date": 1651622075736,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17159264,
            "range": "± 1016989",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 840322676,
            "range": "± 26789197",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12988464157,
            "range": "± 263698024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3261666196,
            "range": "± 141530411",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "72daa7e88a6a5f5abf47c4ec378b0c0919797353",
          "message": "chore(deps): bump serde from 1.0.136 to 1.0.137 (#104)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.136 to 1.0.137.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.136...v1.0.137)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-04T21:53:00+02:00",
          "tree_id": "5b51e1551c4b7c326cfa76f5f61b1b992f1bce0b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/72daa7e88a6a5f5abf47c4ec378b0c0919797353"
        },
        "date": 1651696364796,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10893255,
            "range": "± 156525",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 703832471,
            "range": "± 15989881",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10216061266,
            "range": "± 44078259",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2640011619,
            "range": "± 6239947",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "51b017809d4460f480bad6aa52d3b40f0bd8ae7f",
          "message": "chore(deps): bump sysinfo from 0.23.10 to 0.23.11 (#105)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.10 to 0.23.11.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-06T20:32:33+02:00",
          "tree_id": "6aa96d3dfb17c8e79ce29cfddab53c6a9a071520",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/51b017809d4460f480bad6aa52d3b40f0bd8ae7f"
        },
        "date": 1651864046821,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11062349,
            "range": "± 98405",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 706380622,
            "range": "± 6545324",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10058582593,
            "range": "± 55775396",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2639423421,
            "range": "± 7146123",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "51b017809d4460f480bad6aa52d3b40f0bd8ae7f",
          "message": "chore(deps): bump sysinfo from 0.23.10 to 0.23.11 (#105)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.10 to 0.23.11.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-06T18:32:33Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/51b017809d4460f480bad6aa52d3b40f0bd8ae7f"
        },
        "date": 1652060610822,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16935046,
            "range": "± 813821",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 883204596,
            "range": "± 14763660",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14160398780,
            "range": "± 214813124",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3131078522,
            "range": "± 55344452",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ceefa8d882a636fea8e8043df699c95e2897be3f",
          "message": "chore(deps): bump log from 0.4.16 to 0.4.17 (#110)\n\nBumps [log](https://github.com/rust-lang/log) from 0.4.16 to 0.4.17.\r\n- [Release notes](https://github.com/rust-lang/log/releases)\r\n- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/log/commits/0.4.17)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: log\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-09T22:15:32+02:00",
          "tree_id": "5f45aaa436e2b04035acd223c516c529cafe8c4f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ceefa8d882a636fea8e8043df699c95e2897be3f"
        },
        "date": 1652129700953,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16728827,
            "range": "± 821019",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 875337180,
            "range": "± 9794595",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14280046308,
            "range": "± 120630520",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3189173071,
            "range": "± 27362559",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "316e3a2cf424aac05e46a3904872b2bf09f13255",
          "message": "chore(deps): bump serde_json from 1.0.80 to 1.0.81 (#108)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.80 to 1.0.81.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.80...v1.0.81)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-09T22:16:56+02:00",
          "tree_id": "0edf5d75a5612cfe6675c4697ce0df3f64d7d178",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/316e3a2cf424aac05e46a3904872b2bf09f13255"
        },
        "date": 1652129704359,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14682483,
            "range": "± 182034",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 749122463,
            "range": "± 7944578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11897499768,
            "range": "± 50016696",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2679755690,
            "range": "± 13082948",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "93ae23fa06ef0d65fd1aae19b968eb244b9590b9",
          "message": "chore(deps): bump tokio from 1.18.0 to 1.18.2 (#109)\n\nBumps [tokio](https://github.com/tokio-rs/tokio) from 1.18.0 to 1.18.2.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.18.0...tokio-1.18.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-09T22:18:15+02:00",
          "tree_id": "e3295e9114735899c4c0d305a35107ac31cdc145",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/93ae23fa06ef0d65fd1aae19b968eb244b9590b9"
        },
        "date": 1652129839168,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17930655,
            "range": "± 916792",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 900917404,
            "range": "± 7251908",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14301245011,
            "range": "± 58979225",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3188876524,
            "range": "± 110818537",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "9f81e31d190c4adf764701a9929842f035f1124e",
          "message": "chore: rename `packsquash-cli` package. Stop using unstable Cargo feature\n\nBy defining a custom binary with the desired name instead of modifying\nthe filename of the default binary, which is the same as the name of the\npackage, we can achieve the same result as before, but in a more\nstraightforward way with less configuration.\n\nWhile at it, I've discovered that some Rust RFC recommends for words in\ncrate names to be separated by underscores instead of dashes. This\nconvention is far from being universally observed, and some prominent\ncrates use dashes, but we want to do things right, so let's use\nunderscores.",
          "timestamp": "2022-05-09T22:27:12+02:00",
          "tree_id": "265fc5bae7e09f2eeb5433c2fd1c16ee31346faf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9f81e31d190c4adf764701a9929842f035f1124e"
        },
        "date": 1652131134972,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17840692,
            "range": "± 849822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 902262656,
            "range": "± 15259720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14295614009,
            "range": "± 50070541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3237618131,
            "range": "± 21946390",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "fb5012ad72dc36b97b730f65072969a4ab79409e",
          "message": "chore(benches): fix `unused-macro-rules` Clippy lint",
          "timestamp": "2022-05-14T23:08:36+02:00",
          "tree_id": "d2802162b907b8599d8dc4aa4d7fcd99e0133a10",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fb5012ad72dc36b97b730f65072969a4ab79409e"
        },
        "date": 1652565001151,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17715076,
            "range": "± 800298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1045565968,
            "range": "± 9471832",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 17587589430,
            "range": "± 162044748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2710830802,
            "range": "± 57347161",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "id": "fb5012ad72dc36b97b730f65072969a4ab79409e",
          "message": "chore(benches): fix `unused-macro-rules` Clippy lint",
          "timestamp": "2022-05-14T21:08:36Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fb5012ad72dc36b97b730f65072969a4ab79409e"
        },
        "date": 1652665670330,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16261891,
            "range": "± 975294",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 982895331,
            "range": "± 17048556",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15348234691,
            "range": "± 174722910",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2548526691,
            "range": "± 57945344",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "36238d2413971f9b69bab11898674c1c3a683c30",
          "message": "chore(deps): bump sysinfo from 0.23.11 to 0.23.12 (#112)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.11 to 0.23.12.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T16:41:19+02:00",
          "tree_id": "eb80dbaa79b5fe5756ba62339dd063a25c8175ef",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36238d2413971f9b69bab11898674c1c3a683c30"
        },
        "date": 1652713793311,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17611213,
            "range": "± 1152409",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1004444381,
            "range": "± 6554269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 17288521114,
            "range": "± 68248175",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2603072640,
            "range": "± 98308862",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cf69f68da1176e46f556134f74d5d7885b380cab",
          "message": "chore(deps): bump tokio-util from 0.7.1 to 0.7.2 (#113)\n\nBumps [tokio-util](https://github.com/tokio-rs/tokio) from 0.7.1 to 0.7.2.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-util-0.7.1...tokio-util-0.7.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-util\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T16:40:59+02:00",
          "tree_id": "0def437895572fdc3a87d10e506bf5b6974d6ab3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cf69f68da1176e46f556134f74d5d7885b380cab"
        },
        "date": 1652714714218,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12481439,
            "range": "± 717241",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 827033288,
            "range": "± 8599020",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15048915207,
            "range": "± 49419269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2170130816,
            "range": "± 31029527",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "36238d2413971f9b69bab11898674c1c3a683c30",
          "message": "chore(deps): bump sysinfo from 0.23.11 to 0.23.12 (#112)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.11 to 0.23.12.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T14:41:19Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36238d2413971f9b69bab11898674c1c3a683c30"
        },
        "date": 1653269868832,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14471446,
            "range": "± 328183",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 844772431,
            "range": "± 4804601",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11881421318,
            "range": "± 40636371",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2301563746,
            "range": "± 50083010",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "986f98346b8b5e13065aa962e17a3f55d6cec8e8",
          "message": "chore(deps): bump sysinfo from 0.23.12 to 0.23.13 (#115)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.12 to 0.23.13.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T10:55:57+02:00",
          "tree_id": "8b3ae80acebdbcb5b3ffd441aef8ff18c66212c7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/986f98346b8b5e13065aa962e17a3f55d6cec8e8"
        },
        "date": 1653297759469,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10940065,
            "range": "± 388703",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 798749393,
            "range": "± 3041573",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10286961629,
            "range": "± 232733808",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2263753173,
            "range": "± 37512297",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0f3d5cbf5daba4b511d8faff56c5e527d76a4198",
          "message": "chore(deps): bump regex from 1.5.5 to 1.5.6 (#116)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.5.5 to 1.5.6.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.5.5...1.5.6)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T10:55:05+02:00",
          "tree_id": "fbf0b902cd6fa7df36aee6a218ea5c34fd5c51dd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0f3d5cbf5daba4b511d8faff56c5e527d76a4198"
        },
        "date": 1653297800266,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15217361,
            "range": "± 800856",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 968973538,
            "range": "± 6640036",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11799692135,
            "range": "± 76415206",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2434862018,
            "range": "± 41204927",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "fccf9271176d14f298d29ca52b50f3e691a9420f",
          "message": "chore(repo): add pull request template",
          "timestamp": "2022-05-24T18:06:47Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fccf9271176d14f298d29ca52b50f3e691a9420f"
        },
        "date": 1653875133463,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14455898,
            "range": "± 306056",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 843099948,
            "range": "± 3610205",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11813732314,
            "range": "± 49724981",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2304541011,
            "range": "± 27639808",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8fd914740d637a6d70630529d3cb1c067056dc74",
          "message": "chore(deps): bump indexmap from 1.8.1 to 1.8.2 (#117)\n\nBumps [indexmap](https://github.com/bluss/indexmap) from 1.8.1 to 1.8.2.\r\n- [Release notes](https://github.com/bluss/indexmap/releases)\r\n- [Changelog](https://github.com/bluss/indexmap/blob/1.8.2/RELEASES.rst)\r\n- [Commits](https://github.com/bluss/indexmap/compare/1.8.1...1.8.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: indexmap\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-30T10:34:32+02:00",
          "tree_id": "abac87e5022076ebb1397c48f4d177bb3ae568ec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8fd914740d637a6d70630529d3cb1c067056dc74"
        },
        "date": 1653901496611,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14805255,
            "range": "± 802988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 911767694,
            "range": "± 17280590",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11204192028,
            "range": "± 127996581",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2389575506,
            "range": "± 97561393",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9120ccadd5cfd7351aee9ef985a9206314fec5f0",
          "message": "chore(deps): bump uuid from 1.0.0 to 1.1.0 (#118)\n\nBumps [uuid](https://github.com/uuid-rs/uuid) from 1.0.0 to 1.1.0.\r\n- [Release notes](https://github.com/uuid-rs/uuid/releases)\r\n- [Commits](https://github.com/uuid-rs/uuid/compare/1.0.0...1.1.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uuid\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-30T10:35:37+02:00",
          "tree_id": "68842baf953a799fc6134a2a7e4d7f7f29129b51",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9120ccadd5cfd7351aee9ef985a9206314fec5f0"
        },
        "date": 1653902418976,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12521403,
            "range": "± 973027",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 760236228,
            "range": "± 30533013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10003337162,
            "range": "± 664960343",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2057313084,
            "range": "± 137271511",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "25ae2b83d93e64feb2f32880daf10a9e263da724",
          "message": "chore(ISSUE_TEMPLATE/feature_request): fix typo",
          "timestamp": "2022-05-30T14:36:45Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/25ae2b83d93e64feb2f32880daf10a9e263da724"
        },
        "date": 1654479911531,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15163860,
            "range": "± 372715",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 981670515,
            "range": "± 11747361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16876762899,
            "range": "± 129012265",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2588930372,
            "range": "± 34624492",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c0bd5e0b4fb01399b8ba7bfc7a506d2af1a3463d",
          "message": "chore(deps): bump uuid from 1.1.0 to 1.1.1 (#124)\n\nBumps [uuid](https://github.com/uuid-rs/uuid) from 1.1.0 to 1.1.1.\r\n- [Release notes](https://github.com/uuid-rs/uuid/releases)\r\n- [Commits](https://github.com/uuid-rs/uuid/compare/1.1.0...1.1.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uuid\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-06T12:21:25+02:00",
          "tree_id": "a61be1b1399ec40d06f7f124c25ecec7cf26dd20",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c0bd5e0b4fb01399b8ba7bfc7a506d2af1a3463d"
        },
        "date": 1654512551480,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13328465,
            "range": "± 812251",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 806033084,
            "range": "± 40168877",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13805909009,
            "range": "± 770683126",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2205190681,
            "range": "± 116060629",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9365909009e6fd59913dc873bc3252342c956d9f",
          "message": "chore(deps): bump sysinfo from 0.23.13 to 0.24.1 (#123)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.13 to 0.24.1.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-06T12:22:44+02:00",
          "tree_id": "e660cdf5b8ba24e336e5abfada25b07e7ab43690",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9365909009e6fd59913dc873bc3252342c956d9f"
        },
        "date": 1654512575105,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13304624,
            "range": "± 620040",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 873010192,
            "range": "± 14050270",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13565321793,
            "range": "± 117685189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2236418669,
            "range": "± 24877859",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad0be826ed42dfa93a105c78359838c463455e0c",
          "message": "chore(deps): bump tokio from 1.18.2 to 1.19.1 (#122)\n\nBumps [tokio](https://github.com/tokio-rs/tokio) from 1.18.2 to 1.19.1.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.18.2...tokio-1.19.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T10:56:13+02:00",
          "tree_id": "7a2bfad7b0e65db724aebb462e29fc40276b7927",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ad0be826ed42dfa93a105c78359838c463455e0c"
        },
        "date": 1654594563487,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17074707,
            "range": "± 556218",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 995557101,
            "range": "± 7442422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16615426324,
            "range": "± 217933305",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2598312540,
            "range": "± 39601814",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9c6fc71fe22f88f004655314498c08acddce9fda",
          "message": "chore(deps): bump tokio-util from 0.7.2 to 0.7.3 (#120)\n\nBumps [tokio-util](https://github.com/tokio-rs/tokio) from 0.7.2 to 0.7.3.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-util-0.7.2...tokio-util-0.7.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-util\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T12:23:01+02:00",
          "tree_id": "41cef1225fdfe9cbc94c8dfede10d86753101d0a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9c6fc71fe22f88f004655314498c08acddce9fda"
        },
        "date": 1654599120142,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17544001,
            "range": "± 822853",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1006453858,
            "range": "± 7487041",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16905449952,
            "range": "± 139663110",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671668402,
            "range": "± 27339099",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42495828bb9197e9a9e26f406f7b1f129bc043e0",
          "message": "chore(deps): bump tokio-stream from 0.1.8 to 0.1.9 (#121)\n\nBumps [tokio-stream](https://github.com/tokio-rs/tokio) from 0.1.8 to 0.1.9.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-stream-0.1.8...tokio-stream-0.1.9)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-stream\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T12:29:12+02:00",
          "tree_id": "feb5c21763f6cfcf1d5c295c3a489cf5184ed711",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/42495828bb9197e9a9e26f406f7b1f129bc043e0"
        },
        "date": 1654599388894,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14113513,
            "range": "± 924629",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 841171237,
            "range": "± 3242152",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14165729159,
            "range": "± 77986080",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2190069551,
            "range": "± 54801905",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "f68079c29bf4da75272c7ab7ff4b696c0c542491",
          "message": "chore(ci): refactor Docker workflow job",
          "timestamp": "2022-06-08T19:08:20+02:00",
          "tree_id": "be5ff8313066e878cb1f06ec41401f2b39f30087",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f68079c29bf4da75272c7ab7ff4b696c0c542491"
        },
        "date": 1654710576813,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14720317,
            "range": "± 707298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 958389917,
            "range": "± 10121423",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15179464379,
            "range": "± 61556939",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2450483515,
            "range": "± 78103758",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "46e7f522f37dd2cedf11a7811520cf7b06bf20a3",
          "message": "chore(ci): add Dockerfile to included push path for build workflow",
          "timestamp": "2022-06-08T19:18:25+02:00",
          "tree_id": "2f400a6431014f4fc19191213edd25f86f8d69c4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46e7f522f37dd2cedf11a7811520cf7b06bf20a3"
        },
        "date": 1654710798458,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11253379,
            "range": "± 259338",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 793473722,
            "range": "± 5499109",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12799262187,
            "range": "± 59280554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2267196023,
            "range": "± 51756552",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "a0f0c195d132a1d60f0c23c0f7bde8abb6117a9e",
          "message": "chore(ci/docker): rename job and fix typo",
          "timestamp": "2022-06-08T19:59:33+02:00",
          "tree_id": "773c465e930e100e1e48f5a91bf1a88c9404ce38",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a0f0c195d132a1d60f0c23c0f7bde8abb6117a9e"
        },
        "date": 1654712849749,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12570106,
            "range": "± 3453177",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 754849043,
            "range": "± 2697922",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13264407805,
            "range": "± 65151037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2053127914,
            "range": "± 47030235",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "21dd13854b6df0699dc078c59ce78fa247536303",
          "message": "chore(ci/docker): fix GitHub expression not being evaluated to YAML bool",
          "timestamp": "2022-06-08T20:31:31+02:00",
          "tree_id": "8615690f17b4ed371bf8e9bab456a1fb86ba7f97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/21dd13854b6df0699dc078c59ce78fa247536303"
        },
        "date": 1654714953333,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17557390,
            "range": "± 1026165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1024867577,
            "range": "± 13249931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 18053649048,
            "range": "± 103167904",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2838314884,
            "range": "± 119759978",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "b4ac04d1b1d7667bfa4a8618d7652d9c2414d4c9",
          "message": "chore(deps): update dependencies",
          "timestamp": "2022-06-08T22:36:06+02:00",
          "tree_id": "a723a3937cd2826c2cace6c61952b68895c1c9e5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b4ac04d1b1d7667bfa4a8618d7652d9c2414d4c9"
        },
        "date": 1654722371946,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15688201,
            "range": "± 1141729",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 991494167,
            "range": "± 22183057",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16740331682,
            "range": "± 178893339",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2648813890,
            "range": "± 63952792",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "7cc3d3675fbc6ea5fbe708d7557855aba614c3db",
          "message": "chore(ci/docker): tweak comment and add proper description label",
          "timestamp": "2022-06-08T22:47:11+02:00",
          "tree_id": "3e598535644ff1708593029f66e16ca436b82c6f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7cc3d3675fbc6ea5fbe708d7557855aba614c3db"
        },
        "date": 1654723052153,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15909948,
            "range": "± 758191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 997827211,
            "range": "± 13104958",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15469189839,
            "range": "± 203309023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671755276,
            "range": "± 78327309",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "404d88bacb2d21ecc84a0c7d032ff49ff033bb97",
          "message": "chore(static_analysis): remove withdrawn advisory from deny.toml",
          "timestamp": "2022-06-08T22:49:03+02:00",
          "tree_id": "4eb133ae4652c70db8a8198bc1dcb14e17b6fe96",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/404d88bacb2d21ecc84a0c7d032ff49ff033bb97"
        },
        "date": 1654723932190,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16810987,
            "range": "± 834016",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1039286659,
            "range": "± 32973517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16323709976,
            "range": "± 314867821",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2636259248,
            "range": "± 81254762",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "46f5772d58963dcfd69ddab3de8213bd3295a942",
          "message": "fix(pack_file/asset_type): legacy font .bin files can be anywhere\n\nThe Minecraft wiki incorrectly pointed out that .bin files belong to\nassets/<namespace>/font, but the game interprets the resource location\nin an absolute context, so valid resource locations for those files can\npoint anywhere in the pack. Indeed, a PackSquash user reported dealing\nwith a pack that had a .bin file outside of the usual folder, which\nPackSquash broke due to me coding a too restrictive glob pattern based\non the wiki misinformation.\n\nMake the .bin glob pattern more permissive to fix the situation.",
          "timestamp": "2022-06-08T23:13:20+02:00",
          "tree_id": "14a5b1d3fc161ec5ea77fa354ad8f65e5c1934e9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46f5772d58963dcfd69ddab3de8213bd3295a942"
        },
        "date": 1654725717322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13000168,
            "range": "± 604687",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 826767095,
            "range": "± 1712070",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13995786116,
            "range": "± 49827923",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2214103027,
            "range": "± 60926880",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "id": "46f5772d58963dcfd69ddab3de8213bd3295a942",
          "message": "fix(pack_file/asset_type): legacy font .bin files can be anywhere\n\nThe Minecraft wiki incorrectly pointed out that .bin files belong to\nassets/<namespace>/font, but the game interprets the resource location\nin an absolute context, so valid resource locations for those files can\npoint anywhere in the pack. Indeed, a PackSquash user reported dealing\nwith a pack that had a .bin file outside of the usual folder, which\nPackSquash broke due to me coding a too restrictive glob pattern based\non the wiki misinformation.\n\nMake the .bin glob pattern more permissive to fix the situation.",
          "timestamp": "2022-06-08T21:13:20Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46f5772d58963dcfd69ddab3de8213bd3295a942"
        },
        "date": 1655085841218,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14830139,
            "range": "± 993401",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 978210971,
            "range": "± 9348325",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15308909972,
            "range": "± 82251273",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2491328574,
            "range": "± 68808633",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "AlexTMjugador@users.noreply.github.com",
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "e27ebb38979a0aa82ca5eae83c13e54f1cef59ba",
          "message": "chore(deps): update globset, sysinfo and uuid to their latest versions",
          "timestamp": "2022-06-13T14:00:09+02:00",
          "tree_id": "a6cdff0f19948952e41a3ec7b6d06ab18432cebf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e27ebb38979a0aa82ca5eae83c13e54f1cef59ba"
        },
        "date": 1655124347134,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10997347,
            "range": "± 317731",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 790386573,
            "range": "± 1901595",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12763358946,
            "range": "± 48940367",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2255457640,
            "range": "± 74777213",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}