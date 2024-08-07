window.BENCHMARK_DATA = {
  "lastUpdate": 1723059215630,
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
          "id": "f9db548dd98f820fd08af838d76ec687f6b793be",
          "message": "chore: fix some more Clippy lints, tweak their config",
          "timestamp": "2024-06-30T01:20:03+02:00",
          "tree_id": "848b3dc28cdfee1b3c2664f9cfc83e9b952c7922",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f9db548dd98f820fd08af838d76ec687f6b793be"
        },
        "date": 1719705174013,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9765806,
            "range": "± 478589",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143814770,
            "range": "± 1284629",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2770611691,
            "range": "± 26558367",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 612074516,
            "range": "± 2729468",
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
          "id": "64523766c6aa8472184c08a6901a3490fa5e79dc",
          "message": "fix(deps): update rust crate serde_json to v1.0.119",
          "timestamp": "2024-07-01T01:42:06Z",
          "tree_id": "0958ebc3ab4af90cd639300725dc1e2a848c1d9e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/64523766c6aa8472184c08a6901a3490fa5e79dc"
        },
        "date": 1719810010240,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9583820,
            "range": "± 308516",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144053307,
            "range": "± 2600570",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2761763429,
            "range": "± 26445042",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608560863,
            "range": "± 2900250",
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
          "id": "6ad79f629d2d7d79646044e6817888db4aec78d0",
          "message": "ci: bring back GitHub Actions cache for Windows\n\nAccording to all known laws of informatics, sccache should work well on\nWindows. But for some reason in our environment Windows builds with the\nMinGW toolchain get so slow due to sccache overheads, even though our\ncurrent sccache approach with a Redis database storing compilation\naritfacts is overall a net improvement for the remaining targets. So\nlet's fall back to the older cache method for Windows to continue\ncutting down on CI time.",
          "timestamp": "2024-07-02T12:11:03+02:00",
          "tree_id": "1543deaccc8d22ae010e2e018ba0256468091b82",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6ad79f629d2d7d79646044e6817888db4aec78d0"
        },
        "date": 1719916958722,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9623940,
            "range": "± 1481301",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143312455,
            "range": "± 2715269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2758846887,
            "range": "± 29182899",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608325546,
            "range": "± 2167146",
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
          "id": "9e522cd1a57a3da100a9a70be4a096b96e8aaee6",
          "message": "fix(deps): update rust crate serde_json to v1.0.120 (#301)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-02T12:36:30+02:00",
          "tree_id": "ac3c15a735f9b5f101445074065384bc198f1c80",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9e522cd1a57a3da100a9a70be4a096b96e8aaee6"
        },
        "date": 1719917795638,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9515515,
            "range": "± 411785",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142904206,
            "range": "± 1684132",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2786014782,
            "range": "± 84680905",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 603017890,
            "range": "± 3277280",
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
          "id": "be81c5845491980fd6349007699b14c37db7df4f",
          "message": "chore(deps): update debian:bullseye-slim docker digest to 34b63f5",
          "timestamp": "2024-07-04T01:03:36Z",
          "tree_id": "91652c5d50c76d5adc055f280628c660f1acf8d8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/be81c5845491980fd6349007699b14c37db7df4f"
        },
        "date": 1720068850526,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9573502,
            "range": "± 359887",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142463883,
            "range": "± 5465531",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2728355689,
            "range": "± 21208021",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 603626659,
            "range": "± 3572739",
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
          "id": "6f241db78319ab120ae13fa7cdb871d4c2b84825",
          "message": "chore(deps): update docker/setup-buildx-action digest to 4fd8129",
          "timestamp": "2024-07-04T08:14:12Z",
          "tree_id": "909f7b16eef31a0d860cd3f8a79b4fc25dd3bbd9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6f241db78319ab120ae13fa7cdb871d4c2b84825"
        },
        "date": 1720091412781,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9786946,
            "range": "± 76455",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144183260,
            "range": "± 2921814",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2743118667,
            "range": "± 24347318",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608642272,
            "range": "± 3248948",
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
          "id": "f6efc4abfec73811a14a407d168241a68fdddc07",
          "message": "chore(deps): update taiki-e/install-action digest to e6ea94a",
          "timestamp": "2024-07-05T15:18:39Z",
          "tree_id": "12af5c1edc7ede8e5508e571e399ed459d5462e2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f6efc4abfec73811a14a407d168241a68fdddc07"
        },
        "date": 1720194177757,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9710505,
            "range": "± 1495719",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142484879,
            "range": "± 2701264",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2757655257,
            "range": "± 27090332",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 606158315,
            "range": "± 2819537",
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
          "id": "65e12e373e94f77ff5559b9a8f22f0c92b653205",
          "message": "chore(deps): update gcr.io/distroless/static-debian11 docker digest to 1dbe426",
          "timestamp": "2024-07-05T19:34:37Z",
          "tree_id": "18d9f7738adb511ebf3563dfc1f8d6b94b6aa67c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/65e12e373e94f77ff5559b9a8f22f0c92b653205"
        },
        "date": 1720209390696,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9502150,
            "range": "± 52243",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142592479,
            "range": "± 6609861",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2752761900,
            "range": "± 21539400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 606401351,
            "range": "± 3331492",
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
          "id": "877af1ffec04ad075a832d6ceecd8d9b3d5cb9e0",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-07-07T00:47:37Z",
          "tree_id": "a52c5752db6e2259027cc339f551ff868db38eed",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/877af1ffec04ad075a832d6ceecd8d9b3d5cb9e0"
        },
        "date": 1720327190186,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9964046,
            "range": "± 1584126",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143426285,
            "range": "± 3968176",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2777422719,
            "range": "± 25449776",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608422798,
            "range": "± 3570292",
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
          "id": "3db94cb2f2f54938e132436cc4c75a1b47fbc2d9",
          "message": "fix(deps): update rust crate serde to v1.0.204",
          "timestamp": "2024-07-07T04:16:56Z",
          "tree_id": "733437c4318a7c970b5ee3c6fba1c5770728ca63",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3db94cb2f2f54938e132436cc4c75a1b47fbc2d9"
        },
        "date": 1720334010855,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10045059,
            "range": "± 68320",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 145035154,
            "range": "± 1243316",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2776849572,
            "range": "± 19669521",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 614586631,
            "range": "± 2736727",
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
          "id": "bb876a64c711b1ac52d45ba12d1e28794fb7bafa",
          "message": "chore(deps): update dependency certifi to v2024.7.4",
          "timestamp": "2024-07-07T06:12:55Z",
          "tree_id": "01d868161e7c14cb5288d41a1bf39e61e5170ded",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bb876a64c711b1ac52d45ba12d1e28794fb7bafa"
        },
        "date": 1720346145958,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9582338,
            "range": "± 72431",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142922464,
            "range": "± 2022354",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2768633708,
            "range": "± 17973695",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 609796760,
            "range": "± 2516773",
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
          "id": "2bea85a555a6d551bbb7a328904d94bcee36977c",
          "message": "fix(deps): update rust crate rgb to v0.8.42",
          "timestamp": "2024-07-07T19:01:19Z",
          "tree_id": "1b5b53d5f1bcc475e5d895e702a8e293f7d39b9c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2bea85a555a6d551bbb7a328904d94bcee36977c"
        },
        "date": 1720393692888,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9659652,
            "range": "± 1337686",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142405228,
            "range": "± 2111834",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2735062904,
            "range": "± 30715182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 600611767,
            "range": "± 2425467",
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
          "id": "719954ce0903dfd4180888fc4fd3693b481de81e",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-07-10T00:12:34Z",
          "tree_id": "db97016ab3c8479596ef6d9abef22cf09c7e6088",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/719954ce0903dfd4180888fc4fd3693b481de81e"
        },
        "date": 1720582101896,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10613099,
            "range": "± 79419",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142669265,
            "range": "± 2931783",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2745190041,
            "range": "± 13440761",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 613599443,
            "range": "± 1526255",
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
          "id": "cc6e2baaa7c13de17008953d52c55a6887792804",
          "message": "chore(deps): update ci dependencies (#304)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-10T12:55:38+02:00",
          "tree_id": "7ecf40bf0169866c69f1be52c43b421687a3a3d5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cc6e2baaa7c13de17008953d52c55a6887792804"
        },
        "date": 1720610191015,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9683166,
            "range": "± 86379",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142867426,
            "range": "± 3716889",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2746910632,
            "range": "± 14538998",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 609217764,
            "range": "± 2699712",
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
          "id": "97a5f9ac8189f147d56d83d0057e51c768a06121",
          "message": "chore(deps): update taiki-e/install-action digest to 30bcc3a",
          "timestamp": "2024-07-10T13:15:19Z",
          "tree_id": "0a2a907465fa598264956d951bf0a5291b0f2821",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/97a5f9ac8189f147d56d83d0057e51c768a06121"
        },
        "date": 1720632173824,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9708780,
            "range": "± 995205",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140878485,
            "range": "± 5855393",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2743073057,
            "range": "± 13883904",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608172009,
            "range": "± 2607346",
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
          "id": "be9cfc7fa175715a6eab64a6c12c64fdc209f498",
          "message": "chore(deps): update taiki-e/install-action digest to ef2fb5a",
          "timestamp": "2024-07-10T19:52:36Z",
          "tree_id": "d85e15e25588736ab59dc2edbe6d34b7ce3490ae",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/be9cfc7fa175715a6eab64a6c12c64fdc209f498"
        },
        "date": 1720649443832,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9553640,
            "range": "± 82538",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141298270,
            "range": "± 2317665",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2744973293,
            "range": "± 32023617",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 611192158,
            "range": "± 2584104",
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
          "id": "5e471e523e4977caf6e8bebeef431595629093d3",
          "message": "chore(deps): update taiki-e/install-action digest to 32300fc",
          "timestamp": "2024-07-13T01:02:47Z",
          "tree_id": "51ae7f741021085094edb6ae1250050b8fe6bec7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5e471e523e4977caf6e8bebeef431595629093d3"
        },
        "date": 1720843934691,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9652502,
            "range": "± 886777",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141916145,
            "range": "± 3173503",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2752461967,
            "range": "± 10334576",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 610357195,
            "range": "± 3024931",
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
          "id": "55408cfb31abdb834016538a7c2ed40e3ce773e1",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-07-13T03:49:48Z",
          "tree_id": "467999709f5c8214e5fb037a8d9665fbc001799e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/55408cfb31abdb834016538a7c2ed40e3ce773e1"
        },
        "date": 1720858108398,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9568993,
            "range": "± 1424694",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140554484,
            "range": "± 2871022",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2735651479,
            "range": "± 11146408",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 603147082,
            "range": "± 3688301",
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
          "id": "9f1852d86509236704d86e6f730ba2148baa6622",
          "message": "fix(deps): update rust crate bytes to v1.6.1",
          "timestamp": "2024-07-13T12:59:40Z",
          "tree_id": "35d10a49d815f67d76484e616e33f59ffdc63919",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9f1852d86509236704d86e6f730ba2148baa6622"
        },
        "date": 1720893983891,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9702169,
            "range": "± 335512",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142509050,
            "range": "± 2205968",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2759161341,
            "range": "± 23578176",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 608561714,
            "range": "± 2334666",
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
          "id": "e80d7bcf708358fbb7ea37586b0ef817363e0d19",
          "message": "chore(deps): update taiki-e/install-action digest to 3e71e71",
          "timestamp": "2024-07-16T02:25:09Z",
          "tree_id": "a3614d05cb297a50ae1b4b5c9117b16a5e04139a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e80d7bcf708358fbb7ea37586b0ef817363e0d19"
        },
        "date": 1721100315520,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9528124,
            "range": "± 3647005",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141950876,
            "range": "± 1105873",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2797725201,
            "range": "± 28664413",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 605685722,
            "range": "± 2380776",
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
          "id": "69f29d2b2352118a0fa2d3e9b9ed0b8882b8e52e",
          "message": "fix(deps): update rust crate rgb to v0.8.45",
          "timestamp": "2024-07-16T03:04:10Z",
          "tree_id": "e6cb8140af33d6bd24bea88f6efbd6ca97f29cf1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/69f29d2b2352118a0fa2d3e9b9ed0b8882b8e52e"
        },
        "date": 1721112700941,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9498362,
            "range": "± 296127",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141092668,
            "range": "± 2438037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2786067708,
            "range": "± 67320524",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 603132103,
            "range": "± 2389169",
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
          "id": "bffc6ea9f9297a247ee7311b9adf78bb5edcf8fa",
          "message": "fix(deps): update rust crate tokio to v1.38.1",
          "timestamp": "2024-07-16T21:30:39Z",
          "tree_id": "0eba34ce54351cd558f9f97bb0e0182d829d9d99",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bffc6ea9f9297a247ee7311b9adf78bb5edcf8fa"
        },
        "date": 1721179224322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9513238,
            "range": "± 1412769",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140185540,
            "range": "± 2759061",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2743249973,
            "range": "± 31602735",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 601329964,
            "range": "± 2891589",
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
          "id": "24ef938f41e274954d138a6fd5586700c327ef0a",
          "message": "ci: use stable Debian images for some targets\n\nThe testing Debian release is unstable and may have broken packages from\ntime to time, but most importantly, we don't really need it nowadays as\nthe latest Debian stable release has all the packages we need in their\nright versions.",
          "timestamp": "2024-07-28T14:31:35+02:00",
          "tree_id": "f6d260b3d669b6206acbe9027fc122e1f4ad333c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/24ef938f41e274954d138a6fd5586700c327ef0a"
        },
        "date": 1722171213308,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9573347,
            "range": "± 398903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120034398,
            "range": "± 2817025",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2728457868,
            "range": "± 79175362",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 676774081,
            "range": "± 2874138",
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
          "id": "a683f2a00c84a4cd8ca6f1e28151855f8523cf29",
          "message": "chore(deps): update ci dependencies (#305)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T12:37:14Z",
          "tree_id": "63e322022bb73304a31a0a1b6fe540da88fbb61d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a683f2a00c84a4cd8ca6f1e28151855f8523cf29"
        },
        "date": 1722171551813,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9564598,
            "range": "± 347656",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 122194249,
            "range": "± 3136748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2707591596,
            "range": "± 72272866",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 677475762,
            "range": "± 3097592",
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
          "id": "a683f2a00c84a4cd8ca6f1e28151855f8523cf29",
          "message": "chore(deps): update ci dependencies (#305)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T12:37:14Z",
          "tree_id": "63e322022bb73304a31a0a1b6fe540da88fbb61d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a683f2a00c84a4cd8ca6f1e28151855f8523cf29"
        },
        "date": 1722171637113,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9563522,
            "range": "± 64067",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120636034,
            "range": "± 3365538",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2699162069,
            "range": "± 86088721",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 674074534,
            "range": "± 1736095",
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
          "id": "21574613c1f05be91531a254dc7c92dcf2033cc0",
          "message": "chore: fix new Clippy lint",
          "timestamp": "2024-07-28T19:52:05+02:00",
          "tree_id": "9c48cb0879365088de6916c4e73a6afd922b8982",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/21574613c1f05be91531a254dc7c92dcf2033cc0"
        },
        "date": 1722190260034,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9659028,
            "range": "± 70497",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125934443,
            "range": "± 4299259",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2760470949,
            "range": "± 73712028",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 677572767,
            "range": "± 3614744",
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
          "id": "1c294296793f5eaa7bfe943d0c6a891167e2f09e",
          "message": "chore(deps): pin dependencies (#307)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T21:09:49Z",
          "tree_id": "85dd2ad5c7900a8f7c8546a78c0192f26e8c6f87",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1c294296793f5eaa7bfe943d0c6a891167e2f09e"
        },
        "date": 1722202140994,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9597561,
            "range": "± 154588",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 124686171,
            "range": "± 2607013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2728104931,
            "range": "± 70699665",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 676389317,
            "range": "± 2702303",
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
          "id": "8d5dc3dc5660ffd7dabed0b91602f131c848f1a1",
          "message": "chore(deps): pin dependencies (#307)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T23:18:18Z",
          "tree_id": "85dd2ad5c7900a8f7c8546a78c0192f26e8c6f87",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8d5dc3dc5660ffd7dabed0b91602f131c848f1a1"
        },
        "date": 1722209863403,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9681861,
            "range": "± 1463486",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 123171201,
            "range": "± 4634488",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2713189668,
            "range": "± 49884802",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 676444874,
            "range": "± 1986061",
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
          "id": "038e3106265c9bad0616195447cf1dd56af12bda",
          "message": "fix(deps): update rust crate serde_json to v1.0.121 (#306)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T23:21:55Z",
          "tree_id": "a706c69cabe789ae5a97f620c1a03b3701d34139",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/038e3106265c9bad0616195447cf1dd56af12bda"
        },
        "date": 1722210324198,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9751365,
            "range": "± 2374187",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 122890526,
            "range": "± 2842800",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2694822550,
            "range": "± 73419234",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 666911725,
            "range": "± 2497718",
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
          "id": "8d5dc3dc5660ffd7dabed0b91602f131c848f1a1",
          "message": "chore(deps): pin dependencies (#307)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T23:18:18Z",
          "tree_id": "85dd2ad5c7900a8f7c8546a78c0192f26e8c6f87",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8d5dc3dc5660ffd7dabed0b91602f131c848f1a1"
        },
        "date": 1722210544075,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9741347,
            "range": "± 98634",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 126166928,
            "range": "± 3690705",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2721672291,
            "range": "± 64177902",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 680447355,
            "range": "± 4348706",
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
          "id": "038e3106265c9bad0616195447cf1dd56af12bda",
          "message": "fix(deps): update rust crate serde_json to v1.0.121 (#306)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-28T23:21:55Z",
          "tree_id": "a706c69cabe789ae5a97f620c1a03b3701d34139",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/038e3106265c9bad0616195447cf1dd56af12bda"
        },
        "date": 1722210966615,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9447201,
            "range": "± 1187597",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118350864,
            "range": "± 2655400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2695059927,
            "range": "± 63591013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 668912520,
            "range": "± 2827586",
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
          "id": "332caed6d8df7bfc2594cc579affb04e5da31d30",
          "message": "ci: ensure external merge queue PRs get their checks in place",
          "timestamp": "2024-07-29T11:37:49+02:00",
          "tree_id": "43335d04c6fe90374a1ba5a4cd045e3f0f769dde",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/332caed6d8df7bfc2594cc579affb04e5da31d30"
        },
        "date": 1722247300116,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9470302,
            "range": "± 1621537",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118669615,
            "range": "± 3406477",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2727897169,
            "range": "± 67572389",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 667259039,
            "range": "± 3045551",
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
          "id": "fdffe69d488099230ce2131c21779fdd740db495",
          "message": "chore(deps): bump dependencies to get rid of janked crate",
          "timestamp": "2024-07-31T10:12:53+02:00",
          "tree_id": "fee25d6f1903b137b4390cbcfe17264de01a77c9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fdffe69d488099230ce2131c21779fdd740db495"
        },
        "date": 1722415020142,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9760252,
            "range": "± 1224452",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125854159,
            "range": "± 3943657",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2772541975,
            "range": "± 65030495",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 677338269,
            "range": "± 5528468",
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
          "id": "53fa08bf637ab806f3dcca188ef462230c966f6a",
          "message": "chore(deps): update ci dependencies (#309)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-31T08:26:06Z",
          "tree_id": "05375d79848b721954672f8a04ec2ca14e955d3f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/53fa08bf637ab806f3dcca188ef462230c966f6a"
        },
        "date": 1722415760647,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9771205,
            "range": "± 108024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125822231,
            "range": "± 2850523",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2721109960,
            "range": "± 145541525",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 680957053,
            "range": "± 2644824",
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
          "id": "53fa08bf637ab806f3dcca188ef462230c966f6a",
          "message": "chore(deps): update ci dependencies (#309)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-31T08:26:06Z",
          "tree_id": "05375d79848b721954672f8a04ec2ca14e955d3f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/53fa08bf637ab806f3dcca188ef462230c966f6a"
        },
        "date": 1722416418075,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9880085,
            "range": "± 1423991",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127275614,
            "range": "± 2543713",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2773797493,
            "range": "± 39436378",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 680593173,
            "range": "± 2755721",
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
          "id": "1976cc930439f708b66d822da5d1db8a3a93440c",
          "message": "fix(deps): update rust crate bytes to v1.7.0 (#308)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-31T12:27:56Z",
          "tree_id": "ae85b56d84ef39e20135f5df6ddd407403741177",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1976cc930439f708b66d822da5d1db8a3a93440c"
        },
        "date": 1722430134274,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9771298,
            "range": "± 132588",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 128718625,
            "range": "± 3080722",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2687744672,
            "range": "± 50396590",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 679178650,
            "range": "± 3060659",
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
          "id": "1976cc930439f708b66d822da5d1db8a3a93440c",
          "message": "fix(deps): update rust crate bytes to v1.7.0 (#308)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-07-31T12:27:56Z",
          "tree_id": "ae85b56d84ef39e20135f5df6ddd407403741177",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1976cc930439f708b66d822da5d1db8a3a93440c"
        },
        "date": 1722430827090,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10197608,
            "range": "± 66659",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 122776990,
            "range": "± 3436043",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2708839827,
            "range": "± 57155960",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 684421731,
            "range": "± 2266772",
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
          "id": "40d70ef1a65c1f4c1647910c8e30e1700a886cbb",
          "message": "chore: update some dependencies\n\n`sysinfo` requires more work as it introduced breaking changes.",
          "timestamp": "2024-08-01T23:28:40+02:00",
          "tree_id": "0187153d52a932242f28ef04a2e4f1b702dca837",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/40d70ef1a65c1f4c1647910c8e30e1700a886cbb"
        },
        "date": 1722549138419,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9546691,
            "range": "± 1325489",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121990666,
            "range": "± 2516400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2624633755,
            "range": "± 68622806",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 671100096,
            "range": "± 2917139",
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
          "id": "f57ea4fe2db16549a26c1b90bfc0762b29555bac",
          "message": "chore(deps): update taiki-e/install-action digest to 3451569 (#310)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-01T21:42:33Z",
          "tree_id": "20e5fffd93074393034f1597454d1e2ec35a89e1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f57ea4fe2db16549a26c1b90bfc0762b29555bac"
        },
        "date": 1722549712633,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9504029,
            "range": "± 203065",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 122319923,
            "range": "± 2338412",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2641822205,
            "range": "± 70156596",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 669214697,
            "range": "± 2942395",
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
          "id": "f57ea4fe2db16549a26c1b90bfc0762b29555bac",
          "message": "chore(deps): update taiki-e/install-action digest to 3451569 (#310)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-01T21:42:33Z",
          "tree_id": "20e5fffd93074393034f1597454d1e2ec35a89e1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f57ea4fe2db16549a26c1b90bfc0762b29555bac"
        },
        "date": 1722550437437,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9531858,
            "range": "± 56280",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 123491393,
            "range": "± 2354212",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2635219450,
            "range": "± 95868477",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 668686198,
            "range": "± 2473857",
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
          "id": "dec079ac4542ed788f8e3391331dbf561e49c5c6",
          "message": "fix(ci): make version metadata gathering work again",
          "timestamp": "2024-08-03T18:33:28+02:00",
          "tree_id": "5ce3f8ea8a0bc18b37f8189c8bb8e5c9c6ae3687",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/dec079ac4542ed788f8e3391331dbf561e49c5c6"
        },
        "date": 1722704147912,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9728987,
            "range": "± 135165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 131886216,
            "range": "± 1682304",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2757078796,
            "range": "± 65467097",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 674316271,
            "range": "± 3707236",
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
          "id": "17c443316ad149d87483a03ca98297ca57d04302",
          "message": "chore(deps): update ci dependencies (#312)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-04T03:23:59Z",
          "tree_id": "0f1941b80c54c67a4f3ffb48987ec4683b207d7d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/17c443316ad149d87483a03ca98297ca57d04302"
        },
        "date": 1722743179830,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9551880,
            "range": "± 68810",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129895272,
            "range": "± 3872889",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2717196721,
            "range": "± 117393327",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 667794434,
            "range": "± 2990866",
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
          "id": "17c443316ad149d87483a03ca98297ca57d04302",
          "message": "chore(deps): update ci dependencies (#312)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-04T03:23:59Z",
          "tree_id": "0f1941b80c54c67a4f3ffb48987ec4683b207d7d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/17c443316ad149d87483a03ca98297ca57d04302"
        },
        "date": 1722745018049,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9498372,
            "range": "± 212541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 130526180,
            "range": "± 2723379",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2747036249,
            "range": "± 61825666",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 671941452,
            "range": "± 2628085",
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
          "id": "e039bcc221fbca67e51c0a5132e90416bea9d862",
          "message": "chore(deps): update taiki-e/install-action digest to c4b9b42 (#313)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-04T09:31:22Z",
          "tree_id": "68ce4125fc89e9a874db7a427d9273f7cc3adb9e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e039bcc221fbca67e51c0a5132e90416bea9d862"
        },
        "date": 1722764998241,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9460988,
            "range": "± 352057",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 128189601,
            "range": "± 3611738",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2701209556,
            "range": "± 69887180",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 665960996,
            "range": "± 3610961",
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
          "id": "e039bcc221fbca67e51c0a5132e90416bea9d862",
          "message": "chore(deps): update taiki-e/install-action digest to c4b9b42 (#313)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-04T09:31:22Z",
          "tree_id": "68ce4125fc89e9a874db7a427d9273f7cc3adb9e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e039bcc221fbca67e51c0a5132e90416bea9d862"
        },
        "date": 1722765532803,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9488273,
            "range": "± 224150",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129309683,
            "range": "± 2931978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2669562460,
            "range": "± 61685471",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 664122756,
            "range": "± 3765151",
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
          "id": "a704945f45dcd0324915511f7cabee4df0b0ae40",
          "message": "chore(deps): update ci dependencies (#314)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-07T05:08:51Z",
          "tree_id": "04ebbbeb8655aa421b29414501965c4cb2d7ff7f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a704945f45dcd0324915511f7cabee4df0b0ae40"
        },
        "date": 1723008694795,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9666138,
            "range": "± 67293",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 132115175,
            "range": "± 3042788",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2779746239,
            "range": "± 70164068",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 673296770,
            "range": "± 2656868",
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
          "id": "a704945f45dcd0324915511f7cabee4df0b0ae40",
          "message": "chore(deps): update ci dependencies (#314)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-07T05:08:51Z",
          "tree_id": "04ebbbeb8655aa421b29414501965c4cb2d7ff7f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a704945f45dcd0324915511f7cabee4df0b0ae40"
        },
        "date": 1723009671254,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9759214,
            "range": "± 4191770",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121184730,
            "range": "± 2236817",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2627820255,
            "range": "± 35457696",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 667334923,
            "range": "± 3455188",
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
          "id": "d9c38c2e0ffdfbf9d5c7e08d61d81e10fd121936",
          "message": "chore(deps): update dependency tqdm to v4.66.5 (#315)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-07T05:08:58Z",
          "tree_id": "2bb66fda343b227eba475f027c0cfe178770ff69",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d9c38c2e0ffdfbf9d5c7e08d61d81e10fd121936"
        },
        "date": 1723009730177,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9864198,
            "range": "± 87732",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 132093112,
            "range": "± 1714089",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2829709905,
            "range": "± 62251126",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 674707695,
            "range": "± 2826607",
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
          "id": "580a8c524b900c453faa15e90b9e19bed515bb8a",
          "message": "chore(deps): update taiki-e/install-action digest to ada21a8 (#316)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-07T19:04:16Z",
          "tree_id": "b3c7f43449148b82e8eba8ffe7266fe4899e8c52",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/580a8c524b900c453faa15e90b9e19bed515bb8a"
        },
        "date": 1723058570634,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9648073,
            "range": "± 1429832",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120842372,
            "range": "± 2750608",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2627729044,
            "range": "± 35061497",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662815948,
            "range": "± 1725640",
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
          "id": "580a8c524b900c453faa15e90b9e19bed515bb8a",
          "message": "chore(deps): update taiki-e/install-action digest to ada21a8 (#316)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-08-07T19:04:16Z",
          "tree_id": "b3c7f43449148b82e8eba8ffe7266fe4899e8c52",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/580a8c524b900c453faa15e90b9e19bed515bb8a"
        },
        "date": 1723059214530,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9628608,
            "range": "± 41588",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121380005,
            "range": "± 3037166",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2627205493,
            "range": "± 24617538",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 665604488,
            "range": "± 2832339",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}