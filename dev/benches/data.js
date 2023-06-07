window.BENCHMARK_DATA = {
  "lastUpdate": 1686178336871,
  "repoUrl": "https://github.com/ComunidadAylas/PackSquash",
  "entries": {
    "PackSquash library quick benchmarks": [
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "aa7f8f10b54102e0b65e7812001180babd175f77",
          "message": "fix(deps): update rust crate patricia_tree to 0.5.6",
          "timestamp": "2023-04-12T10:25:31Z",
          "tree_id": "81537c316487c6979bbf57634a14fb7de4ce2052",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aa7f8f10b54102e0b65e7812001180babd175f77"
        },
        "date": 1681318567021,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9603865,
            "range": "± 115165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 294967338,
            "range": "± 12124786",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4319219316,
            "range": "± 21833144",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 633464644,
            "range": "± 8549139",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "c629a5530cbc0f0b5bfd50fdc85e0708b6acbaa9",
          "message": "chore: rename `type_alias_impl_trait` feature gate to `impl_trait_in_assoc_type`\n\nThis change was introduced by\nhttps://github.com/rust-lang/rust/pull/110237 in the latest nightlies.",
          "timestamp": "2023-04-13T15:56:05+02:00",
          "tree_id": "3d51c00c53ffc79b7b1ccbe076a0b19d99bda1bd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c629a5530cbc0f0b5bfd50fdc85e0708b6acbaa9"
        },
        "date": 1681396330477,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13039263,
            "range": "± 17630626",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 407852716,
            "range": "± 7640996",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5642009865,
            "range": "± 59645559",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 776160568,
            "range": "± 11363664",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "baab04d023250030e5c4e979e164f46e585323dd",
          "message": "fix(deps): update rust crate vorbis_rs to 0.3.0",
          "timestamp": "2023-04-14T16:09:52Z",
          "tree_id": "3933b4a0570f48ddb672422fb5a1b2cd0bda2ea9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/baab04d023250030e5c4e979e164f46e585323dd"
        },
        "date": 1681504184486,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11119688,
            "range": "± 12353191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 335086796,
            "range": "± 5239869",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4662073572,
            "range": "± 19292650",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 646574605,
            "range": "± 7304539",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "ad362f78bc8dc46b9cb7eed0321d740e77301bbb",
          "message": "perf(png_file): use experimental OxiPNG raw API\n\nThis new API renders it unnecessary for PackSquash to encode\nintermediate PNGs for them to be consumed by OxiPNG, which is noticeably\nfaster and more memory efficient, especially for smaller images, where\nthe encoding overhead is significant.\n\nWhile at it, the image processing code was refactored to be much more\nergonomic and easier to read, 0 is now rejected as a value for the\n`maximum_width_and_height` option, the OxiPNG options were slightly\ntweaked to use a better set of filters, and images no longer are\ndownsized if their color type cannot be changed.",
          "timestamp": "2023-04-16T19:14:07+02:00",
          "tree_id": "46f3dbf63533ad6700c33706b267489dc7a74254",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ad362f78bc8dc46b9cb7eed0321d740e77301bbb"
        },
        "date": 1681667455948,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9497940,
            "range": "± 10825940",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 289746747,
            "range": "± 4704674",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1801161028,
            "range": "± 24382336",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 252057137,
            "range": "± 7163394",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1b5211c38e7fd90c7aaa990db600ef5eb29ad519",
          "message": "chore(deps): update rust crate git2 to 0.17.1",
          "timestamp": "2023-04-16T20:05:36Z",
          "tree_id": "d54f548f566c0fde06c00f6230c92ecc71aaed24",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1b5211c38e7fd90c7aaa990db600ef5eb29ad519"
        },
        "date": 1681688645648,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13526210,
            "range": "± 727097",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 392382359,
            "range": "± 12335503",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2334704596,
            "range": "± 87673091",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 332659251,
            "range": "± 8643696",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "de8730a2be619efcb1ecbbf6cdb1ca8ae1ce11a5",
          "message": "chore(deps): update dependency soupsieve to v2.4.1",
          "timestamp": "2023-04-17T02:17:59Z",
          "tree_id": "5cc04d34d2b4718520d583f3158ff638066c6f4c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/de8730a2be619efcb1ecbbf6cdb1ca8ae1ce11a5"
        },
        "date": 1681722722671,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9843554,
            "range": "± 11586265",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 292780185,
            "range": "± 4517863",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1812536600,
            "range": "± 30747045",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 254944974,
            "range": "± 8308215",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "366c30162b8bca05260ed7d5893920dfcdf59c60",
          "message": "perf(png_file): simplify and optimize PixelArray type\n\nExamining the assembly generated by the compiler for release builds\nrevealed that it was not smart enough to avoid a expensive Vec\nreconstruction when converting a Vec<u8> to its strongly-typed\nVec<RGBA8> variant.\n\nTo increase performance and simplify the code, let's make the PixelArray\nalways hold a raw byte buffer, which can be converted to RGBA8 slice\nreferences at zero cost, and doesn't need to be converted back to a raw\nbyte buffer for consumption by OxiPNG.",
          "timestamp": "2023-04-18T15:50:57+02:00",
          "tree_id": "c257772341521ffb6e74b4ae1ad0a95eeac45d31",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/366c30162b8bca05260ed7d5893920dfcdf59c60"
        },
        "date": 1681829720162,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9484059,
            "range": "± 11842693",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 290116620,
            "range": "± 6970693",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1804743981,
            "range": "± 31280089",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 254575024,
            "range": "± 9052605",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6312efe4d298c33e9cb0af5dab8d07ae2069f933",
          "message": "chore(deps): update dependency filelock to v3.12.0",
          "timestamp": "2023-04-18T18:59:30Z",
          "tree_id": "cc9476fdce2e1144dff4138bb4f64a39667f68ae",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6312efe4d298c33e9cb0af5dab8d07ae2069f933"
        },
        "date": 1681864436875,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10271066,
            "range": "± 163829",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 331244441,
            "range": "± 13395510",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2087371286,
            "range": "± 30188303",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259675484,
            "range": "± 6263788",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bed91ec704e433b7ba9e3a8154d473ccecc78733",
          "message": "fix(deps): update rust crate regex to 1.8.0",
          "timestamp": "2023-04-21T00:23:21Z",
          "tree_id": "122f3a252b9b8a4ddda231686582cdc3ff20485a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bed91ec704e433b7ba9e3a8154d473ccecc78733"
        },
        "date": 1682054924505,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10453451,
            "range": "± 11667945",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 327718913,
            "range": "± 3786165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2108950280,
            "range": "± 22809371",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 261489603,
            "range": "± 4865503",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "e3b4c031a83d9583243cb9b23e0cf9dedebb6012",
          "message": "fix(squash_zip): do not return zero bytes written when the spooling buffer size is zero\n\nThis causes a WriteZero I/O error by the WriteAll future, as it assumes\nthat writing zero bytes signals a definitive \"stream full\" condition.",
          "timestamp": "2023-04-21T18:33:09+02:00",
          "tree_id": "f2b62460cc0153be932d0b46cab600cdcc7e6289",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e3b4c031a83d9583243cb9b23e0cf9dedebb6012"
        },
        "date": 1682096964775,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10106566,
            "range": "± 340124",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 292684784,
            "range": "± 5556453",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1809873040,
            "range": "± 4939037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 241724986,
            "range": "± 5783850",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f4b1eb5a34c1ca50e9232eb8ed0b05f55ad0b71d",
          "message": "fix(deps): update rust crate regex to 1.8.1",
          "timestamp": "2023-04-21T21:11:28Z",
          "tree_id": "33439370e7bbe77232581c34fe1d2a4c9747fa09",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f4b1eb5a34c1ca50e9232eb8ed0b05f55ad0b71d"
        },
        "date": 1682131110544,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10508081,
            "range": "± 9359141",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 334865046,
            "range": "± 5804094",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2030309069,
            "range": "± 26869130",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 264583240,
            "range": "± 5591367",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "74f533e11946908e799f840de823f464f2eaeaa2",
          "message": "feat(os_fs): add MacOS resource fork ZIP folder to list of folders to ignore\n\nYou know Apple, you could have used the extra field feature defined in\nthe ZIP specification to store resource fork data, but I get you think\ndifferent.",
          "timestamp": "2023-04-22T12:33:02+02:00",
          "tree_id": "70c67732df402704470deb1429944148c33007b4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/74f533e11946908e799f840de823f464f2eaeaa2"
        },
        "date": 1682162488687,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10142093,
            "range": "± 200967",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 293243901,
            "range": "± 18161686",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1859669820,
            "range": "± 24416085",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 247022198,
            "range": "± 4647456",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "133525ef9458e6a7c43e0a9ebeb5ecfcf328e779",
          "message": "fix(deps): update rust crate tokio-stream to 0.1.13",
          "timestamp": "2023-04-25T21:17:14Z",
          "tree_id": "2d53dc339fd4306154b83db9dab77be038bca7a7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/133525ef9458e6a7c43e0a9ebeb5ecfcf328e779"
        },
        "date": 1682478165089,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10207705,
            "range": "± 10956901",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 350893234,
            "range": "± 3250154",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1945460469,
            "range": "± 19630760",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259356831,
            "range": "± 7110143",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "998af630fda7cf2947404e7529b7848a47a5c9d0",
          "message": "fix(deps): update rust crate tokio-util to 0.7.8",
          "timestamp": "2023-04-26T02:26:44Z",
          "tree_id": "9b35b6edea4de0d64fa0826a2560402438f52754",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/998af630fda7cf2947404e7529b7848a47a5c9d0"
        },
        "date": 1682497251525,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9863408,
            "range": "± 246858",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 351025995,
            "range": "± 3512131",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1929519626,
            "range": "± 20251310",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 263122380,
            "range": "± 5304664",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d8cd703495739c4fc3bfddfadb1356bee551c014",
          "message": "fix(deps): update rust crate tokio-stream to 0.1.14",
          "timestamp": "2023-04-26T12:46:08Z",
          "tree_id": "0588314562f65dedc11f7bc438043229a712e79e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d8cd703495739c4fc3bfddfadb1356bee551c014"
        },
        "date": 1682534634571,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15701778,
            "range": "± 697012",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 550013354,
            "range": "± 16052281",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3311698191,
            "range": "± 65299307",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 397406061,
            "range": "± 11081883",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "36e9cd4a2c254fe0894df45319de58b89fe1dab4",
          "message": "Revert \"chore(deps): update helper python scripts (#209)\"\n\nThis reverts commit 218a32fbb150bfd120546250ae6c48297a8c9f8a.\n\nThe requests package depends on urllib3<1.27 and >=1.21.1, but we\nupdated it to the just-released 2.0.0 version.",
          "timestamp": "2023-04-26T23:41:28+02:00",
          "tree_id": "1fd541be3cfe373f7a92a635f8e2a8c6ba759e33",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36e9cd4a2c254fe0894df45319de58b89fe1dab4"
        },
        "date": 1682547447314,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13081007,
            "range": "± 362639",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 510099980,
            "range": "± 12504023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3210250496,
            "range": "± 24836420",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 365767891,
            "range": "± 4072401",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a2b709405534479c46df49a57c95b91583807f43",
          "message": "chore(deps): update dependency requests to v2.29.0",
          "timestamp": "2023-04-27T02:07:26Z",
          "tree_id": "12f0ce1673a0213c67ec30002745620c6b2de8bc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a2b709405534479c46df49a57c95b91583807f43"
        },
        "date": 1682576811202,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11116002,
            "range": "± 12001407",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 426724179,
            "range": "± 6611490",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2842955448,
            "range": "± 17344921",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 306089875,
            "range": "± 3437435",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "8b60b499834efc164fcee3489103486284b1afa9",
          "message": "chore: update and cleanup some more dependencies",
          "timestamp": "2023-04-29T15:48:20+02:00",
          "tree_id": "586ff04267b47278457a21a4ad3470fef89eaf83",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8b60b499834efc164fcee3489103486284b1afa9"
        },
        "date": 1682779303424,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9752301,
            "range": "± 12661706",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 419858650,
            "range": "± 2765886",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2496027858,
            "range": "± 24354474",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 291032851,
            "range": "± 5919878",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "71f77e1c783e6a66c25fe78e856c45d1a3331114",
          "message": "fix(deps): update rust crate imagequant to 4.2.0",
          "timestamp": "2023-05-01T02:14:27Z",
          "tree_id": "25a211b3840e2aee13ed9fa407d7636297d91049",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/71f77e1c783e6a66c25fe78e856c45d1a3331114"
        },
        "date": 1682923847661,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9918567,
            "range": "± 17640214",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 175181266,
            "range": "± 1368421",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2463985353,
            "range": "± 29232383",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 293815603,
            "range": "± 4914743",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "f27b5b6a0bf89db17ee710e2b58ac194f915d053",
          "message": "fix(deps): update rust crate enumset to 1.0.13",
          "timestamp": "2023-05-01T14:42:22Z",
          "tree_id": "579e601afd786097393e0928a75dfd37fb4caae9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f27b5b6a0bf89db17ee710e2b58ac194f915d053"
        },
        "date": 1682966485255,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10913455,
            "range": "± 133105",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 177619233,
            "range": "± 1255956",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2812607837,
            "range": "± 26516435",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 303427367,
            "range": "± 3616701",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "0f9b1f880e5625d6f6c04c12f65797b6b16a7117",
          "message": "fix(config): correct default value documentation for `ogg_obfuscation`",
          "timestamp": "2023-05-04T21:28:42+02:00",
          "tree_id": "25ec69c5237ad112239a041cad73c7ab1e7ad978",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0f9b1f880e5625d6f6c04c12f65797b6b16a7117"
        },
        "date": 1683230970488,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12894509,
            "range": "± 10966366",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 213057346,
            "range": "± 6214175",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3390839293,
            "range": "± 56325363",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 368834164,
            "range": "± 8929791",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6c565a817927344969477f2a9fa5afc6ea8b766b",
          "message": "chore(deps): update rust crate time to 0.3.21",
          "timestamp": "2023-05-06T03:40:01Z",
          "tree_id": "e32cddf5c2212854fe7f9baf8c792d52c645c876",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6c565a817927344969477f2a9fa5afc6ea8b766b"
        },
        "date": 1683358975733,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10716593,
            "range": "± 11199721",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173743489,
            "range": "± 889766",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2730574165,
            "range": "± 20394966",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 303588700,
            "range": "± 6435070",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "distinct": true,
          "id": "22b7766dcb506609e443d49ee95480665a61ccc2",
          "message": "fix(deps): update rust crate sysinfo to 0.29.0",
          "timestamp": "2023-05-06T11:44:17Z",
          "tree_id": "eeb4c279d172802375e096fc2a46980a966e4bc1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/22b7766dcb506609e443d49ee95480665a61ccc2"
        },
        "date": 1683375408998,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10746928,
            "range": "± 249978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 174822964,
            "range": "± 756470",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2812578041,
            "range": "± 21615783",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 303160997,
            "range": "± 6360215",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "distinct": true,
          "id": "6f0e140d1f47048be4752f5ac5c43bd11ce7e96e",
          "message": "fix(deps): update rust crate wmi to 0.13.0",
          "timestamp": "2023-05-06T14:59:45Z",
          "tree_id": "ec5885887ba5a1625b5b5c943d17a62f1f66133a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6f0e140d1f47048be4752f5ac5c43bd11ce7e96e"
        },
        "date": 1683387195423,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13150110,
            "range": "± 617869",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 215446677,
            "range": "± 14738546",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3400778652,
            "range": "± 69819332",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 372408006,
            "range": "± 6091462",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c65e527e9049d27d5ce7189da0205b95520fd80d",
          "message": "fix(deps): update rust crate enumset to 1.1.2 (#220)\n\n* fix(deps): update rust crate enumset to 1.1.2\r\n\r\n* chore(clippy): fix Clippy warnings and update Cargo deny configuration\r\n\r\n---------\r\n\r\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>\r\nCo-authored-by: Alejandro González <me@alegon.dev>",
          "timestamp": "2023-05-10T12:19:17+02:00",
          "tree_id": "690785ea8ee7f8f02e952a66dfc1afe0e9b21b95",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c65e527e9049d27d5ce7189da0205b95520fd80d"
        },
        "date": 1683716496148,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10551103,
            "range": "± 10065065",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 175813318,
            "range": "± 798668",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2737552028,
            "range": "± 21469248",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 298347553,
            "range": "± 6373931",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "61f1f4ea4944390cbbd14f47359ae381fa4ee781",
          "message": "fix(deps): update rust crate serde to 1.0.163",
          "timestamp": "2023-05-11T07:20:18Z",
          "tree_id": "9358fe2ccda23503241fda526821edd0722175cf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/61f1f4ea4944390cbbd14f47359ae381fa4ee781"
        },
        "date": 1683806510699,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11227703,
            "range": "± 12299751",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 166286133,
            "range": "± 2733487",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2225999221,
            "range": "± 61532788",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 295559643,
            "range": "± 8699252",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "7b32464fb559b91a3177aaa69dc9356ba9a3bd5c",
          "message": "feat(png_image): update OxiPNG raw API fork to its latest revision\n\nThis should be even more performant, generate smaller files, and be even\nbetter in general. It also simplifies our code a fair amount.",
          "timestamp": "2023-05-13T13:22:51+02:00",
          "tree_id": "0c0c0cbf2f2e8a9edb292223aec7d3085354e4a8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7b32464fb559b91a3177aaa69dc9356ba9a3bd5c"
        },
        "date": 1683980405684,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10628329,
            "range": "± 11656284",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 162790486,
            "range": "± 1118322",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2144395763,
            "range": "± 34077410",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 269644752,
            "range": "± 2655559",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "180e163b5a609a7a7fb037785861006eac7d2bdc",
          "message": "fix(deps): update rust crate uuid to 1.3.3",
          "timestamp": "2023-05-15T11:20:37Z",
          "tree_id": "9d47ddf5b37bbaba503a3da6d6ebb0e68a26cecd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/180e163b5a609a7a7fb037785861006eac7d2bdc"
        },
        "date": 1684161754112,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10452682,
            "range": "± 14304689",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 160105333,
            "range": "± 1799820",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1997317219,
            "range": "± 42448969",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259294880,
            "range": "± 5859979",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "3830950d8569cb45401704f0623f0ac50ec56050",
          "message": "chore(deps): update OxiPNG to use latest revision\n\nThis is necessary because the raw API PR was merged, and its original\nsource branch deleted.\n\nWhile at it, let's upgrade locked dependency versions and the toml\ncrate.",
          "timestamp": "2023-05-19T17:43:28+02:00",
          "tree_id": "7aee3785b6841de01b96f60c68f0bd3aa4b6fa29",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3830950d8569cb45401704f0623f0ac50ec56050"
        },
        "date": 1684513256706,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10576230,
            "range": "± 15947744",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 163822982,
            "range": "± 814224",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2176419386,
            "range": "± 41062879",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 274330556,
            "range": "± 4597762",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "428cc044867903d47c7fbc30d1178b805ca13b9c",
          "message": "fix(deps): update rust crate java-properties to v2 (#222)\n\n* fix(deps): update rust crate java-properties to v2\r\n\r\n* chore(clippy): fix new lints\r\n\r\n---------\r\n\r\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>\r\nCo-authored-by: Alejandro González <me@alegon.dev>",
          "timestamp": "2023-05-21T11:38:00+02:00",
          "tree_id": "5183cfeb080c6d2419a0dc2011935d295adb6d3b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/428cc044867903d47c7fbc30d1178b805ca13b9c"
        },
        "date": 1684664021604,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13529112,
            "range": "± 12868829",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 195156520,
            "range": "± 3173213",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2241027611,
            "range": "± 78577416",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 309825118,
            "range": "± 5220158",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ce23124e656d7dd606f542675562046ba87bd2ed",
          "message": "chore(deps): update dependency requests to v2.31.0",
          "timestamp": "2023-05-22T16:34:04Z",
          "tree_id": "497a08a7446c1d37be471bd042c69f3a422fcfdc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ce23124e656d7dd606f542675562046ba87bd2ed"
        },
        "date": 1684789275810,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12734975,
            "range": "± 14205837",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 194299914,
            "range": "± 3700233",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2589653176,
            "range": "± 86936951",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 313696490,
            "range": "± 5745419",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a670b35ff43f5c74e87add9cf5612f4bd715cc3b",
          "message": "fix(deps): update rust crate regex to 1.8.2",
          "timestamp": "2023-05-23T01:05:22Z",
          "tree_id": "3eb6ff9268d6b60d9236dc295af436731ab3b106",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a670b35ff43f5c74e87add9cf5612f4bd715cc3b"
        },
        "date": 1684817936124,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9944551,
            "range": "± 11734181",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 159325919,
            "range": "± 2083278",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2013920094,
            "range": "± 33697549",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 251977426,
            "range": "± 2819501",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eea4233ceb2ed3b5d19e528328721e056f1e7a6a",
          "message": "fix(deps): update rust crate regex to 1.8.3",
          "timestamp": "2023-05-25T20:34:34Z",
          "tree_id": "b073608dbe9f58c83330ea8f9e42c06817965a15",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eea4233ceb2ed3b5d19e528328721e056f1e7a6a"
        },
        "date": 1685068554050,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13478683,
            "range": "± 14207463",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 205399248,
            "range": "± 1865621",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2751043001,
            "range": "± 64346562",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 329806044,
            "range": "± 1753580",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "a92fd5d68b111d2d1d1b290704354248047c438e",
          "message": "chore: update several dependencies",
          "timestamp": "2023-05-27T13:45:51+02:00",
          "tree_id": "f54d150ed833d0986af80c4f9f27cea204b19716",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a92fd5d68b111d2d1d1b290704354248047c438e"
        },
        "date": 1685190105319,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9195229,
            "range": "± 9911931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157132022,
            "range": "± 1997578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2024196726,
            "range": "± 34426655",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245145514,
            "range": "± 2131100",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "b3c5a7fe77f8806a4981065292e7271e23fc43d2",
          "message": "chore: update Zopfli to v0.7.4\n\nv0.7.3 was yanked and we better forget it existed.",
          "timestamp": "2023-05-27T22:59:47+02:00",
          "tree_id": "10eb4e237afcc9c992ccf308696aba0ad1111b10",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b3c5a7fe77f8806a4981065292e7271e23fc43d2"
        },
        "date": 1685224048114,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9030672,
            "range": "± 69120",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 155630180,
            "range": "± 25050160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2009152669,
            "range": "± 21647917",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 244322441,
            "range": "± 3083866",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b17b55be4769f54c70ed18b292161ac9dacaab3d",
          "message": "fix(deps): update rust crate tokio to 1.28.2",
          "timestamp": "2023-05-28T00:29:22Z",
          "tree_id": "e2fe6a7bb99a1e3c240f3b9071d2976c3fbcc6d9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b17b55be4769f54c70ed18b292161ac9dacaab3d"
        },
        "date": 1685247303873,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8512021,
            "range": "± 9812646",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 154211035,
            "range": "± 1580510",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1928510995,
            "range": "± 36675199",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 236698972,
            "range": "± 2541763",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "02087b1ce229d8562c7d978b0ad5fc190fb28a9f",
          "message": "fix(deps): update rust crate log to 0.4.18",
          "timestamp": "2023-05-28T06:10:21Z",
          "tree_id": "23716e74a3bdf14045d409c9e1954f63716d209f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/02087b1ce229d8562c7d978b0ad5fc190fb28a9f"
        },
        "date": 1685270847584,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8371324,
            "range": "± 94959",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 153975776,
            "range": "± 24176822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1917253145,
            "range": "± 37839995",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 234276244,
            "range": "± 2947328",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bd6e8d7a34031443afeea0671c6d4bba5dd0aa93",
          "message": "fix(deps): update rust crate patricia_tree to 0.6.0",
          "timestamp": "2023-05-28T13:32:46Z",
          "tree_id": "50d6fdd8858ba72d3344159c8bd9f10df69dbb3c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bd6e8d7a34031443afeea0671c6d4bba5dd0aa93"
        },
        "date": 1685284825624,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8441808,
            "range": "± 268053",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 153864293,
            "range": "± 9330166",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1931652571,
            "range": "± 21691076",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 235634064,
            "range": "± 1082903",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "180caa99fe03ee769a7ac76837fc30765c6b4a82",
          "message": "chore(deps): update rust crate git2 to 0.17.2",
          "timestamp": "2023-05-28T18:54:59Z",
          "tree_id": "8f25abfe5fb250bc9e4a0d264b9050e306876930",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/180caa99fe03ee769a7ac76837fc30765c6b4a82"
        },
        "date": 1685310121395,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9150028,
            "range": "± 126643",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 158020659,
            "range": "± 6481873",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2035452109,
            "range": "± 33698384",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245307065,
            "range": "± 1409346",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f1824d0952cc87e00b0b00e9826262f02417f61a",
          "message": "fix(deps): update rust crate const_format to 0.2.31",
          "timestamp": "2023-05-29T22:57:26Z",
          "tree_id": "5fe5e5eae0a52561f61909e25f7935e23cf66b7b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f1824d0952cc87e00b0b00e9826262f02417f61a"
        },
        "date": 1685408902911,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10814691,
            "range": "± 11998584",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 186375855,
            "range": "± 3376043",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2434868448,
            "range": "± 60613913",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 292669078,
            "range": "± 2598284",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5bc9b58960701841020e87d1d7661a3a8b930cfb",
          "message": "fix(deps): update rust crate symphonia to 0.5.3",
          "timestamp": "2023-05-30T02:28:32Z",
          "tree_id": "a1a96482b29b1efa04af680915162b8b56489464",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5bc9b58960701841020e87d1d7661a3a8b930cfb"
        },
        "date": 1685422275778,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9687684,
            "range": "± 13793521",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 159919968,
            "range": "± 1458034",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2065398611,
            "range": "± 45468008",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 250360398,
            "range": "± 2590350",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "85f4c73339a1f3a4c97c76b613e73c2cf8d91b18",
          "message": "fix(deps): update rust crate wmi to 0.13.1",
          "timestamp": "2023-06-02T21:55:48Z",
          "tree_id": "0b45731c6317eb6bf6b00a95ad2c992a44a7e728",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/85f4c73339a1f3a4c97c76b613e73c2cf8d91b18"
        },
        "date": 1685753898615,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8419427,
            "range": "± 13241338",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167060815,
            "range": "± 2147868",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2739034494,
            "range": "± 43154915",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 263311725,
            "range": "± 10928362",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1db553e674ab9879f1f10b127471cd5c5691f42",
          "message": "fix(deps): update rust crate aho-corasick to 1.0.2",
          "timestamp": "2023-06-04T14:30:51Z",
          "tree_id": "caf32efb4525aea410f47dc381439cd92a3ae548",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a1db553e674ab9879f1f10b127471cd5c5691f42"
        },
        "date": 1685898732095,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11752752,
            "range": "± 10556383",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 208626155,
            "range": "± 2876487",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3592785750,
            "range": "± 80376671",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 343861423,
            "range": "± 6138692",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0826525cf9984b7222adfd768a592b70e12ff9d4",
          "message": "fix(deps): update rust crate regex to 1.8.4",
          "timestamp": "2023-06-05T16:03:39Z",
          "tree_id": "6d36f7bd265cac53dc1ed8e678c25bf04903cddd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0826525cf9984b7222adfd768a592b70e12ff9d4"
        },
        "date": 1685993526559,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10031202,
            "range": "± 14527476",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 170701352,
            "range": "± 1169350",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3011472386,
            "range": "± 49723963",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 275311023,
            "range": "± 8495467",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "5d4c414cde25969e3fd3e53f0a09dd146c678664",
          "message": "fix(png_file): drop trailing bytes check\n\nThis check may cause lots of trouble to unsuspecting, not-so-technical\nusers with lots of PNG files, all for the little benefit of complying to\na standard that few applications are sensitive to anyway. Shame on image\neditors for generating this garbage, but we should handle it better,\nwhile letting users know that something is wrong.\n\nv0.4.0 will turn this to a warning. This can't be done now because the\ncurrent design does not allow this.",
          "timestamp": "2023-06-05T21:30:11+02:00",
          "tree_id": "a1cbb98023d2e5a1d9658f75482c430b840653aa",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5d4c414cde25969e3fd3e53f0a09dd146c678664"
        },
        "date": 1685995547090,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11326228,
            "range": "± 10928578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 205471591,
            "range": "± 3533384",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3492365429,
            "range": "± 91469448",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 336295972,
            "range": "± 8930466",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "0f92f66760385b09443812ff05a839a119be882b",
          "message": "ci: ignore flaky ARM64 QEMU musl test\n\nI've run the affected test on my development workstation successfully,\nso the source code is not to blame. My best guess is that the CI runner\nmay be running out of memory, given that the affected tests are\nexpensive to run.\n\nGiven that the affected code tests successfully when targeting glibc for\nthe same arch on CI, we don't have to dig on whatever QEMU/environment\nquirk is causing the issue, so let's just skip the troublesome test.",
          "timestamp": "2023-06-07T12:31:02+02:00",
          "tree_id": "971080e761041e021dc1e93fa3006dc5a6ba7e16",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0f92f66760385b09443812ff05a839a119be882b"
        },
        "date": 1686136139162,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10666372,
            "range": "± 294084",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 198443940,
            "range": "± 2112340",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3440695122,
            "range": "± 31101984",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 324483243,
            "range": "± 8993117",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "a0eb54cc5adb4f003f67dd45fa7a700eee530373",
          "message": "ci: fix deprecation warnings by using updated fork of unmaintained action",
          "timestamp": "2023-06-07T13:01:10+02:00",
          "tree_id": "26db95a43b87e0d48791ffc76cdd0ddae4073f90",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a0eb54cc5adb4f003f67dd45fa7a700eee530373"
        },
        "date": 1686137675874,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9397649,
            "range": "± 2438457",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173661929,
            "range": "± 1530485",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3176891487,
            "range": "± 72778276",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 280723404,
            "range": "± 8776664",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "55b2880a60f161bcdc88fad57f5157b06065fd31",
          "message": "ci: fix workflow deprecation warning for benchmark job",
          "timestamp": "2023-06-07T13:32:08+02:00",
          "tree_id": "9462c1008d364f921d5c07510be29f0d9814a0b0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/55b2880a60f161bcdc88fad57f5157b06065fd31"
        },
        "date": 1686139813699,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11486467,
            "range": "± 541292",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 208300319,
            "range": "± 2178533",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3585029069,
            "range": "± 63486941",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 340519483,
            "range": "± 8584651",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7c9e5020349bee186d1e3f93e3d66d03b476b8b7",
          "message": "chore(deps): update dependency urllib3 to v2.0.3",
          "timestamp": "2023-06-07T11:32:39Z",
          "tree_id": "974b5b4f917c3e23821fb9645d6c7d0d128c9bc2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7c9e5020349bee186d1e3f93e3d66d03b476b8b7"
        },
        "date": 1686153486455,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9103497,
            "range": "± 161605",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169788700,
            "range": "± 3241509",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2971304368,
            "range": "± 45541566",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 274975904,
            "range": "± 8551162",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "59742d12c6c6145c31348f1851d28e38916dec1c",
          "message": "fix(deps): update rust crate sysinfo to 0.29.2",
          "timestamp": "2023-06-07T19:59:37Z",
          "tree_id": "9671cf1a111b35505d4dcbb301356513d66b94b1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/59742d12c6c6145c31348f1851d28e38916dec1c"
        },
        "date": 1686178335683,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10897433,
            "range": "± 459891",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 205912811,
            "range": "± 2228508",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3625414878,
            "range": "± 38796545",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 331851120,
            "range": "± 9903197",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}