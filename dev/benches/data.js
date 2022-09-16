window.BENCHMARK_DATA = {
  "lastUpdate": 1663306627894,
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
          "id": "4c1a70763e16dc7fc14846303563ba74592fd2dc",
          "message": "chore(ci): attempt to fix some failures\n\nA Clippy warning was fixed, and maybe another circumstance that made the\nGit version retrieval fail on build scripts fail.",
          "timestamp": "2022-08-06T18:27:09+02:00",
          "tree_id": "19e9ce363a333ce8df6bce4234a74e77fb7be5dc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4c1a70763e16dc7fc14846303563ba74592fd2dc"
        },
        "date": 1659805095897,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10954892,
            "range": "± 387669",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 340456841,
            "range": "± 3104372",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9999335350,
            "range": "± 23145965",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2184030600,
            "range": "± 28007045",
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
          "id": "8830dd1cda3b47214295baf03250502cf8c57dc0",
          "message": "fix(packsquash_cli/build): disable dir owner validation in CI\n\nThis probably was causing git commands to fail when trying to retrieve\nthe semantic version for a commit on CI.",
          "timestamp": "2022-08-06T19:12:56+02:00",
          "tree_id": "8dc86010b5eef2fcf0d3ca14bf2f8e22005d9086",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8830dd1cda3b47214295baf03250502cf8c57dc0"
        },
        "date": 1659808449897,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14160936,
            "range": "± 320953",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 397147089,
            "range": "± 3847669",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12458762928,
            "range": "± 14014582",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2230905498,
            "range": "± 78614594",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "2c4a11ea8be81201caf4a8e6e9437fd031ea8b6f",
          "message": "fix(ci/windows): use winres patch to try to fix exe metadata\n\nwinres is known to not work on recent Rust toolchains due to some linker\nchanges, but there is a PR submitted upstream to fix that. Let's see if\nit fixes our issue of executable metadata not working.",
          "timestamp": "2022-08-07T20:08:28Z",
          "tree_id": "857822ad0d0670519a83e94d87a2c18c220ded4a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2c4a11ea8be81201caf4a8e6e9437fd031ea8b6f"
        },
        "date": 1659905320939,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14040635,
            "range": "± 167132",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 397791929,
            "range": "± 5041999",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12025306352,
            "range": "± 247973816",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2381306120,
            "range": "± 70101000",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "06cd9f25ad9dd27ea6b2a14efbc9a67d021eb8c9",
          "message": "chore(ci/windows): fix cargo deny check and executable description",
          "timestamp": "2022-08-07T20:58:28Z",
          "tree_id": "e90d9c695f3666eb6da74419cd9a391395407eb1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/06cd9f25ad9dd27ea6b2a14efbc9a67d021eb8c9"
        },
        "date": 1659907780748,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15985099,
            "range": "± 663530",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 454031767,
            "range": "± 8295713",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13757700014,
            "range": "± 70490854",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2684163219,
            "range": "± 24663490",
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
          "id": "22f18739e6f7fc8b69dc6135e9927dbf60659040",
          "message": "fix(deps): update rust crate serde to 1.0.143",
          "timestamp": "2022-08-09T02:38:58Z",
          "tree_id": "2605e1cd1e4e3ba68d0f34f7fa4d1ebb956305bc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/22f18739e6f7fc8b69dc6135e9927dbf60659040"
        },
        "date": 1660025531110,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17574031,
            "range": "± 9078561",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 479324188,
            "range": "± 4046554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15278852133,
            "range": "± 294488131",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2778333347,
            "range": "± 64662635",
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
          "id": "5e5d6fc8557621467136410ecfb0595d4713b1ff",
          "message": "fix(pack_file/shader_file): workaround M1 Mac GLSL compiler quirk\n\nIt was discovered by @pau101 that the GLSL compiler distributed with M1\nMacs may consider some if-else construct as ambiguous and fail shader\ncompilation, even though every other known GLSL compiler accepts them\nfine.\n\nThis problem was fixed in the glsl crate fork PackSquash uses, via a PR\nthat's now merged. Update the glsl crate commit used by PackSquash to\nlet it use this fix.",
          "timestamp": "2022-08-09T14:40:53+02:00",
          "tree_id": "3d87f692bfdec03dfd8171049d89a87ff317c03a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5e5d6fc8557621467136410ecfb0595d4713b1ff"
        },
        "date": 1660050822276,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14206573,
            "range": "± 716209",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 411994217,
            "range": "± 8593093",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10508684284,
            "range": "± 393235939",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2227543994,
            "range": "± 42890431",
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
          "id": "7b22e6d3fdc40932e1ba2d20109f4dbfbfd70623",
          "message": "chore(deps): update dependency filelock to v3.8.0",
          "timestamp": "2022-08-10T10:38:55Z",
          "tree_id": "be5fe14c3b6363c84203ccf37b8226fe8ecc0320",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7b22e6d3fdc40932e1ba2d20109f4dbfbfd70623"
        },
        "date": 1660138823644,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14383251,
            "range": "± 250733",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 402016956,
            "range": "± 2966517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12123111087,
            "range": "± 18722117",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2435610236,
            "range": "± 84122408",
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
          "id": "0b4256ce2d7970ba42bd872a46823cc43cd30fdf",
          "message": "fix(deps): update rust crate sysinfo to 0.25.2",
          "timestamp": "2022-08-10T13:07:45Z",
          "tree_id": "4ece8e98b50a4d8acdc94bea12a716c4ba5ac889",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0b4256ce2d7970ba42bd872a46823cc43cd30fdf"
        },
        "date": 1660147146692,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11099385,
            "range": "± 287418",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 347068427,
            "range": "± 1909087",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10184409755,
            "range": "± 226796905",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2222039152,
            "range": "± 76691992",
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
          "id": "a655abfbd2d0ddd0bc95f86fd422a72e7c71efa9",
          "message": "fix(deps): update rust crate ahash to 0.8.0",
          "timestamp": "2022-08-10T15:32:30Z",
          "tree_id": "62cddbca2d43f81cf28973da50e78d387bcdd8f2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a655abfbd2d0ddd0bc95f86fd422a72e7c71efa9"
        },
        "date": 1660158714175,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14103494,
            "range": "± 370659",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 399963117,
            "range": "± 3176599",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12484721250,
            "range": "± 16244576",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2234064157,
            "range": "± 77485035",
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
          "id": "42a51a87c8794093a2b583fb3cd541a0435e57a1",
          "message": "fix(deps): update rust crate futures to 0.3.23",
          "timestamp": "2022-08-14T13:59:43Z",
          "tree_id": "a0919f9f07c909e03fe84c4c5a36e42b07c9d375",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/42a51a87c8794093a2b583fb3cd541a0435e57a1"
        },
        "date": 1660497249207,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13599595,
            "range": "± 8898394",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 413704777,
            "range": "± 9743077",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10633647252,
            "range": "± 305298416",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2269172813,
            "range": "± 64055311",
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
          "id": "6f10240e696665ba1d75c2343e9c2f38bd49129f",
          "message": "fix(deps): update rust crate optivorbis to 0.1.2",
          "timestamp": "2022-08-14T16:24:45Z",
          "tree_id": "c87089a5342dcd0a6abfd4b50229d931cfa3162b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6f10240e696665ba1d75c2343e9c2f38bd49129f"
        },
        "date": 1660504266416,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13428585,
            "range": "± 182427",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 388024229,
            "range": "± 2163361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12514019522,
            "range": "± 244004839",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2274410303,
            "range": "± 85132662",
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
          "id": "2a9f1f0cac8efecc1222243eaf843d3c0a8b02c9",
          "message": "fix(deps): update rust crate wmi to 0.11.1",
          "timestamp": "2022-08-14T18:47:33Z",
          "tree_id": "904305ccad3b18c2b13a66f142c45c51dbb8d6b2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2a9f1f0cac8efecc1222243eaf843d3c0a8b02c9"
        },
        "date": 1660511286092,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16045500,
            "range": "± 1099410",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 467739223,
            "range": "± 17793189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14927560850,
            "range": "± 63108734",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2683272522,
            "range": "± 70913534",
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
          "id": "29b8bde7a293a13ad5e1808cc0b0cc956de63c1a",
          "message": "fix(ci): correct benchmark job checking source out too soon. Some tweaks",
          "timestamp": "2022-08-16T13:05:49+02:00",
          "tree_id": "f924dc5e6f30b84677e8ecf79f5c2bf55bb3f2a8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/29b8bde7a293a13ad5e1808cc0b0cc956de63c1a"
        },
        "date": 1660649979039,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10798638,
            "range": "± 111518",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 363416710,
            "range": "± 11729378",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10576462460,
            "range": "± 43524622",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2590086816,
            "range": "± 16488155",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "32ccd69c5e6cf635d27f42c465dc8b8637efe2a3",
          "message": "chore(cargo_deny): avoid locked yanked dependency",
          "timestamp": "2022-08-18T13:37:41Z",
          "tree_id": "e7aa5ba39636b20deb87fae4483ecffdca785ec3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/32ccd69c5e6cf635d27f42c465dc8b8637efe2a3"
        },
        "date": 1660833007940,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12064809,
            "range": "± 544394",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 394719902,
            "range": "± 11579109",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11955879420,
            "range": "± 54096328",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2655691242,
            "range": "± 14501533",
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
          "id": "230790252aaa42b29f2f29b4187332c19e65b5a3",
          "message": "chore(deps): bump locked dependency to avoid using yanked crate",
          "timestamp": "2022-08-20T19:59:44+02:00",
          "tree_id": "43ade1807419c23117544bfb4594e0dc3e3a717e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/230790252aaa42b29f2f29b4187332c19e65b5a3"
        },
        "date": 1661020533553,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15237450,
            "range": "± 917262",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 447188292,
            "range": "± 10965276",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12076237077,
            "range": "± 115453543",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3005010925,
            "range": "± 55142308",
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
          "id": "d00bfdcb42e10ec3196c452a578aad3503c628e9",
          "message": "fix(deps): update rust crate serde to 1.0.144",
          "timestamp": "2022-08-21T05:15:14Z",
          "tree_id": "1c3fe674f853834410b0e4ee280fed56c89ec41e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d00bfdcb42e10ec3196c452a578aad3503c628e9"
        },
        "date": 1661080427607,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14085905,
            "range": "± 566079",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 449755179,
            "range": "± 20540991",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13849456221,
            "range": "± 193488674",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3090504478,
            "range": "± 72049672",
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
          "id": "f16c38bd7e6657648ea2d87db2be7dd8ebe637fe",
          "message": "fix(deps): update rust crate sysinfo to 0.25.3",
          "timestamp": "2022-08-21T14:43:43Z",
          "tree_id": "45bff62d26aa03437e272506bf7fde715253b449",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f16c38bd7e6657648ea2d87db2be7dd8ebe637fe"
        },
        "date": 1661102016784,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13591198,
            "range": "± 763037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 428632109,
            "range": "± 12951642",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11276159858,
            "range": "± 287509678",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2785741494,
            "range": "± 72522993",
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
          "id": "9938f9af736afb275e18c8e1c4257fb241f94ac9",
          "message": "fix(deps): update rust crate serde_json to 1.0.85",
          "timestamp": "2022-08-21T23:05:28Z",
          "tree_id": "0a1bd19be93b1021714ac7412af049a030b2f2fc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9938f9af736afb275e18c8e1c4257fb241f94ac9"
        },
        "date": 1661134897878,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12468697,
            "range": "± 334841",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 384307248,
            "range": "± 13486889",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10538973841,
            "range": "± 47197266",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2720184504,
            "range": "± 16186544",
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
          "id": "86f5ac3d74de10d05fc278406def9ee525e6a676",
          "message": "fix(deps): update rust crate imagequant to 4.0.2",
          "timestamp": "2022-08-22T12:11:23Z",
          "tree_id": "23d5e96bbe56dd5c2e813d836218a51279e375f1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/86f5ac3d74de10d05fc278406def9ee525e6a676"
        },
        "date": 1661179605276,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10822334,
            "range": "± 350681",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 359842615,
            "range": "± 12451105",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10188530377,
            "range": "± 47190968",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2596961399,
            "range": "± 24110891",
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
          "id": "b1ffd6793126f65fbe1344c1d42003b061169faa",
          "message": "chore(deps): update dependency urllib3 to v1.26.12",
          "timestamp": "2022-08-22T22:30:10Z",
          "tree_id": "6806c9bf8f7cda539b4e7ab28497af3a343a7083",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b1ffd6793126f65fbe1344c1d42003b061169faa"
        },
        "date": 1661210156868,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13908087,
            "range": "± 713678",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 377842574,
            "range": "± 19991372",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11563315063,
            "range": "± 47346561",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2585918842,
            "range": "± 689194519",
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
          "id": "cd8a7e5cbf4588e9361bfc5f2459225852c1ae4e",
          "message": "chore(deps): update rust crate time to 0.3.14",
          "timestamp": "2022-08-25T04:18:15Z",
          "tree_id": "3a7676b5d006bafe125843483db3fd7c9941c126",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cd8a7e5cbf4588e9361bfc5f2459225852c1ae4e"
        },
        "date": 1661426518575,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15110342,
            "range": "± 413733",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 498172762,
            "range": "± 22128684",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14955926885,
            "range": "± 77344185",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3369705447,
            "range": "± 27234388",
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
          "id": "f19adc6f21d59a69aca4aa067defb88df76c7060",
          "message": "fix(deps): update rust crate imagequant to 4.0.4",
          "timestamp": "2022-08-25T10:47:38Z",
          "tree_id": "8bd0028f1cf6023401aadac496ddaee71c57e19b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f19adc6f21d59a69aca4aa067defb88df76c7060"
        },
        "date": 1661437104067,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12026620,
            "range": "± 476812",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 398746540,
            "range": "± 10830073",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12367560232,
            "range": "± 175284882",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2662216054,
            "range": "± 38128807",
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
          "id": "561da144cf96f9079bfa00a346ad3009298ebe9e",
          "message": "fix(deps): update rust crate futures to 0.3.24",
          "timestamp": "2022-08-29T15:07:57Z",
          "tree_id": "c4cda944bc9d4bf36626beb820b680df98b95bb0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/561da144cf96f9079bfa00a346ad3009298ebe9e"
        },
        "date": 1661799074579,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11967746,
            "range": "± 355033",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 395722429,
            "range": "± 12192324",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12280763883,
            "range": "± 146894580",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2635657193,
            "range": "± 16582416",
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
          "id": "4ebe9db925d63ea1a018cf0d1eaaec47dc41bc9c",
          "message": "fix(deps): update rust crate sysinfo to 0.26.0",
          "timestamp": "2022-08-29T18:02:38Z",
          "tree_id": "43598b3deb2e3b6b6650da97077ad87bb4585529",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4ebe9db925d63ea1a018cf0d1eaaec47dc41bc9c"
        },
        "date": 1661808116881,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11899361,
            "range": "± 119830",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 390641819,
            "range": "± 10918171",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12149480752,
            "range": "± 41881667",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2604295702,
            "range": "± 15319091",
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
          "id": "9960905afb9e49d13e28e24b70445f60e58442a5",
          "message": "chore(deps): update rust crate pretty_assertions to 1.3.0",
          "timestamp": "2022-08-30T14:07:42Z",
          "tree_id": "7d59634a032c105f21a48d5659a120aea528ddd3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9960905afb9e49d13e28e24b70445f60e58442a5"
        },
        "date": 1661883031733,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12301057,
            "range": "± 304454",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 398476313,
            "range": "± 11101730",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11540833217,
            "range": "± 36477039",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2679850351,
            "range": "± 20989727",
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
          "id": "d05dd3775a029f4610e354656e9477e1a01521fb",
          "message": "chore(README.md): fix CI status badge",
          "timestamp": "2022-08-30T21:43:49+02:00",
          "tree_id": "cd9b4e669222d4e31692ac9377e24040ee4bd04a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d05dd3775a029f4610e354656e9477e1a01521fb"
        },
        "date": 1661891201839,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11977237,
            "range": "± 511232",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 384838242,
            "range": "± 12028843",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11511204965,
            "range": "± 49166857",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2660054506,
            "range": "± 17422316",
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
          "id": "5ff70e294e9babbe0909b512f8f11b6b37b93cc7",
          "message": "fix(deps): update rust crate thiserror to 1.0.33",
          "timestamp": "2022-08-31T05:13:46Z",
          "tree_id": "afc004c489b8537674b8a9da4fa1334c1491ab76",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5ff70e294e9babbe0909b512f8f11b6b37b93cc7"
        },
        "date": 1661948916916,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12087895,
            "range": "± 319234",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 392674677,
            "range": "± 7930924",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11573161655,
            "range": "± 50347009",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2704362478,
            "range": "± 18106687",
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
          "id": "ffc01c884d92245620fcde83abcb0059bb75b097",
          "message": "fix(deps): update rust crate sysinfo to 0.26.1",
          "timestamp": "2022-08-31T18:27:10Z",
          "tree_id": "4534e22fb34540ea23a8eb96eec3248391de9729",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ffc01c884d92245620fcde83abcb0059bb75b097"
        },
        "date": 1661984080887,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12164793,
            "range": "± 442925",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 406859782,
            "range": "± 10251940",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11961091580,
            "range": "± 36426184",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2714627592,
            "range": "± 17730694",
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
          "id": "db8bb85ada6c18eb1429e5393e9df7559e151ce1",
          "message": "chore(clippy): fix new lint",
          "timestamp": "2022-09-03T13:26:16+02:00",
          "tree_id": "c37f9bf39d65c724e919a8ca4b08b3fbcc838391",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/db8bb85ada6c18eb1429e5393e9df7559e151ce1"
        },
        "date": 1662206502578,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14683281,
            "range": "± 507672",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 444448019,
            "range": "± 19237098",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12063882494,
            "range": "± 65234504",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2895962993,
            "range": "± 78715930",
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
          "id": "593781c5a05e8ebb68656108d3956822bbee032c",
          "message": "fix(deps): update rust crate tokio to 1.21.0 (#151)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2022-09-03T13:54:01+02:00",
          "tree_id": "c5e3a462731b8907cae3ae5e1aaf0da8af03975a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/593781c5a05e8ebb68656108d3956822bbee032c"
        },
        "date": 1662207913579,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15099284,
            "range": "± 848104",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 446435172,
            "range": "± 19100293",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13995217528,
            "range": "± 65782609",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3058108879,
            "range": "± 24619164",
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
          "id": "7ebffc62294d21b1b37d02db8aac1494d1216459",
          "message": "chore(audio_file): minor refactors",
          "timestamp": "2022-09-03T16:06:29+02:00",
          "tree_id": "3af78ebace1a8bde9bcc91cc821dae5c24e66673",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7ebffc62294d21b1b37d02db8aac1494d1216459"
        },
        "date": 1662215803569,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11914469,
            "range": "± 133055",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 389891088,
            "range": "± 12980917",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12173435430,
            "range": "± 51025112",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2626908550,
            "range": "± 68679236",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "12679d2ad7369b79dfc147ef9f9cba5fa7dfcc0e",
          "message": "chore(deps): update oxipng and zopfli dependencies\n\nThe bureaucracy of submitting the needed upstream changes was just done.\nMinimize software entropy by updating Zopfli to its latest version,\nwhich has everything we need, and using the OxiPNG changes that were\nsubmitted for upstreaming.",
          "timestamp": "2022-09-03T16:33:52Z",
          "tree_id": "b4337c2c9039ed0bff95aeef053bda8b8fe12413",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12679d2ad7369b79dfc147ef9f9cba5fa7dfcc0e"
        },
        "date": 1662225540683,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9208328,
            "range": "± 132424",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 356746091,
            "range": "± 6449091",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10331494674,
            "range": "± 32535822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2583122031,
            "range": "± 60226015",
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
          "id": "795621fe5c663992e3cf28efc1beb547b8f8ee18",
          "message": "fix(deps): update rust crate wmi to 0.11.2 (#152)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2022-09-03T18:58:01+02:00",
          "tree_id": "82353cfdefda089d106d3032cf19b050df3878e4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/795621fe5c663992e3cf28efc1beb547b8f8ee18"
        },
        "date": 1662226194882,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13862688,
            "range": "± 936642",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 457320462,
            "range": "± 18945295",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12118397733,
            "range": "± 164540358",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2942486858,
            "range": "± 75452749",
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
          "id": "c049d8fff6e29c32733df658b20fd95f525ecf22",
          "message": "chore(deps): update dependency tqdm to v4.64.1",
          "timestamp": "2022-09-03T16:58:20Z",
          "tree_id": "9235622d53c9474018f8bbad9f5dce59f43ee5c2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c049d8fff6e29c32733df658b20fd95f525ecf22"
        },
        "date": 1662239397794,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12758177,
            "range": "± 618157",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 476506640,
            "range": "± 12482174",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13604433091,
            "range": "± 49532757",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3179653918,
            "range": "± 31874937",
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
          "id": "09efc2c8c1e623833ae90c42dfa4133be1b7f6de",
          "message": "chore(FUNDING): add GH sponsors link",
          "timestamp": "2022-09-04T01:11:17+02:00",
          "tree_id": "1a485eddf21744cc51a441252393a4cbd2596569",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/09efc2c8c1e623833ae90c42dfa4133be1b7f6de"
        },
        "date": 1662248717673,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10128402,
            "range": "± 402032",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 382806733,
            "range": "± 17450743",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11311343223,
            "range": "± 35409082",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2636004734,
            "range": "± 21764377",
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
          "id": "6318c2704f80303a4d9ffad33a1d2178fdb4db42",
          "message": "fix(deps): update rust crate sysinfo to 0.26.2",
          "timestamp": "2022-09-04T00:29:08Z",
          "tree_id": "d376294bf398e4d99eeb8785181de00371ed8bad",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6318c2704f80303a4d9ffad33a1d2178fdb4db42"
        },
        "date": 1662266739080,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9344518,
            "range": "± 165685",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 362581779,
            "range": "± 14798399",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10094533650,
            "range": "± 49093540",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2550772273,
            "range": "± 18575942",
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
          "id": "2c1c8cff4e60e7681d0a8def70c48bb0711639b4",
          "message": "fix(deps): update rust crate thiserror to 1.0.34",
          "timestamp": "2022-09-04T23:52:33Z",
          "tree_id": "cd1001c21e82d1ad5a33523b4b662c1045a9d794",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2c1c8cff4e60e7681d0a8def70c48bb0711639b4"
        },
        "date": 1662350414594,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10631517,
            "range": "± 637987",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 389207435,
            "range": "± 7917084",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11750214972,
            "range": "± 36195243",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671039974,
            "range": "± 15310903",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "269ae867ff6f17501150d1993479c632b1d580c1",
          "message": "chore(deps): update OxiPNG dependency\n\nSome patches of ours were merged upstream, and a new release has been\npublished. Our patched fork was updated accordingly.",
          "timestamp": "2022-09-05T20:29:49Z",
          "tree_id": "865dcd335d66401e204a34c8080c1f51fcec504b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/269ae867ff6f17501150d1993479c632b1d580c1"
        },
        "date": 1662412586261,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14388062,
            "range": "± 761664",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 471945660,
            "range": "± 18444951",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12694125927,
            "range": "± 121765988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3155993568,
            "range": "± 194571653",
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
          "id": "00895ca9a6d29cbe4f722628c37b3457c9399186",
          "message": "fix(deps): update rust crate png to 0.17.6",
          "timestamp": "2022-09-05T20:53:36Z",
          "tree_id": "2d41fefe90fbad4c898aae129660847fa9c577eb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/00895ca9a6d29cbe4f722628c37b3457c9399186"
        },
        "date": 1662428592303,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10491933,
            "range": "± 94228",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 389887669,
            "range": "± 14731204",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11397146232,
            "range": "± 53083549",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2670827441,
            "range": "± 11883353",
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
          "id": "57acf2f50870b894686909909059ed40417cddf0",
          "message": "fix(deps): update rust crate zopfli to 0.7.1",
          "timestamp": "2022-09-07T13:29:10Z",
          "tree_id": "0e6597dec5eb61ec37882f3ed540336270305e0d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/57acf2f50870b894686909909059ed40417cddf0"
        },
        "date": 1662576284910,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13258526,
            "range": "± 669202",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 439712114,
            "range": "± 15918505",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11635840966,
            "range": "± 128478011",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2894473048,
            "range": "± 89523585",
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
          "id": "a00eabebbcad47a51930e8691ea6f7c5f386a9c5",
          "message": "fix(deps): update rust crate oxipng to 6.0.1",
          "timestamp": "2022-09-07T22:54:43Z",
          "tree_id": "cc067f14f619cabede3da367fd2571b26c828704",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a00eabebbcad47a51930e8691ea6f7c5f386a9c5"
        },
        "date": 1662593126768,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9386218,
            "range": "± 61551",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 363603485,
            "range": "± 5313109",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10164199273,
            "range": "± 62946665",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2581913280,
            "range": "± 79881654",
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
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "3b98e35c210baf22eacf5ad30bd9f359a68b79e2",
          "message": "chore(deps): update OxiPNG to 6.0.1\n\nThis version contains a critical bugfix that affects DEFLATE stream\ncorrectness when using Zopfli.",
          "timestamp": "2022-09-08T11:18:34Z",
          "tree_id": "3e806dfcf721f993635d1880214b3b11871f16c7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3b98e35c210baf22eacf5ad30bd9f359a68b79e2"
        },
        "date": 1662638926559,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9587000,
            "range": "± 213094",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 362866396,
            "range": "± 4083156",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10534371513,
            "range": "± 34832259",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2596194583,
            "range": "± 45283718",
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
          "id": "00e9a91eedf3e0d03cbd698442b183fb25221c22",
          "message": "fix(deps): update rust crate tokio-util to 0.7.4",
          "timestamp": "2022-09-08T14:15:37Z",
          "tree_id": "50582a3f096d505741d3d53e6ccbc3099f6fd0ed",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/00e9a91eedf3e0d03cbd698442b183fb25221c22"
        },
        "date": 1662715298286,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10395229,
            "range": "± 99687",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 373360766,
            "range": "± 16395907",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12058518098,
            "range": "± 33692806",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2597895389,
            "range": "± 9184101",
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
          "id": "fc49415f6e4709aa21d1ca5bb5bbb143cb070cf0",
          "message": "chore(deps): update dependency certifi to v2022.6.15.1",
          "timestamp": "2022-09-09T18:16:13Z",
          "tree_id": "b4897e54fb8de1044168da308759bb5db26c39b3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fc49415f6e4709aa21d1ca5bb5bbb143cb070cf0"
        },
        "date": 1662761833773,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9468841,
            "range": "± 265720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 365350979,
            "range": "± 7030035",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10471625897,
            "range": "± 16275100",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2592399091,
            "range": "± 12486253",
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
          "id": "c7d065f3af18655517a905a35a8dc6a7ad764078",
          "message": "fix(deps): update rust crate tokio to 1.21.1",
          "timestamp": "2022-09-13T12:26:32Z",
          "tree_id": "e33308a711914d86db261b440787a0d9e5dd8be6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c7d065f3af18655517a905a35a8dc6a7ad764078"
        },
        "date": 1663086852543,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11063166,
            "range": "± 629361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 351269984,
            "range": "± 16826588",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9608883217,
            "range": "± 142198541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2280409025,
            "range": "± 42868486",
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
          "id": "78a02e6608e68507970702b0dfe3d2ba7f058221",
          "message": "fix(deps): update rust crate itertools to 0.10.4",
          "timestamp": "2022-09-13T15:48:05Z",
          "tree_id": "c2e64af69fc77d4b03e913ff5c18c3853661bf23",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/78a02e6608e68507970702b0dfe3d2ba7f058221"
        },
        "date": 1663099681635,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8497475,
            "range": "± 2249995",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 462470285,
            "range": "± 17183508",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14418989339,
            "range": "± 14473058",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2876496703,
            "range": "± 8130709",
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
          "id": "f23c981c115067346a755ac15651af6d460ed3f7",
          "message": "fix(deps): update rust crate thiserror to 1.0.35",
          "timestamp": "2022-09-13T19:37:22Z",
          "tree_id": "0ef4a2e3f8ba37413fa3ec6397f9f268e9e182ee",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f23c981c115067346a755ac15651af6d460ed3f7"
        },
        "date": 1663112135294,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10221908,
            "range": "± 850159",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 390786897,
            "range": "± 10445341",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10350275626,
            "range": "± 594200406",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2349932988,
            "range": "± 51680972",
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
          "id": "7f7b9642bd6d0f6fa2869fe35a36e6427c0651dd",
          "message": "chore(deps): update helper python scripts",
          "timestamp": "2022-09-14T02:07:53Z",
          "tree_id": "dc44f21178dddf2072531f3d6cbbd2601930dbfe",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7f7b9642bd6d0f6fa2869fe35a36e6427c0651dd"
        },
        "date": 1663197014121,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9218871,
            "range": "± 128086",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 352422623,
            "range": "± 10620798",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10476783595,
            "range": "± 30215371",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2577607899,
            "range": "± 31483343",
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
          "id": "e9b9f485650e56f06331934d64fdd2ca20203b2c",
          "message": "chore(deps): update dependency certifi to v2022.9.14",
          "timestamp": "2022-09-14T22:13:09Z",
          "tree_id": "9a857892bbda7a798a8222ce8b6050f2930b259c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e9b9f485650e56f06331934d64fdd2ca20203b2c"
        },
        "date": 1663207480627,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12482490,
            "range": "± 729016",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 425108891,
            "range": "± 13047741",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11575051079,
            "range": "± 131747639",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2875833798,
            "range": "± 47886320",
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
          "id": "42269c98f0de79e50b654daad676bc949e335c45",
          "message": "fix(deps): update rust crate patricia_tree to 0.3.2",
          "timestamp": "2022-09-16T01:28:43Z",
          "tree_id": "9f5b0fb0b2089d82978d1a4b500566724ac82531",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/42269c98f0de79e50b654daad676bc949e335c45"
        },
        "date": 1663306626744,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9495425,
            "range": "± 336705",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 371221578,
            "range": "± 4569618",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10565570905,
            "range": "± 39449789",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2597225689,
            "range": "± 20896241",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}