window.BENCHMARK_DATA = {
  "lastUpdate": 1738354261256,
  "repoUrl": "https://github.com/ComunidadAylas/PackSquash",
  "entries": {
    "PackSquash library quick benchmarks": [
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
          "id": "d605784164b70aae042102abf2b39f247e4ed698",
          "message": "fix: downgrade to Cargo resolver version 2\n\nThis is needed to get `cargo-deb` working until\nhttps://github.com/kornelski/cargo-deb/issues/154 is sorted out.",
          "timestamp": "2024-12-03T22:56:31+01:00",
          "tree_id": "a0e3527ff717c2ba1b4c1cffa6f6a54b480724c2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d605784164b70aae042102abf2b39f247e4ed698"
        },
        "date": 1733264876405,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9438570,
            "range": "± 32555",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118087730,
            "range": "± 2632609",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2734564565,
            "range": "± 40889350",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 661977339,
            "range": "± 7408578",
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
          "id": "a918fec5a7b4d54bc5792e0ddda63ee483efae7b",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-12-04T01:23:01Z",
          "tree_id": "7d6ed3908fcd559039ccb9cd4aeb562645dd8885",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a918fec5a7b4d54bc5792e0ddda63ee483efae7b"
        },
        "date": 1733286216136,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9862473,
            "range": "± 1561570",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121533258,
            "range": "± 2038254",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2725984518,
            "range": "± 62888928",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 672637628,
            "range": "± 4940087",
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
          "id": "d0ccf8a7bd6e28cd88c1138700f584cca2047461",
          "message": "chore(deps): update taiki-e/install-action digest to 6aa8b42",
          "timestamp": "2024-12-04T23:26:18Z",
          "tree_id": "09072e8353a31db5113c1a58acf806c6047f07dd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d0ccf8a7bd6e28cd88c1138700f584cca2047461"
        },
        "date": 1733366128505,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9905639,
            "range": "± 159201",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 123215720,
            "range": "± 3934191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2693366715,
            "range": "± 73163090",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 670117551,
            "range": "± 3694652",
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
          "id": "11512b773495702dc5f8b1e838ac110565a4a1da",
          "message": "Revert \"fix: downgrade to Cargo resolver version 2\"\n\nThis reverts commit d605784164b70aae042102abf2b39f247e4ed698.",
          "timestamp": "2024-12-06T15:45:35+01:00",
          "tree_id": "1e2364f2a6033b72e205c106e2bf67348721b426",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/11512b773495702dc5f8b1e838ac110565a4a1da"
        },
        "date": 1733498795123,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9514774,
            "range": "± 71204",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120155981,
            "range": "± 3170628",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2660945576,
            "range": "± 46670852",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662414820,
            "range": "± 4817648",
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
          "id": "47d6c13b85d02015a2e38544fda11decc576a29f",
          "message": "chore: dependency updates",
          "timestamp": "2024-12-06T16:14:39+01:00",
          "tree_id": "eb640c84e7837fb45dfc72e54fc8eda091d4d016",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/47d6c13b85d02015a2e38544fda11decc576a29f"
        },
        "date": 1733500097207,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9753069,
            "range": "± 1508252",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120970578,
            "range": "± 3080445",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2657613408,
            "range": "± 39628899",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 660437405,
            "range": "± 4344648",
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
          "id": "746fb121a63c487ebaf6775f6bc5cc35475732b9",
          "message": "chore(deps): update dependency six to v1.17.0",
          "timestamp": "2024-12-07T01:00:47Z",
          "tree_id": "0ebb96b57397ecca02d16499cdec4b61442df1e9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/746fb121a63c487ebaf6775f6bc5cc35475732b9"
        },
        "date": 1733549093360,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9568658,
            "range": "± 35946",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120040064,
            "range": "± 3590548",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2669432583,
            "range": "± 49695526",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662460905,
            "range": "± 6781682",
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
          "id": "710c8dd1892c45a563390a84613b5183a74d9a6e",
          "message": "chore(deps): update taiki-e/install-action digest to acf70b3",
          "timestamp": "2024-12-07T04:56:03Z",
          "tree_id": "ccd4943ff1bc3f07429d9a5108144654fdbf4fe0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/710c8dd1892c45a563390a84613b5183a74d9a6e"
        },
        "date": 1733556766384,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9453462,
            "range": "± 42077",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118656413,
            "range": "± 3274125",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2711188599,
            "range": "± 47544633",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 658118641,
            "range": "± 1951386",
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
          "id": "5a57c89385777f64f302bee62c15c1a509cea1e7",
          "message": "chore(deps): update ci dependencies (#344)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T09:08:31Z",
          "tree_id": "047cee194a016aa0ce521a5a59bd0b5a2e4f78a9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5a57c89385777f64f302bee62c15c1a509cea1e7"
        },
        "date": 1733910248017,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9675393,
            "range": "± 794329",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119634012,
            "range": "± 2002537",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2678774324,
            "range": "± 44551954",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 667364071,
            "range": "± 2041066",
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
          "id": "5a57c89385777f64f302bee62c15c1a509cea1e7",
          "message": "chore(deps): update ci dependencies (#344)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T09:08:31Z",
          "tree_id": "047cee194a016aa0ce521a5a59bd0b5a2e4f78a9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5a57c89385777f64f302bee62c15c1a509cea1e7"
        },
        "date": 1733911561245,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9531069,
            "range": "± 377108",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119908166,
            "range": "± 4374207",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2700626097,
            "range": "± 39193358",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662680526,
            "range": "± 4466434",
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
          "id": "3201cce4d0d2c74d15c01cd81fc79f7794772898",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-12-13T02:22:15Z",
          "tree_id": "d571f44a366a7085af20708b153b7e86a8b83873",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3201cce4d0d2c74d15c01cd81fc79f7794772898"
        },
        "date": 1734062435179,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9525392,
            "range": "± 4393110",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119532727,
            "range": "± 2460339",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2620162161,
            "range": "± 42241912",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 660184604,
            "range": "± 3286910",
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
          "id": "4a92ad8b7bbc231dd2619f4648653f5edcc11c86",
          "message": "chore(deps): update taiki-e/install-action digest to 8c39981",
          "timestamp": "2024-12-13T07:06:11Z",
          "tree_id": "7174ccbf78a884ed37471e5a9c630d03bc1d73d2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4a92ad8b7bbc231dd2619f4648653f5edcc11c86"
        },
        "date": 1734090428591,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9796004,
            "range": "± 320505",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119835543,
            "range": "± 3601662",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2617905753,
            "range": "± 36538576",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 657108769,
            "range": "± 5758327",
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
          "id": "aaf817a1bd341ef1855a476f1394d5417e87b105",
          "message": "chore(cargo/deps): update all, dogfooding our new `vorbis_rs` version",
          "timestamp": "2024-12-13T20:07:10+01:00",
          "tree_id": "c711a807de79f78013c8b3631de8d216d7556702",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aaf817a1bd341ef1855a476f1394d5417e87b105"
        },
        "date": 1734118864668,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9742242,
            "range": "± 153339",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118353530,
            "range": "± 2667151",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2680209771,
            "range": "± 47527900",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 663808266,
            "range": "± 2765822",
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
          "id": "b4f689598102f45f912f263a11f51bb99f56d3d0",
          "message": "ci: addendum to 7ac7c5774b85e81c933e848add9aca4332231af2",
          "timestamp": "2024-12-15T16:53:37+01:00",
          "tree_id": "bd30e451e54b4c45571960bc8a2f3fa8c9b4f1f1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b4f689598102f45f912f263a11f51bb99f56d3d0"
        },
        "date": 1734280084092,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9554773,
            "range": "± 82187",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118347077,
            "range": "± 2936259",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2646577466,
            "range": "± 47184838",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 661096542,
            "range": "± 3203157",
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
          "id": "6d831a0fe88815b9b3be1d430f731e326e552181",
          "message": "chore(deps): pin dependencies",
          "timestamp": "2024-12-16T00:32:36Z",
          "tree_id": "c37bf70f2eef27f22736e712d807b17d3a225935",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6d831a0fe88815b9b3be1d430f731e326e552181"
        },
        "date": 1734323976151,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9656453,
            "range": "± 576625",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119033962,
            "range": "± 2756167",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2671894291,
            "range": "± 45508920",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662718527,
            "range": "± 2436840",
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
          "id": "8ecfea0ed0877d4d40afe6db56bc4dc092265a6b",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-12-16T16:51:47Z",
          "tree_id": "83cf2cf71c14228a45489297c9703b82a54a04d1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8ecfea0ed0877d4d40afe6db56bc4dc092265a6b"
        },
        "date": 1734378763410,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9632658,
            "range": "± 1256453",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119857998,
            "range": "± 1176313",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2625488952,
            "range": "± 35781335",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 663779734,
            "range": "± 2561615",
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
          "id": "ec4472c219cffb2fd10a82b6135ce1120a79568f",
          "message": "chore(deps): update dependency certifi to v2024.12.14",
          "timestamp": "2024-12-16T19:22:14Z",
          "tree_id": "1f634f610f5db45a41d7b7852129f4146687cdbb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ec4472c219cffb2fd10a82b6135ce1120a79568f"
        },
        "date": 1734389409187,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9713278,
            "range": "± 92394",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120279199,
            "range": "± 2914993",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2655881248,
            "range": "± 65049105",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 664574607,
            "range": "± 3906170",
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
          "id": "81c447001c2f14efe926d26dc5986fba7756a97c",
          "message": "ci: ignore test that is expected to fail to run on our environment",
          "timestamp": "2024-12-19T00:39:43+01:00",
          "tree_id": "d1eae9bfffc7495522418e2ec9b7ef3a23894d09",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/81c447001c2f14efe926d26dc5986fba7756a97c"
        },
        "date": 1734567341563,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9870081,
            "range": "± 179081",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119470867,
            "range": "± 2654992",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2627487354,
            "range": "± 36817066",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 665265150,
            "range": "± 4779105",
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
          "id": "5cfc0b9db22093abae6f42d6429564af59f8bc01",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-12-19T01:45:08Z",
          "tree_id": "0efc02703691c4cf6eb1b6e4fd9f01e8dbd4f76c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5cfc0b9db22093abae6f42d6429564af59f8bc01"
        },
        "date": 1734585352803,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9603948,
            "range": "± 1340501",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119758034,
            "range": "± 3775759",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2666631344,
            "range": "± 92280372",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 659137118,
            "range": "± 3799340",
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
          "id": "cd0265262bb2e2fcef90d9db52bc24d4b71581c4",
          "message": "chore(deps): update taiki-e/install-action digest to 9023ed5",
          "timestamp": "2024-12-19T15:23:47Z",
          "tree_id": "86749be535c26bd419f61706d375e57fcb5f0be7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cd0265262bb2e2fcef90d9db52bc24d4b71581c4"
        },
        "date": 1734636724015,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9539214,
            "range": "± 309625",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119953316,
            "range": "± 4001782",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2660313613,
            "range": "± 93673468",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 659145914,
            "range": "± 2297227",
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
          "id": "378cda25b546be344b77222e6eb7ba4c64832f32",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-12-22T01:44:18Z",
          "tree_id": "9ee04b1a6d29b073bbc4973575a99c90bb7c7752",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/378cda25b546be344b77222e6eb7ba4c64832f32"
        },
        "date": 1734842467293,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9639984,
            "range": "± 103749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119246474,
            "range": "± 2992112",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2736389355,
            "range": "± 60631453",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 663922131,
            "range": "± 3709209",
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
          "id": "048ece571a1a398c220907da4ac61174f7b29666",
          "message": "chore(deps): update taiki-e/install-action digest to 8484225",
          "timestamp": "2024-12-22T07:41:59Z",
          "tree_id": "975ff629b5a88356a7294f2b6706843bc3727184",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/048ece571a1a398c220907da4ac61174f7b29666"
        },
        "date": 1734862668270,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9487576,
            "range": "± 1481549",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115470257,
            "range": "± 1383534",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2705138621,
            "range": "± 32005053",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662249242,
            "range": "± 5274279",
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
          "id": "2ddbefb0e3f427530ad65d8e9fa30dd24baba9d8",
          "message": "chore(deps): update dependency urllib3 to v2.3.0",
          "timestamp": "2024-12-25T00:46:30Z",
          "tree_id": "cbb30566b304e42c1495360913e48942e36c933c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2ddbefb0e3f427530ad65d8e9fa30dd24baba9d8"
        },
        "date": 1735102388433,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9547199,
            "range": "± 57949",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 117602278,
            "range": "± 3335189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2687252409,
            "range": "± 37241322",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 659886490,
            "range": "± 3123888",
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
          "id": "18427c3e5cab79acac2645de10cd2b695eea2779",
          "message": "chore(deps): update debian:bookworm-slim docker digest to d365f49",
          "timestamp": "2024-12-25T06:21:47Z",
          "tree_id": "d1abafe952f6d07cff3af233d97ce7956aa05416",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/18427c3e5cab79acac2645de10cd2b695eea2779"
        },
        "date": 1735121406282,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10716255,
            "range": "± 589394",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121720977,
            "range": "± 4240327",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2742138941,
            "range": "± 51947664",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 675575211,
            "range": "± 3378928",
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
          "id": "ffbbb88a884073e0b08765d4762def4b6fe4b22f",
          "message": "chore(deps): update taiki-e/install-action digest to acd2589",
          "timestamp": "2024-12-28T01:26:07Z",
          "tree_id": "73acb93ca60ec23bf889d01111aa116493bceecd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ffbbb88a884073e0b08765d4762def4b6fe4b22f"
        },
        "date": 1735363075146,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9657331,
            "range": "± 61389",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118481308,
            "range": "± 3953873",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2681969899,
            "range": "± 51104517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 660705492,
            "range": "± 3323711",
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
          "id": "33ba47274f73089868d914885b6d9440759b880a",
          "message": "chore(deps): update dependency charset-normalizer to v3.4.1",
          "timestamp": "2024-12-28T04:44:59Z",
          "tree_id": "05c368ebdcce81a93b894b8ba9d1e2658f6f6a77",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/33ba47274f73089868d914885b6d9440759b880a"
        },
        "date": 1735370386787,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9494079,
            "range": "± 446988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 117878650,
            "range": "± 3067218",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2677259393,
            "range": "± 63675578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 659882987,
            "range": "± 3123989",
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
          "id": "210b05f530a9ca9eb22a06eb3c8f30e6624f69dc",
          "message": "chore(deps): update swatinem/rust-cache digest to 720f7e4",
          "timestamp": "2024-12-28T10:01:03Z",
          "tree_id": "1404c824dba8622b03e6b2d4c2104eeced60c77c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/210b05f530a9ca9eb22a06eb3c8f30e6624f69dc"
        },
        "date": 1735389039267,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9473344,
            "range": "± 185005",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118429283,
            "range": "± 2667724",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2684372890,
            "range": "± 33029085",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 661038406,
            "range": "± 2579433",
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
          "id": "d3f48e103534dcb7c3ec87f72770b7b9cd7d83fc",
          "message": "tweak(system_id/linux): more robust aggregated DMI serials ID result\n\nBy sorting serial numbers before hashing them the code can isolate\nitself from unimportant ordering differences in the retrieved serial\nnumbers, which can be caused by both software quirks and hardware\nmodifications.",
          "timestamp": "2024-12-30T01:31:03+01:00",
          "tree_id": "7bb105a81f969d4935e8c7c3bbdfcfe553ec6aed",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3f48e103534dcb7c3ec87f72770b7b9cd7d83fc"
        },
        "date": 1735520890731,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9657874,
            "range": "± 70935",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 116655817,
            "range": "± 3880899",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2669247767,
            "range": "± 51345038",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 663828935,
            "range": "± 6341957",
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
          "id": "e22448aa4ad9213dab2486ce3582775c3e75e83f",
          "message": "chore(deps): update swatinem/rust-cache digest to f0deed1",
          "timestamp": "2024-12-31T00:51:19Z",
          "tree_id": "44e7eefbeddc2fd713bfa06b7cd0da80b53cb5ac",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e22448aa4ad9213dab2486ce3582775c3e75e83f"
        },
        "date": 1735620403003,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9493591,
            "range": "± 152329",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 117473083,
            "range": "± 4235562",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2671778315,
            "range": "± 116728021",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 657396149,
            "range": "± 4155429",
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
          "id": "61cf1e167291a0503762cc76ae5f75d05ac06336",
          "message": "chore(deps): update taiki-e/install-action digest to 77b010c",
          "timestamp": "2024-12-31T12:47:23Z",
          "tree_id": "54997e068fe7689a97d39181fc995bda010d876d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/61cf1e167291a0503762cc76ae5f75d05ac06336"
        },
        "date": 1735664726021,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9546544,
            "range": "± 1020439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118413796,
            "range": "± 4957935",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2654226010,
            "range": "± 45266578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 658179538,
            "range": "± 3142974",
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
          "id": "4248f699add05d66e9674df737338c41a52588e0",
          "message": "chore(deps): update taiki-e/install-action digest to dbc32cd",
          "timestamp": "2024-12-31T20:09:21Z",
          "tree_id": "608d099aab26276a56fb66d3dc1b525d71674623",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4248f699add05d66e9674df737338c41a52588e0"
        },
        "date": 1735685916157,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9476497,
            "range": "± 3344433",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 117104246,
            "range": "± 1517157",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2645528779,
            "range": "± 60658710",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 658157101,
            "range": "± 3490389",
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
          "id": "ab1f83d4c02d5baf08872b7c66fdcd817dd52ac6",
          "message": "chore(test): fix Windows build",
          "timestamp": "2025-01-02T19:46:31+01:00",
          "tree_id": "a7e7972a6f2f15b81ccb24722c21774101de6092",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ab1f83d4c02d5baf08872b7c66fdcd817dd52ac6"
        },
        "date": 1735845735334,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9919462,
            "range": "± 1431362",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118781176,
            "range": "± 2085820",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2719970314,
            "range": "± 44232733",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 669125623,
            "range": "± 5115394",
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
          "id": "7455bcaa9dea7ad4233da97bd97203f4df16e82d",
          "message": "chore(deps): update taiki-e/install-action digest to a86da1a",
          "timestamp": "2025-01-07T12:00:54Z",
          "tree_id": "5db780bc6607a760b72697524485a1e86f45bdfc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7455bcaa9dea7ad4233da97bd97203f4df16e82d"
        },
        "date": 1736255016713,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10561879,
            "range": "± 296641",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 133070911,
            "range": "± 3015348",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2558614518,
            "range": "± 39180486",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 673978955,
            "range": "± 4282466",
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
          "id": "e7f6095b663766979fe06af1b7784c1e3c468780",
          "message": "tweak: fix build version gathering on CI, prettify build date display",
          "timestamp": "2025-01-08T00:49:13+01:00",
          "tree_id": "504762348192301b2d3c07d16989b9a53c1e9d09",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e7f6095b663766979fe06af1b7784c1e3c468780"
        },
        "date": 1736295590227,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9666629,
            "range": "± 49312",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 130419832,
            "range": "± 3624874",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2637789893,
            "range": "± 25366746",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 680549948,
            "range": "± 3083010",
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
          "id": "1efb0fd3ca2039cac75eb33f7a1c1c01deff07c0",
          "message": "chore(deps): update taiki-e/install-action digest to 08d473f",
          "timestamp": "2025-01-07T23:49:41Z",
          "tree_id": "0213c75b864c7db0130ceb84db52c93349e44e57",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1efb0fd3ca2039cac75eb33f7a1c1c01deff07c0"
        },
        "date": 1736349635184,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10312773,
            "range": "± 1300161",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 133349385,
            "range": "± 2703166",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2560223894,
            "range": "± 30189097",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 678816749,
            "range": "± 4158873",
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
          "id": "e98c42f93079bdb34b00a67a53b48560861aa536",
          "message": "chore(deps): update taiki-e/install-action digest to df5dec2",
          "timestamp": "2025-01-10T01:24:46Z",
          "tree_id": "1b8c80471f21a713970e96495379e70e8f814ffe",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e98c42f93079bdb34b00a67a53b48560861aa536"
        },
        "date": 1736491244381,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9795602,
            "range": "± 689205",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 130851640,
            "range": "± 3229405",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2569919806,
            "range": "± 24995087",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 677117313,
            "range": "± 2627307",
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
          "id": "2bd7aff03d10de2dfb67801db890a48c98f7203f",
          "message": "chore(deps): refresh them all, dogfood our new OptiVorbis version",
          "timestamp": "2025-01-13T22:51:18+01:00",
          "tree_id": "d8ce7f262d5305ab7d6799e6a23bb4539042203c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2bd7aff03d10de2dfb67801db890a48c98f7203f"
        },
        "date": 1736807406277,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9845187,
            "range": "± 118474",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129379108,
            "range": "± 2008808",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2557491541,
            "range": "± 35984337",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 679080790,
            "range": "± 4283602",
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
          "id": "66af0c55aeedcf9173a1e545fc3305a6205de19d",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-01-16T04:19:00Z",
          "tree_id": "51ac257c8434fb3aca7d22ae43a1b38e50f0cbf2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/66af0c55aeedcf9173a1e545fc3305a6205de19d"
        },
        "date": 1737023052922,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10730231,
            "range": "± 149239",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 135080099,
            "range": "± 2063851",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2869412650,
            "range": "± 141234686",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 680625904,
            "range": "± 7622321",
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
          "id": "ff5d63e9586dba55b0aae15a44efca5f4c7fc630",
          "message": "chore(deps): update taiki-e/install-action digest to d125c0a",
          "timestamp": "2025-01-16T17:10:28Z",
          "tree_id": "405bf53c5ba2ca6e33b80480b9f706aacd4b8732",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ff5d63e9586dba55b0aae15a44efca5f4c7fc630"
        },
        "date": 1737064957243,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9975458,
            "range": "± 1335564",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 130806487,
            "range": "± 2091155",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2757632581,
            "range": "± 105417620",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 682958983,
            "range": "± 4623582",
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
          "id": "d180c81ed6b075a3a1b6a19db05ec2539c709710",
          "message": "chore(deps): update taiki-e/install-action digest to a7adeb1",
          "timestamp": "2025-01-19T12:34:24Z",
          "tree_id": "17e8b5e22481c8bce1bc6a755f4d063201376722",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d180c81ed6b075a3a1b6a19db05ec2539c709710"
        },
        "date": 1737309449066,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10220805,
            "range": "± 186721",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 133358763,
            "range": "± 2432999",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2589191832,
            "range": "± 42133769",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 687580238,
            "range": "± 9217051",
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
          "id": "06b094140e4027384b6efa6085530b35ddf8ecb2",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-01-22T15:27:28Z",
          "tree_id": "d9b8caaa19b644ea05f817379b3954ef6a196bb6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/06b094140e4027384b6efa6085530b35ddf8ecb2"
        },
        "date": 1737570355904,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9647055,
            "range": "± 120660",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119490361,
            "range": "± 3013523",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2829054733,
            "range": "± 28826921",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 600387008,
            "range": "± 1587326",
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
          "id": "14855c34167a8d8dd34a8044fb4397bc65c88ba1",
          "message": "chore(deps): update github/codeql-action digest to dd196fa",
          "timestamp": "2025-01-22T20:47:18Z",
          "tree_id": "37eba1d5170a0bcbb1c5fc480b55e3c2db3b18e9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/14855c34167a8d8dd34a8044fb4397bc65c88ba1"
        },
        "date": 1737593961542,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9744823,
            "range": "± 1244281",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129555161,
            "range": "± 3051902",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2547263312,
            "range": "± 28817160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 677800818,
            "range": "± 3515555",
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
          "id": "44cde179b7df12f8e599c487dc53e94bc68feb3c",
          "message": "chore: fix tests build",
          "timestamp": "2025-01-24T23:43:33+01:00",
          "tree_id": "81c11d46dfdcb401e4f7a0aa0afdf0bd00a45191",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/44cde179b7df12f8e599c487dc53e94bc68feb3c"
        },
        "date": 1737760880634,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10468900,
            "range": "± 4188078",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121274907,
            "range": "± 925656",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2879305142,
            "range": "± 33925963",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 611206928,
            "range": "± 3675458",
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
          "id": "59cae1ae9d82592964f333c2aecf1ff9c1efa22c",
          "message": "refactor: simplify some generic type declarations",
          "timestamp": "2025-01-26T00:12:13+01:00",
          "tree_id": "629c2cf01128375ddbbf2eb291706b7ddd510043",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/59cae1ae9d82592964f333c2aecf1ff9c1efa22c"
        },
        "date": 1737848915152,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10082742,
            "range": "± 1090013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118995804,
            "range": "± 3157422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2876775781,
            "range": "± 16470685",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 614826595,
            "range": "± 2750616",
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
          "id": "6c69b9a50f0f987506e148652464e1c5abb8041a",
          "message": "ci: fix benchmark build",
          "timestamp": "2025-01-26T21:14:14+01:00",
          "tree_id": "3d9c5fc56bd7d52afaf0a4f3651860361c7b7799",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6c69b9a50f0f987506e148652464e1c5abb8041a"
        },
        "date": 1737924344298,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10476120,
            "range": "± 983046",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119596231,
            "range": "± 2607420",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2841268749,
            "range": "± 64973962",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 604835114,
            "range": "± 2678054",
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
          "id": "a5e0c2d1ad2b86fc58c8e7ce8707d11b8e8c7b12",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-01-28T09:13:05Z",
          "tree_id": "fcc9a0eaaf0b52e1fde11a4a022aeb9d2a93759b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a5e0c2d1ad2b86fc58c8e7ce8707d11b8e8c7b12"
        },
        "date": 1738070008774,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9977902,
            "range": "± 1296692",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118907520,
            "range": "± 3243164",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2909527518,
            "range": "± 31702306",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 609284111,
            "range": "± 1621492",
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
          "id": "ffca3e53243b894536da9d668275d299a71285cc",
          "message": "chore(deps): update dependency filelock to v3.17.0",
          "timestamp": "2025-01-28T12:41:17Z",
          "tree_id": "342b43d214e12ab48474a7d15e5828bb27d265c6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ffca3e53243b894536da9d668275d299a71285cc"
        },
        "date": 1738087679686,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10167106,
            "range": "± 1072885",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118117881,
            "range": "± 3107050",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2901929970,
            "range": "± 26771587",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 606121581,
            "range": "± 1892604",
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
          "id": "863d5484ce5d33d82a0b20e214fe5abf303822a3",
          "message": "feat: proper compressed compound tag NBT (structure) file optimization",
          "timestamp": "2025-01-30T00:57:06+01:00",
          "tree_id": "8c3b43702c08eb1c390c2202686f86ab5cdaf651",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/863d5484ce5d33d82a0b20e214fe5abf303822a3"
        },
        "date": 1738197486606,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9861208,
            "range": "± 458559",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120495386,
            "range": "± 2736430",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2877858409,
            "range": "± 19579153",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 617416663,
            "range": "± 6206673",
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
          "id": "3b6d2609a8810089e82e7c99436e0f9b5485f382",
          "message": "refactor: slightly simplify some code",
          "timestamp": "2025-01-31T19:46:19+01:00",
          "tree_id": "cf3fd8859684d24dcfd92e5962d86c2b987b9bf4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3b6d2609a8810089e82e7c99436e0f9b5485f382"
        },
        "date": 1738351539920,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9924729,
            "range": "± 112879",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120604892,
            "range": "± 2649097",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2975525357,
            "range": "± 28587370",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 615339602,
            "range": "± 2434223",
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
          "id": "791862b247d357d8e7089f4a7ac5932b418088df",
          "message": "chore(deps): update ci dependencies (#350)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-31T19:16:03Z",
          "tree_id": "138d6f1df9fe8ad50cf564b1c4b93b03b2c2b9a0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/791862b247d357d8e7089f4a7ac5932b418088df"
        },
        "date": 1738353080392,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10486705,
            "range": "± 4934826",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121403820,
            "range": "± 1547529",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2954621216,
            "range": "± 24433662",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 614139015,
            "range": "± 2061962",
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
          "id": "791862b247d357d8e7089f4a7ac5932b418088df",
          "message": "chore(deps): update ci dependencies (#350)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-31T19:16:03Z",
          "tree_id": "138d6f1df9fe8ad50cf564b1c4b93b03b2c2b9a0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/791862b247d357d8e7089f4a7ac5932b418088df"
        },
        "date": 1738354260742,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9959721,
            "range": "± 1280634",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121552011,
            "range": "± 3459756",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2938200034,
            "range": "± 27198364",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 613619758,
            "range": "± 1359111",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}