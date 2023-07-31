window.BENCHMARK_DATA = {
  "lastUpdate": 1690793452057,
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
          "id": "5d0ce1d63359b6d4e0345b628d4d2a918209f127",
          "message": "tweak: disable `downsize_if_single_color` by default\n\nWhile a good idea in theory, the experience with v0.4.0 has shown that\nmore packs are broken by this optimization than helped. Let's disable it\nby default for now, but with a disposition to apply it in a smarter way\nin the future, once the necessary refactors to detect how the image is\nused by the game are in place.",
          "timestamp": "2023-07-03T18:55:35+02:00",
          "tree_id": "fca4f29608785745d64640c827248dfec85f35b3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5d0ce1d63359b6d4e0345b628d4d2a918209f127"
        },
        "date": 1688406319657,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8309610,
            "range": "± 2733095",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173089066,
            "range": "± 7422689",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2751874945,
            "range": "± 34550985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 251996009,
            "range": "± 3575345",
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
          "id": "9abbcfd7145a4d3f6a05729eee36e99f6ee1bca0",
          "message": "fix(deps): update rust crate serde to 1.0.166",
          "timestamp": "2023-07-04T00:59:27Z",
          "tree_id": "f2e849292cb6480ecda593f54ee5349b3448aceb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9abbcfd7145a4d3f6a05729eee36e99f6ee1bca0"
        },
        "date": 1688435691766,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11169986,
            "range": "± 2583789",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 213474979,
            "range": "± 3062233",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3676305220,
            "range": "± 127533612",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 325730449,
            "range": "± 4470161",
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
          "id": "59e74cfcf670f7f01f3a9445c39c5be301ea0e88",
          "message": "fix(deps): update rust crate sysinfo to 0.29.4",
          "timestamp": "2023-07-04T01:03:33Z",
          "tree_id": "90c5235d835a12163bba268dfe6c7d40845afe8f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/59e74cfcf670f7f01f3a9445c39c5be301ea0e88"
        },
        "date": 1688446157428,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9287013,
            "range": "± 321746",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 179923850,
            "range": "± 9564898",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3126460024,
            "range": "± 25402144",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266943515,
            "range": "± 5692782",
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
          "id": "301a3f17991586f99ad578ba9b2a9f1fe06dd2b6",
          "message": "fix(deps): update rust crate rlimit to 0.10.0",
          "timestamp": "2023-07-04T08:27:14Z",
          "tree_id": "b0392e6b96e20c17672cadf4df96d248deb06c4a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/301a3f17991586f99ad578ba9b2a9f1fe06dd2b6"
        },
        "date": 1688464528091,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12194711,
            "range": "± 621738",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 218908403,
            "range": "± 8095926",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3266179246,
            "range": "± 132302614",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 326440976,
            "range": "± 4588114",
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
          "id": "c0e876a6e2877f1f78e720b4cea698890ae5a826",
          "message": "fix(deps): update rust crate thiserror to 1.0.41",
          "timestamp": "2023-07-04T19:46:08Z",
          "tree_id": "299c6e4a137514c02b2bd71df08d9f43a069a256",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c0e876a6e2877f1f78e720b4cea698890ae5a826"
        },
        "date": 1688511901260,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12727477,
            "range": "± 1123121",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 225782006,
            "range": "± 7116561",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3445559673,
            "range": "± 61517311",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 334978793,
            "range": "± 3357831",
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
          "id": "1955c1939a819a2f1020218600d052d93a77d262",
          "message": "fix(deps): update rust crate serde_json to 1.0.100",
          "timestamp": "2023-07-04T22:36:07Z",
          "tree_id": "092e82ccefc414f9d0ad8714b0119737af484085",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1955c1939a819a2f1020218600d052d93a77d262"
        },
        "date": 1688518701779,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8364877,
            "range": "± 159690",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 174035511,
            "range": "± 1278581",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2742330582,
            "range": "± 38915033",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 249968937,
            "range": "± 4154287",
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
          "id": "17e87cffb3bf45199cd9b42975cbd7711ed537eb",
          "message": "fix(deps): update rust crate regex to 1.9.0",
          "timestamp": "2023-07-05T13:18:45Z",
          "tree_id": "eeccbb360ff3e53f119b80c4ed779d7196ef1815",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/17e87cffb3bf45199cd9b42975cbd7711ed537eb"
        },
        "date": 1688579937674,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11685841,
            "range": "± 640457",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 206531620,
            "range": "± 2060398",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3154767515,
            "range": "± 88328751",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 316336411,
            "range": "± 7935711",
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
          "id": "67ce0dc431171aa8e2184d7ae1892b0b348015a6",
          "message": "fix(deps): update rust crate serde_path_to_error to 0.1.13",
          "timestamp": "2023-07-05T17:23:04Z",
          "tree_id": "ec3d41e0629207aa4cfede7120984330db91aaff",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/67ce0dc431171aa8e2184d7ae1892b0b348015a6"
        },
        "date": 1688582833236,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9325304,
            "range": "± 115332",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210143968,
            "range": "± 931197",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2098074984,
            "range": "± 15771439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288539019,
            "range": "± 3126154",
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
          "id": "9355f821a71c3e54f569ca170e61fe86e6636764",
          "message": "fix(deps): update rust crate toml to 0.7.6",
          "timestamp": "2023-07-05T21:30:16Z",
          "tree_id": "7958a99db2ba1a09afe4bc2e5e48aec85361e051",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9355f821a71c3e54f569ca170e61fe86e6636764"
        },
        "date": 1688606351470,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8475867,
            "range": "± 80365",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 174128101,
            "range": "± 14335917",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2750782294,
            "range": "± 43150457",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 254342597,
            "range": "± 4523950",
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
          "id": "e6c445cac4179f7bd230c61e9288e35616d0defb",
          "message": "chore(deps): update rust crate pretty_assertions to 1.4.0",
          "timestamp": "2023-07-06T19:49:06Z",
          "tree_id": "b1692b9316dfd6558de09607cdcd64a7ca95d40c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e6c445cac4179f7bd230c61e9288e35616d0defb"
        },
        "date": 1688682965439,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9807204,
            "range": "± 255621",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210403986,
            "range": "± 1460899",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2051493238,
            "range": "± 15308870",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 289442890,
            "range": "± 3835617",
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
          "id": "877d41b252bfae9b612cc585100c7a390c5a9903",
          "message": "fix(deps): update rust crate thiserror to 1.0.43 (#236)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-07-07T18:27:30+02:00",
          "tree_id": "466de0a2fad7a728169dceaba1c87e134f7d8780",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/877d41b252bfae9b612cc585100c7a390c5a9903"
        },
        "date": 1688750159056,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8814292,
            "range": "± 99833",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204583308,
            "range": "± 2235677",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1925334318,
            "range": "± 8321823",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 275002884,
            "range": "± 2120339",
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
          "id": "82ecf945016e2283edce85397bfcd9d1a4f8aad5",
          "message": "fix(deps): update rust crate regex to 1.9.1",
          "timestamp": "2023-07-07T18:14:35Z",
          "tree_id": "8bd276dc50ea9be46881b8397f6bf99f12920b06",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/82ecf945016e2283edce85397bfcd9d1a4f8aad5"
        },
        "date": 1688767096403,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9359283,
            "range": "± 389635",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 209760231,
            "range": "± 839625",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2164119581,
            "range": "± 18275592",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288863355,
            "range": "± 2300261",
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
          "id": "92f0e3292e3665b41ee8fd91831193dfa0ff56af",
          "message": "chore(deps): update dependency charset-normalizer to v3.2.0",
          "timestamp": "2023-07-07T21:14:04Z",
          "tree_id": "50423ef1b9a2c36750851215e9b3730ea46df6f4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/92f0e3292e3665b41ee8fd91831193dfa0ff56af"
        },
        "date": 1688781804930,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12650960,
            "range": "± 906117",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 266054517,
            "range": "± 10149564",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2342441822,
            "range": "± 46461124",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 359452327,
            "range": "± 5889302",
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
          "id": "32ea244222325d87f9d3420e75824daa72c66f2a",
          "message": "fix(deps): update rust crate serde to 1.0.168",
          "timestamp": "2023-07-09T01:52:08Z",
          "tree_id": "3df7220a698bd0abf42c6a607a7ee3e5e80fbcc1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/32ea244222325d87f9d3420e75824daa72c66f2a"
        },
        "date": 1688878955119,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9325129,
            "range": "± 217930",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 212108117,
            "range": "± 1671215",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2109967534,
            "range": "± 15778633",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 287834439,
            "range": "± 2756396",
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
          "id": "d57301dd00fec034175be09bf207a9c49b4d27ec",
          "message": "chore(deps): update rust crate time to 0.3.23",
          "timestamp": "2023-07-09T07:56:36Z",
          "tree_id": "2eab6ea403db46a7800a124501cd526873d9708e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d57301dd00fec034175be09bf207a9c49b4d27ec"
        },
        "date": 1688903761483,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11919799,
            "range": "± 801366",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 244534438,
            "range": "± 8257757",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2111493361,
            "range": "± 30826128",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 322353759,
            "range": "± 3232569",
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
          "id": "aeb13f4caa6c3b6c08556ced88869921f5b19b81",
          "message": "fix(deps): update rust crate serde to 1.0.169",
          "timestamp": "2023-07-09T11:24:32Z",
          "tree_id": "5edef196d136e8b54f45f8fd6c103a3a5e31dc32",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aeb13f4caa6c3b6c08556ced88869921f5b19b81"
        },
        "date": 1688909978850,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9531103,
            "range": "± 1683985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210256415,
            "range": "± 1396174",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2062460404,
            "range": "± 7862409",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 291309155,
            "range": "± 4828742",
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
          "id": "fba0e383b133985824e857293dce231b597bf40e",
          "message": "fix(deps): update rust crate serde to 1.0.170",
          "timestamp": "2023-07-09T19:07:41Z",
          "tree_id": "c107d6407af59d561b055e6fcf2bdab1d5668504",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fba0e383b133985824e857293dce231b597bf40e"
        },
        "date": 1688944212401,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11803987,
            "range": "± 761011",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 257164451,
            "range": "± 8327666",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2493095709,
            "range": "± 40485907",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 353856629,
            "range": "± 4230259",
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
          "id": "297a3359b29142cd54090dd49efa30d4e75f66c8",
          "message": "fix(deps): update rust crate serde to 1.0.171",
          "timestamp": "2023-07-10T03:48:06Z",
          "tree_id": "2c16d78cecb32e9192f38ebc3c6b9cfc93b74294",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/297a3359b29142cd54090dd49efa30d4e75f66c8"
        },
        "date": 1688979050329,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9368136,
            "range": "± 253263",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 212179655,
            "range": "± 1752001",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2110558270,
            "range": "± 27912035",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 287498319,
            "range": "± 4696760",
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
          "id": "f5d7e27e3cc48aa13e96881fd171375fe89c5e0e",
          "message": "fix(deps): update rust crate serde_json to 1.0.102",
          "timestamp": "2023-07-12T04:43:16Z",
          "tree_id": "8ba2610176977598519263f7cc89ca2e98fc4c76",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f5d7e27e3cc48aa13e96881fd171375fe89c5e0e"
        },
        "date": 1689145115606,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8346769,
            "range": "± 91096",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 203147675,
            "range": "± 1397529",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1914787002,
            "range": "± 12994400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271676573,
            "range": "± 2826428",
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
          "id": "ac81a12e77239ae2742739fdd2f636a9c9fb53de",
          "message": "ci: continue fixing workflows",
          "timestamp": "2023-07-13T12:03:38+02:00",
          "tree_id": "bb464cfb9eec4b669a7ccae48f8a4cdf3a8c1706",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ac81a12e77239ae2742739fdd2f636a9c9fb53de"
        },
        "date": 1689244588721,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9392224,
            "range": "± 676668",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210298127,
            "range": "± 1272525",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2071692180,
            "range": "± 23395585",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288361817,
            "range": "± 4085006",
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
          "id": "ff2b2f25ae7cc0a02297db7fe02884ad99d86735",
          "message": "chore(deps): update rust crate tar to 0.4.39",
          "timestamp": "2023-07-13T14:39:39Z",
          "tree_id": "bc4e2b62d26dbe98d0e0a235216668a65e2ec9d9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ff2b2f25ae7cc0a02297db7fe02884ad99d86735"
        },
        "date": 1689264182452,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11823041,
            "range": "± 835925",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 259018942,
            "range": "± 10686176",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2727935324,
            "range": "± 52994292",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 358653788,
            "range": "± 4374536",
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
          "id": "0540e81d2b012fe53e36b1ad64a54017bb08e079",
          "message": "chore(scripts/manual_packsquash_test): bump default pack_format version",
          "timestamp": "2023-07-15T13:34:37+02:00",
          "tree_id": "b6486cb5273ddd9aeecaacbb136fd20e331b4de6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0540e81d2b012fe53e36b1ad64a54017bb08e079"
        },
        "date": 1689423041040,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11670961,
            "range": "± 668632",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 253632797,
            "range": "± 20890393",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2270684327,
            "range": "± 37070762",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 337619970,
            "range": "± 4221574",
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
          "id": "f33ed49b490dc8d2d7294a6b6a453242aa14c058",
          "message": "ci: attempt to fix Windows runs",
          "timestamp": "2023-07-16T01:04:20+02:00",
          "tree_id": "02120da2abebfe7597c833f0e529d17f343ea1e3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f33ed49b490dc8d2d7294a6b6a453242aa14c058"
        },
        "date": 1689465423006,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12168122,
            "range": "± 610273",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 254316884,
            "range": "± 8332242",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2254312747,
            "range": "± 24344239",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 341870099,
            "range": "± 3405646",
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
          "id": "f90f1e35a00fd585499ffe472c8f4092287cf645",
          "message": "ci: slightly optimize size of generated executables\n\nLet's drop useless panic backtrace logic from them.",
          "timestamp": "2023-07-16T02:05:37+02:00",
          "tree_id": "f4a0c50186b244b73c4177044172cff1b9e977f8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f90f1e35a00fd585499ffe472c8f4092287cf645"
        },
        "date": 1689468760336,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8460913,
            "range": "± 63400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202838709,
            "range": "± 1484476",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1913502739,
            "range": "± 15143965",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 270595967,
            "range": "± 1778971",
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
          "id": "40aef65189b29b429da86a72bbdbbcf84fdb4495",
          "message": "ci: minor fixups",
          "timestamp": "2023-07-16T20:06:31+02:00",
          "tree_id": "501e67efa621203a8c9bdf20726ceaa70f9a09f0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/40aef65189b29b429da86a72bbdbbcf84fdb4495"
        },
        "date": 1689534079112,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9621088,
            "range": "± 581212",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 213413140,
            "range": "± 1546191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2119641556,
            "range": "± 12729062",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 290031149,
            "range": "± 1681354",
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
          "id": "4135360bdb99118f76339db98fa81f6ac5dfdc29",
          "message": "fix(deps): update rust crate sysinfo to 0.29.5",
          "timestamp": "2023-07-16T19:11:19Z",
          "tree_id": "dddd34c4c01c1f4549c94f5e0c216ee9f0b59596",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4135360bdb99118f76339db98fa81f6ac5dfdc29"
        },
        "date": 1689542173511,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9377153,
            "range": "± 818815",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210204185,
            "range": "± 1252121",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2105768603,
            "range": "± 15614114",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 289804621,
            "range": "± 1884800",
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
          "id": "988368aa95ce7a2b335f8213b7ffa98a1f7d3c81",
          "message": "ci: skip ditherbomb tests for AArch64 on CI\n\nQEMU emulation imposes a noticeable performance hit that causes our CI\ntimes to be a lot longer for little benefit.",
          "timestamp": "2023-07-17T10:40:13+02:00",
          "tree_id": "b71c789aa899f038ae847c67ef5d5c8e6f34ef5c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/988368aa95ce7a2b335f8213b7ffa98a1f7d3c81"
        },
        "date": 1689585321354,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9479328,
            "range": "± 185141",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210854966,
            "range": "± 1682004",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2053964015,
            "range": "± 12548803",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288984689,
            "range": "± 3549044",
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
          "id": "0c1807a53e3ea1e63378b1a08dc157e498e32fb8",
          "message": "fix(deps): update rust crate uuid to 1.4.1",
          "timestamp": "2023-07-17T08:42:07Z",
          "tree_id": "05600f871e8ea4433e23f79fd393c65e682d302e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0c1807a53e3ea1e63378b1a08dc157e498e32fb8"
        },
        "date": 1689593076255,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11822170,
            "range": "± 368664",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 262241180,
            "range": "± 7432940",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2587724632,
            "range": "± 16215380",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 359676790,
            "range": "± 3208009",
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
          "id": "019e8e41befe1fc8d65af156e16f09d94b2f4268",
          "message": "ci: exclude Dependency lists from CLI artifact uploads\n\nThese are obviously not necessary and might cause the GH Action to fail.",
          "timestamp": "2023-07-17T23:13:12+02:00",
          "tree_id": "2b546cbdb5f0cdbf97f05cbd6cf6625b4de8a853",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/019e8e41befe1fc8d65af156e16f09d94b2f4268"
        },
        "date": 1689629897826,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8544130,
            "range": "± 94846",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202392816,
            "range": "± 991082",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1914394811,
            "range": "± 13993447",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 274587443,
            "range": "± 2744570",
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
          "id": "d3c589390f62b9c53aeffbda119a5e42e2a0ac9d",
          "message": "ci: change build workflow name to be prettier",
          "timestamp": "2023-07-18T14:44:11+02:00",
          "tree_id": "85925f02192cc0cd8f7985f351b6c70aae7e47ae",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3c589390f62b9c53aeffbda119a5e42e2a0ac9d"
        },
        "date": 1689686456953,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11059947,
            "range": "± 568210",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 254984228,
            "range": "± 9201803",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2207432308,
            "range": "± 104367915",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 348081967,
            "range": "± 14415323",
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
          "id": "5935f7321ca6929b43f4ab8586aa72271530fc05",
          "message": "chore(deps): update dependency urllib3 to v2.0.4",
          "timestamp": "2023-07-19T16:21:58Z",
          "tree_id": "6e0ae507ce09dd2dd3878d06cbba983086a4bbe2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5935f7321ca6929b43f4ab8586aa72271530fc05"
        },
        "date": 1689797825695,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13381369,
            "range": "± 826026",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 264798321,
            "range": "± 12796903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2410860744,
            "range": "± 21433377",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 368943106,
            "range": "± 5070526",
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
          "id": "263fd6a70b30d24f462ae628dcab3af33caa1008",
          "message": "fix(deps): update rust crate rlimit to 0.10.1",
          "timestamp": "2023-07-19T19:41:36Z",
          "tree_id": "80d0305621a2579bbc6020563aff135a2de60a65",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/263fd6a70b30d24f462ae628dcab3af33caa1008"
        },
        "date": 1689805065561,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9390043,
            "range": "± 224445",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 210588059,
            "range": "± 1339985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2078942956,
            "range": "± 13868214",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 286387552,
            "range": "± 2527713",
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
          "id": "580bbcd5f95e8da1c7c57753f786ce057d96fd29",
          "message": "fix(deps): update rust crate serde to 1.0.173",
          "timestamp": "2023-07-20T01:55:05Z",
          "tree_id": "162d3435706f71fee2093efcb1d758f95f1939d9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/580bbcd5f95e8da1c7c57753f786ce057d96fd29"
        },
        "date": 1689828208832,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9954669,
            "range": "± 513028",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 230898746,
            "range": "± 4503936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2360016596,
            "range": "± 77075781",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 307623148,
            "range": "± 13029892",
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
          "id": "2146593f96c64a35323e333a25d88bf1b16ec925",
          "message": "fix(deps): update rust crate tempfile to 3.7.0",
          "timestamp": "2023-07-20T19:33:26Z",
          "tree_id": "e1e9b9ec6ad4ca7955d35f2aa5b4f85ab8c7fe68",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2146593f96c64a35323e333a25d88bf1b16ec925"
        },
        "date": 1689888774352,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9981865,
            "range": "± 366407",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 209975621,
            "range": "± 1514617",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2070699935,
            "range": "± 12927101",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 290937303,
            "range": "± 3093243",
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
          "id": "3da8d5ec347f8f5d2b38e69a17f7c25e706b6ffb",
          "message": "fix(deps): update rust crate serde to 1.0.174",
          "timestamp": "2023-07-21T06:22:21Z",
          "tree_id": "c92e8ddb4b012cd325497e3eaca8b2d5768389fa",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3da8d5ec347f8f5d2b38e69a17f7c25e706b6ffb"
        },
        "date": 1689925859787,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8552939,
            "range": "± 419448",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204862537,
            "range": "± 1587363",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1930710616,
            "range": "± 4033622",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 274591762,
            "range": "± 6995594",
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
          "id": "ce750df66f6d10890cead67ef933d464ff33d6a4",
          "message": "fix(deps): update rust crate thiserror to 1.0.44",
          "timestamp": "2023-07-21T07:18:32Z",
          "tree_id": "e3b83ec7c063730a56ffbfd060e27fda1520f46c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ce750df66f6d10890cead67ef933d464ff33d6a4"
        },
        "date": 1689928165308,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11719843,
            "range": "± 818964",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 255905290,
            "range": "± 7201499",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2621876362,
            "range": "± 33211127",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 358854210,
            "range": "± 6063029",
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
          "id": "1c775e7e86c5c5a46867ce9c4750893fcb2b3556",
          "message": "chore(deps): update dependency certifi to v2023.7.22",
          "timestamp": "2023-07-22T09:16:16Z",
          "tree_id": "da9c8519a08857bde3b7d996eaf18ddb5556723e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1c775e7e86c5c5a46867ce9c4750893fcb2b3556"
        },
        "date": 1690031720118,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8537760,
            "range": "± 165180",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 203854668,
            "range": "± 1483337",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1900861919,
            "range": "± 6103960",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 267987878,
            "range": "± 4150031",
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
          "id": "c2c553c10abdc4928a8507d95612e0e81f799444",
          "message": "fix(deps): update rust crate sysinfo to 0.29.6",
          "timestamp": "2023-07-22T12:42:23Z",
          "tree_id": "8fcbb06f7ac8b5e7e4c4da792a9ef1983791376f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c2c553c10abdc4928a8507d95612e0e81f799444"
        },
        "date": 1690035216688,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13226400,
            "range": "± 805700",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 267046370,
            "range": "± 10287453",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2474184243,
            "range": "± 33617446",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 374723671,
            "range": "± 3834653",
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
          "id": "667b281161ee486d5e7b3e46bc416f1623ecd8b0",
          "message": "chore(deps): update rust crate winresource to 0.1.16",
          "timestamp": "2023-07-22T13:41:16Z",
          "tree_id": "1ad561da41ae29422293dfba2bc65f2d234aa674",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/667b281161ee486d5e7b3e46bc416f1623ecd8b0"
        },
        "date": 1690037758810,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11840874,
            "range": "± 1514425",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 240508428,
            "range": "± 2930232",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2244583605,
            "range": "± 20517208",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 332441367,
            "range": "± 3529484",
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
          "id": "d21481ba27215f1c7e2961672ccefe7dc3459ccf",
          "message": "fix(deps): update rust crate serde to 1.0.175",
          "timestamp": "2023-07-24T03:51:18Z",
          "tree_id": "7f874b0bc0fa5c68509d36bf38899ed9bf3030da",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d21481ba27215f1c7e2961672ccefe7dc3459ccf"
        },
        "date": 1690188939304,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10704013,
            "range": "± 8332475",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 226770221,
            "range": "± 2134024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1908985099,
            "range": "± 26166413",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 291923398,
            "range": "± 4535316",
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
          "id": "cf3f8ddb44cfdea71142695adc05cc2284f3d164",
          "message": "fix(deps): update rust crate patricia_tree to 0.6.2",
          "timestamp": "2023-07-24T17:06:31Z",
          "tree_id": "d466652473de9511e8c28b3af56cc96375854aee",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cf3f8ddb44cfdea71142695adc05cc2284f3d164"
        },
        "date": 1690224969322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11799273,
            "range": "± 613494",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 255627302,
            "range": "± 8669609",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2696222286,
            "range": "± 32041066",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 348618840,
            "range": "± 4205791",
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
          "id": "ebc24aa20e35750027d939e528dc3b1c3d3f5b2c",
          "message": "fix(deps): update rust crate serde to 1.0.176",
          "timestamp": "2023-07-26T21:56:12Z",
          "tree_id": "1424af5f257d802e2c4b6f7cefecab8d5510c4f6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ebc24aa20e35750027d939e528dc3b1c3d3f5b2c"
        },
        "date": 1690425076180,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8407152,
            "range": "± 379326",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202034332,
            "range": "± 1322984",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1886298652,
            "range": "± 8410923",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 268062471,
            "range": "± 2292449",
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
          "id": "13fdc753b6da3fed1e645ec6f9f03ea3a48b0a94",
          "message": "fix(deps): update rust crate globset to 0.4.12",
          "timestamp": "2023-07-27T01:57:34Z",
          "tree_id": "6ee70d8461f115f4ddbb16de66159f53c273369c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/13fdc753b6da3fed1e645ec6f9f03ea3a48b0a94"
        },
        "date": 1690433816070,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8566254,
            "range": "± 84316",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202375486,
            "range": "± 1102350",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1896317053,
            "range": "± 16915040",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 268562985,
            "range": "± 2798766",
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
          "id": "c4949a5b0e8c78c65e7c57bbd571760b7892fab2",
          "message": "fix(deps): update rust crate serde_json to 1.0.104",
          "timestamp": "2023-07-27T04:27:02Z",
          "tree_id": "7518e587054f67dbd2d08331920b6d8eddfe2a83",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c4949a5b0e8c78c65e7c57bbd571760b7892fab2"
        },
        "date": 1690440153962,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8442739,
            "range": "± 7104936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 201964877,
            "range": "± 2440242",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1900928510,
            "range": "± 11189884",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271370463,
            "range": "± 2035798",
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
          "id": "750dee3553f1514b54ca2edd217c0b2201476d49",
          "message": "fix(deps): update rust crate sysinfo to 0.29.7",
          "timestamp": "2023-07-27T06:13:19Z",
          "tree_id": "44f777a79df24bb75663aa47ab72a72e0ebf0aef",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/750dee3553f1514b54ca2edd217c0b2201476d49"
        },
        "date": 1690452593869,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8716997,
            "range": "± 258213",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204005834,
            "range": "± 1019314",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1924841868,
            "range": "± 4264024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 272353111,
            "range": "± 1732683",
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
          "id": "d4f6c1a6210e694556c6f49c5a2bcbc5c9e6ffe2",
          "message": "fix(deps): update rust crate serde to 1.0.177",
          "timestamp": "2023-07-27T21:44:52Z",
          "tree_id": "a3d11e31bb7ab19c8206a0377578676e53eaa218",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d4f6c1a6210e694556c6f49c5a2bcbc5c9e6ffe2"
        },
        "date": 1690509978308,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9208725,
            "range": "± 364182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 207476966,
            "range": "± 3410743",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1762491687,
            "range": "± 17781980",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 264533959,
            "range": "± 4166512",
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
          "id": "6590f6faab40332763b2193bfafe169548f8a1c8",
          "message": "fix(deps): update rust crate serde to 1.0.178",
          "timestamp": "2023-07-29T00:26:31Z",
          "tree_id": "250157baabefb7c7e67ff7f716cefc3af7b001d9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6590f6faab40332763b2193bfafe169548f8a1c8"
        },
        "date": 1690606006722,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9376946,
            "range": "± 136812",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 209809557,
            "range": "± 1667287",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2048172011,
            "range": "± 12594137",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288274915,
            "range": "± 2014188",
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
          "id": "64cbe897f95dcb260dcf4ecd7b542a8f7ebd1ad0",
          "message": "chore(deps): update `vorbis_rs` to 0.4.0",
          "timestamp": "2023-07-29T23:18:27+02:00",
          "tree_id": "d310a8ba844e7ccb4c725285fd0068e01668113d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/64cbe897f95dcb260dcf4ecd7b542a8f7ebd1ad0"
        },
        "date": 1690667592722,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8598908,
            "range": "± 140225",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204001559,
            "range": "± 926286",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1904202073,
            "range": "± 18939708",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 272091024,
            "range": "± 2716456",
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
          "id": "eee2816ca2f5e3c609a850f1e0d67c083c87cce0",
          "message": "chore(deps): update rust crate time to 0.3.24",
          "timestamp": "2023-07-31T01:20:26Z",
          "tree_id": "f0cc68f8a037608c1cb79683d1af91fcc87a8bc7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eee2816ca2f5e3c609a850f1e0d67c083c87cce0"
        },
        "date": 1690781121653,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8711447,
            "range": "± 119422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 201986736,
            "range": "± 1030761",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 1894053741,
            "range": "± 12090387",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271464253,
            "range": "± 2261931",
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
          "id": "4b3d081114aaa9387ccd24fdc405bc6034c7eb16",
          "message": "fix(deps): update rust crate serde to 1.0.179",
          "timestamp": "2023-07-31T04:53:14Z",
          "tree_id": "cc0a7692cc1aa2d1375d30438a31927f1a4b2f63",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4b3d081114aaa9387ccd24fdc405bc6034c7eb16"
        },
        "date": 1690793451350,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9323308,
            "range": "± 136567",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 208574823,
            "range": "± 984530",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2068003878,
            "range": "± 16363059",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 288138547,
            "range": "± 2987425",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}