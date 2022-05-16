window.BENCHMARK_DATA = {
  "lastUpdate": 1652665670863,
  "repoUrl": "https://github.com/ComunidadAylas/PackSquash",
  "entries": {
    "PackSquash library quick benchmarks": [
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
          "id": "11eb1a02be6372360bec1510d8de5b39ab633b78",
          "message": "ci(experimental/x32): target Debian testing\n\nThe unofficial x32 port of Debian does not follow the usual release\ncadence and it usually breaks. Let's see if moving to this other version\nworks.",
          "timestamp": "2022-04-02T15:07:05+02:00",
          "tree_id": "0ee5f528d86a999e39ca2f77e762ba0fa2967d5c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/11eb1a02be6372360bec1510d8de5b39ab633b78"
        },
        "date": 1648906676475,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13129511,
            "range": "± 236815",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 938895395,
            "range": "± 2258387",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10540436476,
            "range": "± 196023102",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1377559804,
            "range": "± 10674290",
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
          "id": "c9c77d012e924d9258ed8af4c3eef26d5987ead5",
          "message": "ci(experimental/x32): try out x64 x32 ABI builds\n\nThe ILP32 data model used by this target has the nice property in theory\nof reducing code size, as pointers are still 32-bit, like in 32-bit\narchitectures. However, it support is very lacking. Let's see how things\nbreak.",
          "timestamp": "2022-04-02T15:01:09+02:00",
          "tree_id": "5668fc07e8e28cc55f5be2c1c909c61cce2df61c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c9c77d012e924d9258ed8af4c3eef26d5987ead5"
        },
        "date": 1648906712903,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13213413,
            "range": "± 748279",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 961459928,
            "range": "± 33920695",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9784234489,
            "range": "± 244408533",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1374037829,
            "range": "± 32741383",
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
          "id": "fe475c9fe21ce87894bf09fb5454d7867247f2ad",
          "message": "ci(experimental/x32): revert\n\nDebian x32 port, the only sane distro that supports the x32 ABI, has\nbroken packages when used in a multiarch configuration with the usual\nx64 ABI, because some versions of transitive dependencies are different\nbetween the x32 port and amd64 repositories, and some packages have\nrestrictive conflictive requirements that do not allow two slightly\ndifferent versions to coexist on the same system.\n\nThe proper solution would be to ditch the amd64 repositories and just\nuse the x32 ones, but the x32 ABI is disabled by default on most\nDebian-based kernels, and I bet that GH Actions runners do not have the\nnecessary kernel boot parameter to enable it back. QEMU does not seem to\nhave support for the x32 ABI, and we don't want to do heavy hackery on\ngetting conflicting packages working fine, so let's give up.",
          "timestamp": "2022-04-02T18:49:47+02:00",
          "tree_id": "a824f1c57cd2d1fc2be50376d615a23368102dbf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fe475c9fe21ce87894bf09fb5454d7867247f2ad"
        },
        "date": 1648920038767,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16870596,
            "range": "± 828861",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1141054466,
            "range": "± 23554228",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13135770705,
            "range": "± 385001726",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1700839200,
            "range": "± 81823574",
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
          "id": "1f15a86e83790f4646235a4dda9a730a4c8459dc",
          "message": "experiment(perf/png_file): print consumed time per pass (old code)",
          "timestamp": "2022-04-03T17:37:46+02:00",
          "tree_id": "41cff5a3d42a703d46bafa87f428cc8c2282dcf5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1f15a86e83790f4646235a4dda9a730a4c8459dc"
        },
        "date": 1649002009801,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14107167,
            "range": "± 162978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 946062431,
            "range": "± 5050445",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10594524889,
            "range": "± 282433865",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1349305406,
            "range": "± 5698466",
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
          "id": "a7b8629b269a8962c6e2814ec2ffe8853fe6a6bb",
          "message": "tweak(png_file): bring performance back to the last pre-refactor levels\n\nThe refactor we did changed how the number of Zopfli compression\niterations is calculated. This had a dramatic effect in the running time\nof our benchmarks, although these increased iterations yielded marginal\ntime savings. Let's review the Zopfli iteration number calculation logic\nto fix some previous calculation errors that made its performance not be\nas predictable as expected. While at it, reduce the number of filters\ntried in half, which should translate to 50% performance savings too\nwithout much impact on file sizes (most PNG files are well-served by a\nfew filters, and we are no longer worried on increasing file sizes\ncompared to the input files).",
          "timestamp": "2022-04-03T20:07:39+02:00",
          "tree_id": "02a697a89cf21fee8057fa08c97b0fa42099bdef",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a7b8629b269a8962c6e2814ec2ffe8853fe6a6bb"
        },
        "date": 1649011297803,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14497296,
            "range": "± 357402",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 751704177,
            "range": "± 4657032",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11934116823,
            "range": "± 55487187",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2686414235,
            "range": "± 12763294",
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
          "id": "22afbad8fce49e8cdec38e2864ede29b4961d411",
          "message": "tweak(png_file): addendum to a7b8629b269a8962c6e2814ec2ffe8853fe6a6bb",
          "timestamp": "2022-04-03T20:40:16+02:00",
          "tree_id": "f39af4e0344a94a920cdbacfe1e44248fa07274a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/22afbad8fce49e8cdec38e2864ede29b4961d411"
        },
        "date": 1649012941827,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17415034,
            "range": "± 809524",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 907079485,
            "range": "± 11395650",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14239779622,
            "range": "± 131084047",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3344379517,
            "range": "± 40526948",
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
          "id": "61913c39861d2f66f587c2e436ab16f028659f3f",
          "message": "perf(png_file): optimize PNG chunk stripping pass code\n\nBy implementing this pass ourselves we no longer have to deal with\nOxiPNG's decoding of the image under the hood, which takes some precious\nmilliseconds to do, essentially turning it as quick of a pass as\npossible.",
          "timestamp": "2022-04-03T23:21:18+02:00",
          "tree_id": "161e95fbd2f52cca47def3979e1c9fe73800390b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/61913c39861d2f66f587c2e436ab16f028659f3f"
        },
        "date": 1649022599442,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12651188,
            "range": "± 2921817",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 733214854,
            "range": "± 2697350",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11105077470,
            "range": "± 61543105",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2573884741,
            "range": "± 15185425",
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
          "id": "61913c39861d2f66f587c2e436ab16f028659f3f",
          "message": "perf(png_file): optimize PNG chunk stripping pass code\n\nBy implementing this pass ourselves we no longer have to deal with\nOxiPNG's decoding of the image under the hood, which takes some precious\nmilliseconds to do, essentially turning it as quick of a pass as\npossible.",
          "timestamp": "2022-04-03T21:21:18Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/61913c39861d2f66f587c2e436ab16f028659f3f"
        },
        "date": 1649036853934,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12249200,
            "range": "± 1131560",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 695470120,
            "range": "± 34940024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9419176895,
            "range": "± 137133286",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2402083331,
            "range": "± 76128751",
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
          "id": "999ec024fe39071ae8240f96922877a10a9e1cf6",
          "message": "tweak(json_file/debloater): add selectors for keys used as comments\n\nThese are ignored by the programs who parse the asset types that get\ndebloated, but I have seen some authors including them. With the custom\nfiles feature they now have less reasons to do that.",
          "timestamp": "2022-04-04T17:05:19+02:00",
          "tree_id": "9808d4370300c14cf40ec65a539e3ae5d1e38b5e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/999ec024fe39071ae8240f96922877a10a9e1cf6"
        },
        "date": 1649086469980,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16891564,
            "range": "± 1678364",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 939293930,
            "range": "± 11518875",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12481623985,
            "range": "± 193261931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3257449361,
            "range": "± 69573535",
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
          "id": "0424cee91618da961a7fd17a180957a52381d206",
          "message": "chore(deps): bump enumset from 1.0.8 to 1.0.9 (#88)\n\nBumps [enumset](https://github.com/Lymia/enumset) from 1.0.8 to 1.0.9.\r\n- [Release notes](https://github.com/Lymia/enumset/releases)\r\n- [Changelog](https://github.com/Lymia/enumset/blob/master/RELEASES.md)\r\n- [Commits](https://github.com/Lymia/enumset/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: enumset\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T10:21:20+02:00",
          "tree_id": "63f0c46fb48edd514d529152a1b508fcc4fc3903",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0424cee91618da961a7fd17a180957a52381d206"
        },
        "date": 1649149096853,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14585476,
            "range": "± 417886",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 742830747,
            "range": "± 30176011",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11622064865,
            "range": "± 545486492",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2646075086,
            "range": "± 152045767",
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
          "id": "05cc7fa87ef7e027e608cc2b2c0922263c5cec1b",
          "message": "chore(deps): bump gstreamer-audio from 0.18.5 to 0.18.7 (#87)\n\nBumps gstreamer-audio from 0.18.5 to 0.18.7.\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: gstreamer-audio\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T10:24:08+02:00",
          "tree_id": "25538a64da1ea801ce1793e1d453d24de24d1099",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/05cc7fa87ef7e027e608cc2b2c0922263c5cec1b"
        },
        "date": 1649149125640,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14735281,
            "range": "± 257279",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 756672008,
            "range": "± 2781411",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11733975928,
            "range": "± 66970484",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2801626410,
            "range": "± 13278618",
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
          "id": "aaee249e96a87e659a850a704902a67861f9d14b",
          "message": "chore(dependabot): tweak Cargo update check frequency, keep GH Actions updated",
          "timestamp": "2022-04-05T11:20:13+02:00",
          "tree_id": "9dd1948bc975b284f74614afb1ee93cefd6f02e9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aaee249e96a87e659a850a704902a67861f9d14b"
        },
        "date": 1649152169564,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13710053,
            "range": "± 722810",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 851672146,
            "range": "± 8233107",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11213180479,
            "range": "± 180155092",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2867067974,
            "range": "± 63467587",
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
          "id": "9e88976a77acf3ffeeea6c50fbc8587c251ada5a",
          "message": "chore(deps): bump enumset from 1.0.9 to 1.0.10 (#90)\n\nBumps [enumset](https://github.com/Lymia/enumset) from 1.0.9 to 1.0.10.\r\n- [Release notes](https://github.com/Lymia/enumset/releases)\r\n- [Changelog](https://github.com/Lymia/enumset/blob/master/RELEASES.md)\r\n- [Commits](https://github.com/Lymia/enumset/compare/v1.0.9...v1.0.10)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: enumset\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T11:10:02+02:00",
          "tree_id": "92aa8611e8645d668b1683fd120a9e8e88918823",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9e88976a77acf3ffeeea6c50fbc8587c251ada5a"
        },
        "date": 1649152320454,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16476562,
            "range": "± 1258334",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 891817216,
            "range": "± 13061326",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12340237782,
            "range": "± 192883750",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3184219653,
            "range": "± 40357340",
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
          "id": "c69b31ad07ecf66641bd6d6ad832dc8d17b277d8",
          "message": "chore(deps): bump EndBug/add-and-commit from 7 to 9 (#91)\n\nBumps [EndBug/add-and-commit](https://github.com/EndBug/add-and-commit) from 7 to 9.\r\n- [Release notes](https://github.com/EndBug/add-and-commit/releases)\r\n- [Changelog](https://github.com/EndBug/add-and-commit/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/EndBug/add-and-commit/compare/v7...v9)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: EndBug/add-and-commit\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T11:41:43+02:00",
          "tree_id": "715e7f6deb60cb6d5749a67343ce66c548c8fc90",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c69b31ad07ecf66641bd6d6ad832dc8d17b277d8"
        },
        "date": 1649153250615,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12439951,
            "range": "± 180452",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 733796648,
            "range": "± 3826722",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11087615482,
            "range": "± 46110892",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2595986582,
            "range": "± 62656014",
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
          "id": "2467f6802a9d27cf47bbfae29562f678decaad67",
          "message": "chore(deps): bump actions/checkout from 2 to 3 (#92)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 2 to 3.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v2...v3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T11:42:11+02:00",
          "tree_id": "3671c880b9070dfe7ffaa5401a91a6bf6ee015f7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2467f6802a9d27cf47bbfae29562f678decaad67"
        },
        "date": 1649153394322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16925873,
            "range": "± 1450525",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 913653060,
            "range": "± 22240532",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12313186900,
            "range": "± 173723907",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3115852032,
            "range": "± 139503455",
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
          "id": "3471e5223543a195171ba93511f9f63ff992ea79",
          "message": "chore(deps): bump gstreamer-app from 0.18.0 to 0.18.7 (#89)\n\nBumps gstreamer-app from 0.18.0 to 0.18.7.\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: gstreamer-app\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-05T11:38:45+02:00",
          "tree_id": "cbbec63652ca60a7fd46f3c7fa7bead979268c47",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3471e5223543a195171ba93511f9f63ff992ea79"
        },
        "date": 1649154113411,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12680978,
            "range": "± 448098",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 735886872,
            "range": "± 4130673",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11100692779,
            "range": "± 65952945",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2614214030,
            "range": "± 32862523",
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
          "id": "2255bb0dd8c5deae5845a85b883e1bd8edaed7b8",
          "message": "ci: do not run build workflow when no source file changes\n\nThis is meant to save precious time and resources for commits that just\ntweak something in the documentation and cannot possibly affect the\nbuild output. PRs still run the workflow anyway because they might want\nto refactor the entire project structure or something.",
          "timestamp": "2022-04-05T12:06:23+02:00",
          "tree_id": "36c36c686723be209eec10d8f13eba77322abb2b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2255bb0dd8c5deae5845a85b883e1bd8edaed7b8"
        },
        "date": 1649155571375,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12499353,
            "range": "± 400009",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 731364769,
            "range": "± 4551985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11151296776,
            "range": "± 63130122",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2607070929,
            "range": "± 26631074",
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
          "id": "08c4c5977f9865e8108f00cafd141ad8ed4dd264",
          "message": "fix(optifine/cit): do not skip CIT tex. metadata. Debloat item models\n\nSome OptiFine features, namely \"custom items\" (CIT) and \"connected\ntextures\" (CTM), support the vanilla way of putting a .mcmeta file\nalongside a texture to make it animated. v0.3.0 processed these files,\nbut when moving to using asset masks we stopped doing so incorrectly,\ndue to an oversight.\n\nThe custom items feature can also read vanilla item models from\nOptiFine-specific directories in a resource pack. These were processed\nby PackSquash anyway, but by falling back to the generic JSON asset\ntype, which disabled any debloating done to vanilla models.\n\nDoing integration tests with real packs in preparation for the v0.3.1\nrelease showed these problems, so let's fix them.",
          "timestamp": "2022-04-05T13:16:17+02:00",
          "tree_id": "1b7f0f3572fa7fdf356fc5f0b74c4c31ebb6d667",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/08c4c5977f9865e8108f00cafd141ad8ed4dd264"
        },
        "date": 1649158915237,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16453096,
            "range": "± 805748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 832358635,
            "range": "± 12400689",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13394875220,
            "range": "± 155266412",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3072548704,
            "range": "± 55594810",
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
          "id": "128ab820890c5bfd27574be2b9496c2713561113",
          "message": "chore(deps): update for the last time before v0.3.1 🎉",
          "timestamp": "2022-04-06T23:24:05+02:00",
          "tree_id": "aa5e2b39b2b617d9a9fa40f6287a912e1ad3e4bb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/128ab820890c5bfd27574be2b9496c2713561113"
        },
        "date": 1649283885056,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14388739,
            "range": "± 741004",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 838536294,
            "range": "± 14125878",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11200250463,
            "range": "± 94763682",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2780860724,
            "range": "± 32149719",
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
          "id": "128ab820890c5bfd27574be2b9496c2713561113",
          "message": "chore(deps): update for the last time before v0.3.1 🎉",
          "timestamp": "2022-04-06T23:24:05+02:00",
          "tree_id": "aa5e2b39b2b617d9a9fa40f6287a912e1ad3e4bb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/128ab820890c5bfd27574be2b9496c2713561113"
        },
        "date": 1649283934452,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15287991,
            "range": "± 769367",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 883848729,
            "range": "± 4828193",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14221428882,
            "range": "± 52904347",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3232188309,
            "range": "± 106471955",
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
          "id": "128ab820890c5bfd27574be2b9496c2713561113",
          "message": "chore(deps): update for the last time before v0.3.1 🎉",
          "timestamp": "2022-04-06T21:24:05Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/128ab820890c5bfd27574be2b9496c2713561113"
        },
        "date": 1649283988783,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15555003,
            "range": "± 865073",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 893438622,
            "range": "± 9840584",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14296120281,
            "range": "± 112919205",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3291863808,
            "range": "± 131277729",
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
          "id": "8d1eb2a276bcc615449d9f8bb3e47b479bf78a4c",
          "message": "chore: bump PackSquash packages to the next release version",
          "timestamp": "2022-04-07T12:37:34+02:00",
          "tree_id": "e020e0c98eedc8d7c85630a71b1a067e835b0629",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8d1eb2a276bcc615449d9f8bb3e47b479bf78a4c"
        },
        "date": 1649330195721,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14112854,
            "range": "± 166079",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 699304734,
            "range": "± 7289239",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13821882405,
            "range": "± 75601058",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2508120721,
            "range": "± 12606953",
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
          "id": "8d1eb2a276bcc615449d9f8bb3e47b479bf78a4c",
          "message": "chore: bump PackSquash packages to the next release version",
          "timestamp": "2022-04-07T10:37:34Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8d1eb2a276bcc615449d9f8bb3e47b479bf78a4c"
        },
        "date": 1649640889677,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12817845,
            "range": "± 1047504",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 691453377,
            "range": "± 255743537",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13001888550,
            "range": "± 7937080020",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2463603917,
            "range": "± 12507436",
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
          "id": "48b9aefd130a9e0b0aca5c74fc86b76093a64115",
          "message": "chore(deps): bump sysinfo from 0.23.8 to 0.23.9 (#94)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.8 to 0.23.9.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-04-11T10:04:52+02:00",
          "tree_id": "7d4665d2c237d33ea8c9ea09aea9048332a86d7f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/48b9aefd130a9e0b0aca5c74fc86b76093a64115"
        },
        "date": 1649666112743,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15451549,
            "range": "± 914768",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 829307632,
            "range": "± 13414122",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15241714174,
            "range": "± 109016535",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2958390090,
            "range": "± 20092100",
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
      }
    ]
  }
}