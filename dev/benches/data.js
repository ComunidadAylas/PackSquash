window.BENCHMARK_DATA = {
  "lastUpdate": 1696061234663,
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
          "id": "7c4c2f18209de1020539746e82ca7bf74764c120",
          "message": "chore(deps): update taiki-e/install-action digest to b89cfc4",
          "timestamp": "2023-09-10T07:47:35Z",
          "tree_id": "a7e0532a4a88ea007b51c806577e070d3a689edb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7c4c2f18209de1020539746e82ca7bf74764c120"
        },
        "date": 1694344332155,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11058544,
            "range": "± 201315",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 206166164,
            "range": "± 3690373",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3592272190,
            "range": "± 60609569",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 316015498,
            "range": "± 4012934",
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
          "id": "b440820a442d3427084096033221515d962697f9",
          "message": "fix(deps): update rust crate imagequant to 4.2.1",
          "timestamp": "2023-09-11T00:42:57Z",
          "tree_id": "86611e34fe3e257d897f0dfd8e0301161fb849ff",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b440820a442d3427084096033221515d962697f9"
        },
        "date": 1694409911841,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8609906,
            "range": "± 172701",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157189436,
            "range": "± 1731857",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2715858656,
            "range": "± 25631155",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 241245505,
            "range": "± 2996702",
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
          "id": "ff0981f135bbd537f714045f047e929997da95d1",
          "message": "chore(deps): update taiki-e/install-action digest to 9afdc87",
          "timestamp": "2023-09-11T13:05:01Z",
          "tree_id": "52b9c17c97aafe5936a359d41f1cc775772895d4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ff0981f135bbd537f714045f047e929997da95d1"
        },
        "date": 1694446905077,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9232306,
            "range": "± 259945",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 170619215,
            "range": "± 14641542",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3262414945,
            "range": "± 52521710",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 261622229,
            "range": "± 2457583",
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
          "id": "d3c3e13f6e8e5348f5409d9fa0f2d3b04215771e",
          "message": "refactor(properties_file): simplify, standarize on Windows-1252 encoding\n\nThe Java documentation specifies that .properties files should use the\nWindows-1252 (also imprecisely known as ISO-8859-1) encoding, but there\nwas code for supporting the Unicode BOM, which should not appear on the\nfirst place and does not help with incompatible misinterpretations of\nthe source encoding as UTF-8; it'd only help if a UTF-8 with BOM file\nonly uses ASCII characters, but break in other cases.\n\nIn the future, we might study and implement ways of handling several\nencodings, depending on the encoding expected by the mod using these\nfiles.",
          "timestamp": "2023-09-11T21:22:37+02:00",
          "tree_id": "5dfb46497166e6ee7e05831468cd1360605ab029",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3c3e13f6e8e5348f5409d9fa0f2d3b04215771e"
        },
        "date": 1694461927209,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9418391,
            "range": "± 179838",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 172061743,
            "range": "± 1654214",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3045057505,
            "range": "± 54966918",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 265867836,
            "range": "± 3601289",
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
          "id": "fe6f026063d606b0b84363a7016c7491748cb0c2",
          "message": "chore(deps): update taiki-e/install-action digest to 63c295a",
          "timestamp": "2023-09-11T23:23:58Z",
          "tree_id": "6a3af7fd2117721543d8ceabf6b588901ba0a755",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fe6f026063d606b0b84363a7016c7491748cb0c2"
        },
        "date": 1694485353254,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8496630,
            "range": "± 165124",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157681162,
            "range": "± 7243710",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2726426226,
            "range": "± 31095359",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245989973,
            "range": "± 2595922",
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
          "id": "9dbbd902bc1b3ca2cc0039d0f306fb11c374f570",
          "message": "chore(deps): update docker/build-push-action action to v5 (#249)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T11:22:26+02:00",
          "tree_id": "51f83f7e4daecb9fc9390aa0b8211a852efd70f1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9dbbd902bc1b3ca2cc0039d0f306fb11c374f570"
        },
        "date": 1694512803230,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8482274,
            "range": "± 301585",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157842017,
            "range": "± 10978130",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2708190589,
            "range": "± 40259720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 247912694,
            "range": "± 3112067",
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
          "id": "cb5a0a4ba11c2002d6cb5b330b87e1e4e9ac923d",
          "message": "chore(deps): update docker/login-action action to v3 (#250)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T11:23:44+02:00",
          "tree_id": "c6d9b4a5d6c2686be51db44a8b7fe991696290ab",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cb5a0a4ba11c2002d6cb5b330b87e1e4e9ac923d"
        },
        "date": 1694513144194,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8549108,
            "range": "± 109956",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156826305,
            "range": "± 9165108",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2712534146,
            "range": "± 26322149",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 246208811,
            "range": "± 2431997",
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
          "id": "a9c37e3d90c01cc2a2cd7c54b371631709d9d22f",
          "message": "chore(deps): update docker/metadata-action action to v5 (#251)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T11:24:18+02:00",
          "tree_id": "c512866daebb5349a63b337604cf1681c1a7245a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a9c37e3d90c01cc2a2cd7c54b371631709d9d22f"
        },
        "date": 1694513524178,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8583485,
            "range": "± 258501",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 158638536,
            "range": "± 15740486",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2727095698,
            "range": "± 23943300",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 248313073,
            "range": "± 1935193",
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
          "id": "dd8d026c36736a3efd5e3bfc56321326c77789bc",
          "message": "chore(deps): update taiki-e/install-action digest to 0163f6c (#248)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T13:19:04+02:00",
          "tree_id": "2c557f6dafd42676ff079552e74028628fb9d072",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/dd8d026c36736a3efd5e3bfc56321326c77789bc"
        },
        "date": 1694519190645,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11146199,
            "range": "± 319496",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 203156533,
            "range": "± 4451017",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3638800912,
            "range": "± 47267150",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 317125061,
            "range": "± 7048630",
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
          "id": "e2f356b1773e97976557ade42fcca953a240cce8",
          "message": "chore(deps): update docker/setup-buildx-action action to v3 (#252)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-12T13:21:21+02:00",
          "tree_id": "780f4a1e03532b2a90a7daa88504aa0b09613972",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e2f356b1773e97976557ade42fcca953a240cce8"
        },
        "date": 1694519583487,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9373994,
            "range": "± 598739",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169396947,
            "range": "± 6272330",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3028196272,
            "range": "± 32320625",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 262855269,
            "range": "± 1881572",
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
          "id": "b4cd17cd4619c54b1ab86bfeb1a6216ce6b04434",
          "message": "chore(deps): update swatinem/rust-cache digest to a95ba19",
          "timestamp": "2023-09-12T18:58:46Z",
          "tree_id": "e9818eeece6363863b19633582d5af8ed6c34e6a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b4cd17cd4619c54b1ab86bfeb1a6216ce6b04434"
        },
        "date": 1694562057329,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8398675,
            "range": "± 225632",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157758032,
            "range": "± 14119651",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2704066528,
            "range": "± 28371985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 247208355,
            "range": "± 2205985",
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
          "id": "a287e03afcdae9fef62f48335764bbabbc71f03f",
          "message": "fix(deps): update rust crate toml to 0.8.0",
          "timestamp": "2023-09-13T04:39:21Z",
          "tree_id": "82f1ff2cfdddfb5129bcf65fd4c5f60fe1551e61",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a287e03afcdae9fef62f48335764bbabbc71f03f"
        },
        "date": 1694596175886,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8413723,
            "range": "± 99424",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156982241,
            "range": "± 15816090",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2711159150,
            "range": "± 23053034",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 246709141,
            "range": "± 14073089",
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
          "id": "20d8148e9fc97cc6b43e659fcbf650b24805cc95",
          "message": "chore(deps): update dependency filelock to v3.12.4",
          "timestamp": "2023-09-13T19:05:41Z",
          "tree_id": "9755e6984e4ceef9fde72570e799f33e3fa3480d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/20d8148e9fc97cc6b43e659fcbf650b24805cc95"
        },
        "date": 1694643623087,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9696299,
            "range": "± 106237",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 168462223,
            "range": "± 2008128",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3039597006,
            "range": "± 37241596",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266220333,
            "range": "± 2703739",
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
          "id": "98c0ece69aaf2c176d343f542d3d0a4b62efbed3",
          "message": "fix(deps): update rust crate serde_json to 1.0.107",
          "timestamp": "2023-09-14T00:14:10Z",
          "tree_id": "fbc515de3edf0017c0f880b09f77d8817dfa928b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/98c0ece69aaf2c176d343f542d3d0a4b62efbed3"
        },
        "date": 1694666067015,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11000198,
            "range": "± 1343903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 194117981,
            "range": "± 2557380",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3601016628,
            "range": "± 71818588",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 318242633,
            "range": "± 13774911",
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
          "id": "e22a4c6fa83e04db15073df158d5ec19ef5166e0",
          "message": "chore(deps): update taiki-e/install-action digest to 09f5ee3",
          "timestamp": "2023-09-15T03:20:05Z",
          "tree_id": "4cf361553aa9e592bfa9aa1eae4a3e93c1a752de",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e22a4c6fa83e04db15073df158d5ec19ef5166e0"
        },
        "date": 1694764815928,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9388603,
            "range": "± 6021816",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 161808661,
            "range": "± 1542626",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2724800367,
            "range": "± 73951549",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 270320477,
            "range": "± 4075857",
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
          "id": "e854698450bf1a6192c8dd8eb0e420e7d716d3d5",
          "message": "chore(deps): update taiki-e/install-action digest to 1c8b6e3",
          "timestamp": "2023-09-15T18:08:58Z",
          "tree_id": "407903989ab5179fd4db4f4199f85620a8266230",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e854698450bf1a6192c8dd8eb0e420e7d716d3d5"
        },
        "date": 1694817909377,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10967528,
            "range": "± 297884",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 186478764,
            "range": "± 4048573",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3166509718,
            "range": "± 84607317",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 309675474,
            "range": "± 6032818",
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
          "id": "4e50f9b3f86a3df28856f3b7d4d9f42c545a3fbc",
          "message": "build: addendum to 2a161e044da8e5da1bd4bc4d90cacef8ba1cedd7",
          "timestamp": "2023-09-17T15:18:55+02:00",
          "tree_id": "6cd9ab91bf5b4d4ea64ce9940dbcb2f5b03707af",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4e50f9b3f86a3df28856f3b7d4d9f42c545a3fbc"
        },
        "date": 1694958820586,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8447692,
            "range": "± 133546",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 151902200,
            "range": "± 14639494",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2382009528,
            "range": "± 52554031",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 251877817,
            "range": "± 6657456",
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
          "id": "8b6a08e3f288ef386068cd18ef656b5bad9b4b18",
          "message": "docs(CHANGELOG): typo fix, thank reporter of musl performance issues",
          "timestamp": "2023-09-18T00:49:41+02:00",
          "tree_id": "0b6b40ded887465b81517ccd5dd6e54e75a3f149",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8b6a08e3f288ef386068cd18ef656b5bad9b4b18"
        },
        "date": 1694992646009,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8639977,
            "range": "± 133075",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 154894399,
            "range": "± 1363769",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2432413560,
            "range": "± 52275189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 252055111,
            "range": "± 3857414",
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
          "id": "23e0b8755cdad9679fb7b142324374822a815298",
          "message": "chore(deps): update dtolnay/rust-toolchain digest to 1482605",
          "timestamp": "2023-09-17T23:06:01Z",
          "tree_id": "5007f3652e737151ede891a5590bfbf94165498c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/23e0b8755cdad9679fb7b142324374822a815298"
        },
        "date": 1694998614873,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8683684,
            "range": "± 334595",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 154244724,
            "range": "± 11285424",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2435736503,
            "range": "± 53854948",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 252516035,
            "range": "± 4231557",
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
          "id": "66d279fde0a7e9aa6e45960a02728c494a387d93",
          "message": "ci: slightly simplify workflow definition\n\nIt turns out that omitting matrix keys altogether should make the CI\njobs execute as we want, while leading to shorter workflow definition\nYAML. Related docs: https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs#example-adding-configurations",
          "timestamp": "2023-09-18T21:09:11+02:00",
          "tree_id": "60f8444c9d591f3cc1b62830fba606b0c9e598de",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/66d279fde0a7e9aa6e45960a02728c494a387d93"
        },
        "date": 1695066113322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9125309,
            "range": "± 148611",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 159899248,
            "range": "± 13175597",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2736779602,
            "range": "± 66392033",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 268558834,
            "range": "± 8235975",
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
          "id": "c1886441c796557bafcb3cdc747ad6147ff2c2d4",
          "message": "Revert \"ci: slightly simplify workflow definition\"\n\nThis reverts commit 66d279fde0a7e9aa6e45960a02728c494a387d93.\n\nWhile the change worked, it lead to workflows showing a different and\nuglier name on the GitHub web UI, which was not worth it when compared\nto the DRY benefits.",
          "timestamp": "2023-09-18T21:10:02+02:00",
          "tree_id": "5007f3652e737151ede891a5590bfbf94165498c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c1886441c796557bafcb3cdc747ad6147ff2c2d4"
        },
        "date": 1695066879664,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12815395,
            "range": "± 493267",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 198897744,
            "range": "± 2158351",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2930680784,
            "range": "± 61591238",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 329158473,
            "range": "± 10391630",
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
          "id": "79bc670130c62ccde72fa149b99f9965a79ba1ab",
          "message": "chore(deps): update taiki-e/install-action digest to 2358ab6",
          "timestamp": "2023-09-18T21:28:57Z",
          "tree_id": "ff4910f44be929e97a0b3c758ffe3232d071bd61",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/79bc670130c62ccde72fa149b99f9965a79ba1ab"
        },
        "date": 1695074317059,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11668543,
            "range": "± 800851",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204539326,
            "range": "± 5225863",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3367257079,
            "range": "± 91387093",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 337488644,
            "range": "± 6444812",
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
          "id": "40e5dbeda3b6cdb5a9902f2983fc97dee4ba9615",
          "message": "fix(deps): update rust crate aho-corasick to 1.1.0",
          "timestamp": "2023-09-18T21:29:41Z",
          "tree_id": "58da962a111fdb4b799a250b8234d68bdd94503a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/40e5dbeda3b6cdb5a9902f2983fc97dee4ba9615"
        },
        "date": 1695083679181,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9484431,
            "range": "± 211113",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 165323064,
            "range": "± 4941881",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2758860164,
            "range": "± 64311877",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 269471304,
            "range": "± 14725128",
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
          "id": "210473dbd37f74a8a59c64dce4cddd3fc5825a48",
          "message": "chore(deps): update debian:bullseye-slim docker digest to 40c0654",
          "timestamp": "2023-09-20T03:51:31Z",
          "tree_id": "be214f3f780f413b1eeaf6875ff5342a4d4c2629",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/210473dbd37f74a8a59c64dce4cddd3fc5825a48"
        },
        "date": 1695192810605,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9119411,
            "range": "± 275942",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164238629,
            "range": "± 1407560",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3031131397,
            "range": "± 78698069",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 263783106,
            "range": "± 7010241",
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
          "id": "3a34dcf31cefc3343ad4219801da0c4efcff50ee",
          "message": "chore(deps): update debian:bullseye-slim docker digest to c618be8",
          "timestamp": "2023-09-20T10:40:47Z",
          "tree_id": "db291527a6c8fae8bf80a7a965409c8bf785454b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3a34dcf31cefc3343ad4219801da0c4efcff50ee"
        },
        "date": 1695221361284,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8372176,
            "range": "± 83713",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 153609039,
            "range": "± 17738439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2435984888,
            "range": "± 54111362",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 250358699,
            "range": "± 5667248",
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
          "id": "dcd1e0dd4bdcf43827870dd42547303e119bde85",
          "message": "chore(deps): update dependency urllib3 to v2.0.5",
          "timestamp": "2023-09-20T14:23:37Z",
          "tree_id": "3ab2c937f4e6549ffcc9ec428184d3def5fcfc4a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/dcd1e0dd4bdcf43827870dd42547303e119bde85"
        },
        "date": 1695230133197,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12581093,
            "range": "± 1293887",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 196826692,
            "range": "± 2973576",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2970688108,
            "range": "± 77468936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 328056357,
            "range": "± 8568866",
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
          "id": "c2bbf9dcdb0afb9cd022a9e0d2b8b13f682ccc81",
          "message": "fix(deps): update rust crate aho-corasick to 1.1.1",
          "timestamp": "2023-09-20T16:44:38Z",
          "tree_id": "280a518518592e0c67d6478fad5e3479a9e23633",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c2bbf9dcdb0afb9cd022a9e0d2b8b13f682ccc81"
        },
        "date": 1695241455434,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12188180,
            "range": "± 780144",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 207730098,
            "range": "± 6717966",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3738053225,
            "range": "± 120556731",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 326612261,
            "range": "± 5671118",
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
          "id": "ef4e4a4b65e08e78850a7187cdc284d8607cb74e",
          "message": "fix(deps): update rust crate tokio-util to 0.7.9",
          "timestamp": "2023-09-20T19:52:48Z",
          "tree_id": "8ed0555d3a2cdb458777e66810bfeece8f269958",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ef4e4a4b65e08e78850a7187cdc284d8607cb74e"
        },
        "date": 1695246274892,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12749269,
            "range": "± 687836",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202544404,
            "range": "± 5306263",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2984224407,
            "range": "± 112183747",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 337757399,
            "range": "± 11230716",
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
          "id": "b1038486a8339499e393bab4a3580bdc3ceb2db6",
          "message": "fix(deps): update rust crate winapi-util to 0.1.6",
          "timestamp": "2023-09-20T21:16:16Z",
          "tree_id": "812fee984396917835b0e9f3e565254360437082",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b1038486a8339499e393bab4a3580bdc3ceb2db6"
        },
        "date": 1695256848899,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9492134,
            "range": "± 336437",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 168025678,
            "range": "± 18653982",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2897612021,
            "range": "± 58722871",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271022065,
            "range": "± 6896769",
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
          "id": "40845ee552451d03f7a9aebe661c29b82987586a",
          "message": "chore(deps): update taiki-e/install-action digest to 2afb713",
          "timestamp": "2023-09-21T03:07:28Z",
          "tree_id": "3ca7b8378620a3fe179e818aa19508f559fba628",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/40845ee552451d03f7a9aebe661c29b82987586a"
        },
        "date": 1695283745039,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9484046,
            "range": "± 88857",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164184052,
            "range": "± 2650789",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2735500636,
            "range": "± 86139439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 270628420,
            "range": "± 4716777",
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
          "id": "786db0ce22e2e0e155de74d80acc1cb143f3f81c",
          "message": "chore(deps): update taiki-e/install-action digest to 05acba8",
          "timestamp": "2023-09-22T12:45:25Z",
          "tree_id": "9e33ed57695345ac9e61d4c7c5d1db4f5a6afafb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/786db0ce22e2e0e155de74d80acc1cb143f3f81c"
        },
        "date": 1695400959208,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8464821,
            "range": "± 52911",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 152682599,
            "range": "± 10930723",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2436987532,
            "range": "± 40298535",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 253092841,
            "range": "± 7423645",
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
          "id": "ce17f4edd9f560ca80165fb9624c89ea20baf224",
          "message": "chore(deps): update actions/checkout digest to 8ade135",
          "timestamp": "2023-09-22T19:17:35Z",
          "tree_id": "d7a315074321e224de78385f4ec16484cbe45dff",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ce17f4edd9f560ca80165fb9624c89ea20baf224"
        },
        "date": 1695423827394,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9579838,
            "range": "± 272985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164773169,
            "range": "± 10849985",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3002085155,
            "range": "± 79677439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 269619108,
            "range": "± 6277589",
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
          "id": "aea7813cebdda80a164c9a87e33621043ea1e4be",
          "message": "chore(deps): update taiki-e/install-action digest to 5b205dd",
          "timestamp": "2023-09-23T13:56:34Z",
          "tree_id": "66329b373b26ab1607ff4e90fa59495814bced4d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aea7813cebdda80a164c9a87e33621043ea1e4be"
        },
        "date": 1695487057169,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8535795,
            "range": "± 100972",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 153467501,
            "range": "± 13782542",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2425624976,
            "range": "± 46789692",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 251857622,
            "range": "± 11810591",
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
          "id": "69a856ce6dc8bd99e4f86c349526cbeb3277f902",
          "message": "test: update unit tests for ea933d50675366d36d9e23af55e0c4e38d232388",
          "timestamp": "2023-09-24T19:21:02+02:00",
          "tree_id": "bcb5732030f25c92dc4b40181aae82f093b89230",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/69a856ce6dc8bd99e4f86c349526cbeb3277f902"
        },
        "date": 1695578071079,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10983851,
            "range": "± 404146",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 199505015,
            "range": "± 5326724",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3569195144,
            "range": "± 102978157",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 317438089,
            "range": "± 8773907",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "7822554+AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8eebff9af82e301eeac85a1cab786a1bb01e0979",
          "message": "docs(issue_template): add neutral bug report issue template",
          "timestamp": "2023-09-26T11:37:24+02:00",
          "tree_id": "01708a6a00e25900b97ec31750ca57153cebfa4b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8eebff9af82e301eeac85a1cab786a1bb01e0979"
        },
        "date": 1695723015254,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9546673,
            "range": "± 387014",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164093202,
            "range": "± 7034836",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2735355423,
            "range": "± 61849092",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271940412,
            "range": "± 7688369",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "7822554+AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "committer": {
            "email": "me@alegon.dev",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "635023d5d75861ed261603ffefb9e5b935fb5538",
          "message": "docs(issue_template): add neutral distribution channel to bug template\n\nFor some kind of bugs, the PackSquash distribution does not matter.",
          "timestamp": "2023-09-26T11:44:12+02:00",
          "tree_id": "464151968e5c6115a101b257c3ff49ec542a3809",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/635023d5d75861ed261603ffefb9e5b935fb5538"
        },
        "date": 1695723441762,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12850669,
            "range": "± 528636",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 201452432,
            "range": "± 2301313",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3088954488,
            "range": "± 111678189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 340762258,
            "range": "± 10434200",
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
          "id": "acd056b3d36b41def43e925ac070939899436588",
          "message": "fix(json_file): consider tex. meta as being potentially directory-listed\n\nNot doing so when using the features introduced at\nea933d50675366d36d9e23af55e0c4e38d232388 causes the game to not read\ntexture metadata, potentially breaking effect like texture animations.",
          "timestamp": "2023-09-26T19:06:02+02:00",
          "tree_id": "c4e6d1d4cef222a5d749c333d38635e5ba6207b4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/acd056b3d36b41def43e925ac070939899436588"
        },
        "date": 1695750029738,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9166893,
            "range": "± 83947",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164071061,
            "range": "± 7247101",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2961750849,
            "range": "± 80867983",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 264129711,
            "range": "± 5007452",
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
          "id": "917f1aca6dfe624b9f279c21d87c954fbf379e72",
          "message": "chore: rename `may_be_directory_listed_atlas_texture` to `may_be_directory_listed_atlas_sprite`\n\nGiven that atlases can be textures themselves, I think that referring to\ntheir constituent textures using the word \"sprite\" has less potential\nfor ambiguity.",
          "timestamp": "2023-09-26T19:13:32+02:00",
          "tree_id": "10fbb38d8945c6757f1ab0412da3abc95a065c56",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/917f1aca6dfe624b9f279c21d87c954fbf379e72"
        },
        "date": 1695750391008,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11542921,
            "range": "± 946762",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204173002,
            "range": "± 4306532",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3647734041,
            "range": "± 89153270",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 325437774,
            "range": "± 6561237",
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
          "id": "bc7c3f19401a4b088218149730e95abefed3d98b",
          "message": "chore(deps): update taiki-e/install-action digest to 0087e57",
          "timestamp": "2023-09-26T19:02:40Z",
          "tree_id": "1a81f48634b3335732264dbecd30c4a25500dd77",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bc7c3f19401a4b088218149730e95abefed3d98b"
        },
        "date": 1695767581816,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9241607,
            "range": "± 613305",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164810436,
            "range": "± 10094050",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3008698086,
            "range": "± 65207698",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 267007026,
            "range": "± 9937581",
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
          "id": "c8c36852be2bcb8acfd5c5c2a1e754770171cd72",
          "message": "fix(deps): update rust crate toml to 0.8.1",
          "timestamp": "2023-09-26T22:04:36Z",
          "tree_id": "10ad2d68f5f4e5594af8067900113933f11c4bbc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c8c36852be2bcb8acfd5c5c2a1e754770171cd72"
        },
        "date": 1695779295415,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9449911,
            "range": "± 114513",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 168338043,
            "range": "± 3766294",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3065446300,
            "range": "± 70489427",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 272819551,
            "range": "± 4006056",
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
          "id": "c76701fa01928ae1a4cfef6fbcb26ab2596730fc",
          "message": "fix(deps): update rust crate thiserror to 1.0.49",
          "timestamp": "2023-09-27T01:19:40Z",
          "tree_id": "7fc2c995ea50cb54672f3fdbe576cedb70a65d14",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c76701fa01928ae1a4cfef6fbcb26ab2596730fc"
        },
        "date": 1695785446598,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9623002,
            "range": "± 520860",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 165939233,
            "range": "± 10604083",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2777294635,
            "range": "± 37110969",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 271780835,
            "range": "± 9465577",
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
          "id": "16dac135be33ab1646aba937b6c666214ef980b1",
          "message": "fix(deps): update rust crate indexmap to 2.0.1",
          "timestamp": "2023-09-27T19:59:28Z",
          "tree_id": "9a80b0668eab6495283817471ac33c433b5442e4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/16dac135be33ab1646aba937b6c666214ef980b1"
        },
        "date": 1695855863795,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12692099,
            "range": "± 1001375",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204790103,
            "range": "± 6067154",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3399164990,
            "range": "± 90194033",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 338643546,
            "range": "± 11392399",
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
          "id": "8ac3765c4d13b1d8ac01aa00a83d900c8cecab94",
          "message": "chore(deps): update taiki-e/install-action digest to e07b619",
          "timestamp": "2023-09-28T07:08:41Z",
          "tree_id": "773747ac6a53c81eabd29e5bde46934f674b4aa4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8ac3765c4d13b1d8ac01aa00a83d900c8cecab94"
        },
        "date": 1695895641611,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11937496,
            "range": "± 830399",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 201035125,
            "range": "± 5031727",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2638260697,
            "range": "± 192703383",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 346496840,
            "range": "± 11518856",
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
          "id": "e95ed38da4f8e7845c3c86bae122175aec9ed336",
          "message": "fix(deps): update rust crate zopfli to 0.8.0 (#255)\n\n* fix(deps): update rust crate zopfli to 0.8.0\r\n\r\n* chore(deps): adapt to Zopfli v0.8.0 API changes\r\n\r\n* chore(deps): enable nightly features of some dependencies\r\n\r\nThis might provide a slight performance benefit at no cost for us.\r\n\r\n---------\r\n\r\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>\r\nCo-authored-by: Alejandro González <me@alegon.dev>",
          "timestamp": "2023-09-28T12:15:00+02:00",
          "tree_id": "f34b952060bfb8c0855f92df4cb2ea0725e59e4b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e95ed38da4f8e7845c3c86bae122175aec9ed336"
        },
        "date": 1695897778119,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8129842,
            "range": "± 1140762",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 159423780,
            "range": "± 5091081",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2128000360,
            "range": "± 26188930",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 260150711,
            "range": "± 6687530",
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
          "id": "eff6cf9f9fbd7830fa5b90e6c2b6b0f06df4081a",
          "message": "perf(image_processor): update OxiPNG, slightly tweak Zopfli iterations\n\nThis should provide slightly better savings at the cost of a little bit\nmore of execution time.",
          "timestamp": "2023-09-28T14:02:42+02:00",
          "tree_id": "f64dd5aecbe94e5918a724d943cbda497401bf23",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eff6cf9f9fbd7830fa5b90e6c2b6b0f06df4081a"
        },
        "date": 1695904562123,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10001037,
            "range": "± 416941",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 188322112,
            "range": "± 5904852",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5722601853,
            "range": "± 117770991",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1118960296,
            "range": "± 13873672",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "spongecade.129@gmail.com",
            "name": "Spongecade",
            "username": "Spongecade"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6be1b82d0d0935f9d28985859d7e779374aa8fff",
          "message": "Update Minecraft Wiki links to new domain (#256)",
          "timestamp": "2023-09-28T16:25:21+02:00",
          "tree_id": "89f9d74c06d68fc57f5c3006c45a6bcd4bcd0275",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6be1b82d0d0935f9d28985859d7e779374aa8fff"
        },
        "date": 1695912854243,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9812263,
            "range": "± 564359",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 180754775,
            "range": "± 4035862",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4296806437,
            "range": "± 119097635",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 983753767,
            "range": "± 25086456",
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
          "id": "37225c241bc2e7ab973ac6734c5d1f582317ee74",
          "message": "chore(deps): update taiki-e/install-action digest to 6184f1c",
          "timestamp": "2023-09-28T14:25:36Z",
          "tree_id": "a3a82170a78dd196e6ef8626f5b80084beb3a21e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/37225c241bc2e7ab973ac6734c5d1f582317ee74"
        },
        "date": 1695915430671,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10483763,
            "range": "± 785312",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 191717635,
            "range": "± 4688144",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5754531916,
            "range": "± 103187526",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1152804646,
            "range": "± 29605825",
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
          "id": "2587147e70b9bdd414af58e168690c0637a1bfc6",
          "message": "docs: better document OxiPNG update perf. and compression implications",
          "timestamp": "2023-09-29T23:21:53+02:00",
          "tree_id": "05456c6485f2e2c8d2e3aaa84abe13627abdc703",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2587147e70b9bdd414af58e168690c0637a1bfc6"
        },
        "date": 1696024364754,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 7991402,
            "range": "± 113812",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156516687,
            "range": "± 9576519",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5835440760,
            "range": "± 54008080",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 939334921,
            "range": "± 7605952",
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
          "id": "2a2b8f89ba4795a87a0b16b09fdab3110ec29b9c",
          "message": "fix(deps): update rust crate indexmap to 2.0.2",
          "timestamp": "2023-09-29T19:22:15Z",
          "tree_id": "ab6e191fda9e52f589112dcb83a49cb321261e62",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2a2b8f89ba4795a87a0b16b09fdab3110ec29b9c"
        },
        "date": 1696024453136,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10172082,
            "range": "± 552836",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 189219614,
            "range": "± 4264772",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5718045025,
            "range": "± 105915956",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1117928356,
            "range": "± 23854073",
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
          "id": "9ba6a20e288eea9889dfb747c3fe8647c038ade2",
          "message": "chore(deps): update taiki-e/install-action digest to 57aaba5",
          "timestamp": "2023-09-30T04:54:56Z",
          "tree_id": "8509e36945955213a9216387a5b30e520e33b100",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9ba6a20e288eea9889dfb747c3fe8647c038ade2"
        },
        "date": 1696061233527,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8305238,
            "range": "± 146545",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 159343297,
            "range": "± 10991265",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5830423907,
            "range": "± 42768817",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 929815864,
            "range": "± 11458771",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}