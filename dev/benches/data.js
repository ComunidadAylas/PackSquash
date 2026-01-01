window.BENCHMARK_DATA = {
  "lastUpdate": 1767305023670,
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
          "id": "22406c0cf55cc67af47541a0113bb06ca6268083",
          "message": "chore(deps): update helper python scripts (#364)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-13T19:47:39Z",
          "tree_id": "d5eaaaa21c46578b8615a55feb6e067d91c01964",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/22406c0cf55cc67af47541a0113bb06ca6268083"
        },
        "date": 1752437060260,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10415654,
            "range": "± 292945",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115333777,
            "range": "± 1936292",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 785878176,
            "range": "± 15441341",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 765096375,
            "range": "± 4134134",
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
          "id": "89dee4d14f9a647850b86865d1359d5f76ba8d4a",
          "message": "chore: migrate embedded links away from Firebase Dynamic Links\n\nAs it'd be expected with almost every nice Google product, Firebase\nDynamic Links is being discontinued for good on August 25, 2025, the\ntime when its links will finally stop working.\n\nLuckily, things have changed since I originally set up those\nredirections in 2021, and the PackSquash project now controls its own\ndomain. Thus, we can just migrate to the already existing GitHub\nPages website, which can be used for redirections.\n\nThe only thing I may miss from Firebase are its analytics of how many\nhits the redirects receive over time, but frankly, I've barely looked at\nthem during these years, and if they are ever really necessary, we can\njust migrate to a more featureful static pages host, like Cloudflare\nPages.",
          "timestamp": "2025-07-13T22:00:15+02:00",
          "tree_id": "8b5f414eb5cb574e8a446f12f5c768fe97c550fd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/89dee4d14f9a647850b86865d1359d5f76ba8d4a"
        },
        "date": 1752437786143,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10107651,
            "range": "± 133266",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114686419,
            "range": "± 1745547",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 776882009,
            "range": "± 22264464",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 759752334,
            "range": "± 4070663",
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
          "id": "27db99653520d064e14abe3df8d51e194954050e",
          "message": "chore: migrate embedded links away from Firebase Dynamic Links\n\nAs it'd be expected with almost every nice Google product, Firebase\nDynamic Links is being discontinued for good on August 25, 2025, the\ntime when its links will finally stop working.\n\nLuckily, things have changed since I originally set up those\nredirections in 2021, and the PackSquash project now controls its own\ndomain. Thus, we can just migrate to the already existing GitHub\nPages website, which can be used for redirections.\n\nThe only thing I may miss from Firebase are its analytics of how many\nhits the redirects receive over time, but frankly, I've barely looked at\nthem during these years, and if they are ever really necessary, we can\njust migrate to a more featureful static pages host, like Cloudflare\nPages.",
          "timestamp": "2025-07-13T22:07:08+02:00",
          "tree_id": "b95af46834a4392b817d0250325a4cfc1af73d31",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/27db99653520d064e14abe3df8d51e194954050e"
        },
        "date": 1752438201793,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9638182,
            "range": "± 342593",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 112836164,
            "range": "± 1896838",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 784267569,
            "range": "± 10218036",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 759686548,
            "range": "± 2680929",
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
          "id": "3d74cb457f704f4b8b57452cb7d789df0563157d",
          "message": "art: use PackSquash Kawaii logo by @Kenny-Hui\n\n@Kenny-Hui has graciously contributed this beautiful logo they made\nusing free software to the project, releasing it under a CC0 license.",
          "timestamp": "2025-07-13T22:49:34+02:00",
          "tree_id": "c63e738fcd990912c38640f9f3711ee6243e4cdd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3d74cb457f704f4b8b57452cb7d789df0563157d"
        },
        "date": 1752440757543,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9821192,
            "range": "± 244618",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114413046,
            "range": "± 1371275",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 793102306,
            "range": "± 15197208",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 762286048,
            "range": "± 2951336",
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
          "id": "33bd11f7126b24a6be7cc2fe179dfb7003917ee0",
          "message": "docs(README): rename contributor as per their request",
          "timestamp": "2025-07-13T23:23:37+02:00",
          "tree_id": "2ff7c2dfbb3499949a20947726e2ee9bf0ed847a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/33bd11f7126b24a6be7cc2fe179dfb7003917ee0"
        },
        "date": 1752442801815,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9815994,
            "range": "± 78987",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114841693,
            "range": "± 1594108",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 768894555,
            "range": "± 20073899",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 760937023,
            "range": "± 4093082",
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
          "id": "36cc401b5b893bbad0fe547463df3b3f9900b82e",
          "message": "chore: update remaining Rust dependencies",
          "timestamp": "2025-07-13T23:37:55+02:00",
          "tree_id": "be720cef1ad745e2d3ab1dee9f583c1af7b8b46d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36cc401b5b893bbad0fe547463df3b3f9900b82e"
        },
        "date": 1752443728444,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10167461,
            "range": "± 191346",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114074759,
            "range": "± 1397931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 754842680,
            "range": "± 16821036",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 758833638,
            "range": "± 2918221",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "scnmed.g@gmail.com",
            "name": "Chen",
            "username": "ChenCMD"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b4ccbfe35337fb36bdac27c92c2a2fddb563eb37",
          "message": "fix: ensure `.mcfunction` files named only with extension are not omitted (#366)",
          "timestamp": "2025-07-13T21:50:17Z",
          "tree_id": "b3a5c7bc219512a5784eded1b426faa831fc3db0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b4ccbfe35337fb36bdac27c92c2a2fddb563eb37"
        },
        "date": 1752444395078,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10197927,
            "range": "± 62813",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113626517,
            "range": "± 1363771",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 763715489,
            "range": "± 13117517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 756036268,
            "range": "± 3742022",
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
          "id": "461d62814184235ce2cfbd6dadb3a2c0eb98bf4a",
          "message": "docs(README): add @ChenCMD to contributors list",
          "timestamp": "2025-07-13T23:52:28+02:00",
          "tree_id": "434bdfce6acc52a6b080f7991d5740c13e22b8ab",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/461d62814184235ce2cfbd6dadb3a2c0eb98bf4a"
        },
        "date": 1752444539954,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10177932,
            "range": "± 37637",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114031222,
            "range": "± 974148",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 755562359,
            "range": "± 17363413",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 756364091,
            "range": "± 3529073",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "scnmed.g@gmail.com",
            "name": "Chen",
            "username": "ChenCMD"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1bdfd6367694ff160c25a21d2d9792ac6ce0338c",
          "message": "fix: ensure `.mcfunction` files named only with extension are not omitted (#366)",
          "timestamp": "2025-07-13T21:50:17Z",
          "tree_id": "be0279dfc6c3f066bc4fc61e27ac0cc191823b20",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1bdfd6367694ff160c25a21d2d9792ac6ce0338c"
        },
        "date": 1752444857636,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9707961,
            "range": "± 82462",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113122954,
            "range": "± 832427",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 753844034,
            "range": "± 16501522",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 752444356,
            "range": "± 2270074",
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
          "id": "991c589fc4666a5ed15556eb3222963d6e6f7112",
          "message": "docs(README): fix CI status badge",
          "timestamp": "2025-07-14T00:01:12+02:00",
          "tree_id": "eec46bc6f838cf06eb70be45603c68de2120f0eb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/991c589fc4666a5ed15556eb3222963d6e6f7112"
        },
        "date": 1752445037107,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9715811,
            "range": "± 309975",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115464818,
            "range": "± 1564668",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 773250197,
            "range": "± 12997282",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 760418690,
            "range": "± 3711845",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "scnmed.g@gmail.com",
            "name": "Chen",
            "username": "ChenCMD"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c3578dc3e8f3f12cab46bc6e2a844d6fd95ef26f",
          "message": "fix: ensure `.mcfunction` files named only with extension are not omitted (#366)",
          "timestamp": "2025-07-13T21:50:17Z",
          "tree_id": "8375b58f3ff2c9e63b6aef8e735124ed8ef47085",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c3578dc3e8f3f12cab46bc6e2a844d6fd95ef26f"
        },
        "date": 1752445189249,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9562336,
            "range": "± 59173",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 112407790,
            "range": "± 1269167",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 760864514,
            "range": "± 20002305",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 753123287,
            "range": "± 2651554",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "scnmed.g@gmail.com",
            "name": "Chen",
            "username": "ChenCMD"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c3578dc3e8f3f12cab46bc6e2a844d6fd95ef26f",
          "message": "fix: ensure `.mcfunction` files named only with extension are not omitted (#366)",
          "timestamp": "2025-07-13T21:50:17Z",
          "tree_id": "8375b58f3ff2c9e63b6aef8e735124ed8ef47085",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c3578dc3e8f3f12cab46bc6e2a844d6fd95ef26f"
        },
        "date": 1752445737509,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9667102,
            "range": "± 145621",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 112201936,
            "range": "± 792652",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 761976871,
            "range": "± 16198375",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 752889056,
            "range": "± 3197319",
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
          "id": "afabc91c96da2538e963c1066f5b6ba2c3e98ef0",
          "message": "docs(CHANGELOG): add entry for PR #366",
          "timestamp": "2025-07-28T12:37:23+02:00",
          "tree_id": "e071e4b25d7d80c0f257af271d728a883031c5a5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/afabc91c96da2538e963c1066f5b6ba2c3e98ef0"
        },
        "date": 1753700416430,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9813682,
            "range": "± 135299",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114483556,
            "range": "± 1642048",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 767544306,
            "range": "± 19660539",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 755089952,
            "range": "± 5859240",
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
          "id": "c45b6b1cf7e6d4ac6934c750aacd1b29b9d41caa",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-08-01T02:20:57Z",
          "tree_id": "4148bce5fbeeeb7346d19b5303790c62b8b1cee3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c45b6b1cf7e6d4ac6934c750aacd1b29b9d41caa"
        },
        "date": 1754025295049,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10345304,
            "range": "± 405984",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115017838,
            "range": "± 1074160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 789064965,
            "range": "± 14118763",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 759077142,
            "range": "± 2923498",
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
          "id": "909d3e96848f5edf93450db2664da4b796c359fe",
          "message": "chore: update dependencies",
          "timestamp": "2025-08-26T15:25:57+02:00",
          "tree_id": "89572739fdb49529143752bb14d41bbe63e7077d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/909d3e96848f5edf93450db2664da4b796c359fe"
        },
        "date": 1756216096982,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9881551,
            "range": "± 169049",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 116593487,
            "range": "± 2490991",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 773711217,
            "range": "± 8728031",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693445995,
            "range": "± 3186103",
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
          "id": "17c7984d6064eeda5ac8343b97146c6437479e6d",
          "message": "fix(docker): attempt to properly attach image metadata for GHCR",
          "timestamp": "2025-08-27T01:20:24+02:00",
          "tree_id": "5f85e61061ca3f933e4f376b17e080bba2511d6f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/17c7984d6064eeda5ac8343b97146c6437479e6d"
        },
        "date": 1756251473517,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10301199,
            "range": "± 215749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 116390114,
            "range": "± 782267",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 778028426,
            "range": "± 15288098",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 697766307,
            "range": "± 1630211",
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
          "id": "a2c8bfbd0b8be47497c8e4227b030e40ced205d6",
          "message": "fix(ci/docker): re-add context field",
          "timestamp": "2025-08-28T02:04:20+02:00",
          "tree_id": "a1e19b882b6c391fa26b12997d17cf3910468ea5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a2c8bfbd0b8be47497c8e4227b030e40ced205d6"
        },
        "date": 1756340850435,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10104870,
            "range": "± 467797",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139540368,
            "range": "± 4422339",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 753969907,
            "range": "± 11524900",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 697266492,
            "range": "± 3482016",
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
          "id": "223363f76142e1065891179b298d16139aaf6c7c",
          "message": "chore(deps): full round of Rust dependency upgrades",
          "timestamp": "2025-09-22T18:29:53+02:00",
          "tree_id": "dc505a70ad05229e7953ac8600842d6bf66431fa",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/223363f76142e1065891179b298d16139aaf6c7c"
        },
        "date": 1758560154299,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9881198,
            "range": "± 367240",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139643525,
            "range": "± 2054229",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 755013293,
            "range": "± 13590178",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695192634,
            "range": "± 2876512",
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
          "id": "5fec9a0fe54b4beb91bbbc7e57e565a082153165",
          "message": "chore: update Rust dependencies",
          "timestamp": "2025-10-05T17:54:23+02:00",
          "tree_id": "02c67fcf69955e60be91614b84a71ca4c4709794",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5fec9a0fe54b4beb91bbbc7e57e565a082153165"
        },
        "date": 1759681288532,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9720010,
            "range": "± 101930",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140345626,
            "range": "± 3270822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 821268129,
            "range": "± 9027147",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693721099,
            "range": "± 3078189",
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
          "id": "a850c9918e365034b2baf2bb561a1a7b56dfd581",
          "message": "chore: reformat TOML files with tombi",
          "timestamp": "2025-10-05T17:59:16+02:00",
          "tree_id": "d762620d3efd6f7755de1bffe3274d7f7b6c1e37",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a850c9918e365034b2baf2bb561a1a7b56dfd581"
        },
        "date": 1759681801544,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10206487,
            "range": "± 46398",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139319615,
            "range": "± 2422996",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 813407274,
            "range": "± 10605550",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695532068,
            "range": "± 4878564",
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
          "id": "65a6f79043b2ce5f6a7871fb2dc30fad37603e19",
          "message": "chore(deps): update ci dependencies (#370)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-05T16:26:38Z",
          "tree_id": "d9af026bdd57c32659172f28e0ac68e143eedb67",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/65a6f79043b2ce5f6a7871fb2dc30fad37603e19"
        },
        "date": 1759682595367,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9592230,
            "range": "± 530880",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141060010,
            "range": "± 3922744",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 823406250,
            "range": "± 11975228",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 694021079,
            "range": "± 2681400",
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
          "id": "65a6f79043b2ce5f6a7871fb2dc30fad37603e19",
          "message": "chore(deps): update ci dependencies (#370)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-05T16:26:38Z",
          "tree_id": "d9af026bdd57c32659172f28e0ac68e143eedb67",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/65a6f79043b2ce5f6a7871fb2dc30fad37603e19"
        },
        "date": 1759683221713,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9891868,
            "range": "± 388842",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140042179,
            "range": "± 2484168",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 822097039,
            "range": "± 6984807",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 696993460,
            "range": "± 3029488",
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
          "id": "63fdced185c714ba3298268ff711644b35dd20a3",
          "message": "chore(deps): update helper python scripts (#369)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-05T16:48:54Z",
          "tree_id": "6844028521594639175d2ede0bc779aa99f0e910",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63fdced185c714ba3298268ff711644b35dd20a3"
        },
        "date": 1759684025134,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9727141,
            "range": "± 83713",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139236676,
            "range": "± 2145975",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 821849804,
            "range": "± 9142741",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695123879,
            "range": "± 2339269",
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
          "id": "63fdced185c714ba3298268ff711644b35dd20a3",
          "message": "chore(deps): update helper python scripts (#369)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-05T16:48:54Z",
          "tree_id": "6844028521594639175d2ede0bc779aa99f0e910",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63fdced185c714ba3298268ff711644b35dd20a3"
        },
        "date": 1759684650676,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9771673,
            "range": "± 106506",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140689066,
            "range": "± 2330947",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 820234011,
            "range": "± 14200054",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693481842,
            "range": "± 2206536",
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
          "id": "68ae25457dd32c13ab6cba453636adcec00cfbb5",
          "message": "chore: fix Clippy lints",
          "timestamp": "2025-10-06T00:40:19+02:00",
          "tree_id": "c47e240ca39241c2953b41530612dcb513d62579",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/68ae25457dd32c13ab6cba453636adcec00cfbb5"
        },
        "date": 1759705101283,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10256149,
            "range": "± 201465",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139076635,
            "range": "± 1074085",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 815616296,
            "range": "± 8817192",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 696035269,
            "range": "± 1642076",
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
          "id": "6538e88b72e192107087ba7cec585321edf05d09",
          "message": "chore(renovate): ignore `criterion` for now\n\nThe latest major versions of `criterion` brought quite a few API changes\nthat are not so worthwhile to tackle at the moment, since it's a\ndevelopment dependency only.",
          "timestamp": "2025-10-23T18:34:23+02:00",
          "tree_id": "ae32fa77b646b74dbc94f4ccaec86a4828cfcf80",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6538e88b72e192107087ba7cec585321edf05d09"
        },
        "date": 1761238734906,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9663393,
            "range": "± 58382",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138649854,
            "range": "± 2794484",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 828839927,
            "range": "± 11972142",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693473154,
            "range": "± 3849893",
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
          "id": "d3b0c9ba06c2f99418fbf904720c32a7fe260f8c",
          "message": "chore(deps): update rust dependencies (#361)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-23T16:50:56Z",
          "tree_id": "028ea36f783ab85d7a9e8fcdccb6d80ada8b95b6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3b0c9ba06c2f99418fbf904720c32a7fe260f8c"
        },
        "date": 1761239483501,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10566517,
            "range": "± 230653",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141320669,
            "range": "± 2465288",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 828876136,
            "range": "± 15933848",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 694462796,
            "range": "± 4208530",
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
          "id": "d3b0c9ba06c2f99418fbf904720c32a7fe260f8c",
          "message": "chore(deps): update rust dependencies (#361)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-10-23T16:50:56Z",
          "tree_id": "028ea36f783ab85d7a9e8fcdccb6d80ada8b95b6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3b0c9ba06c2f99418fbf904720c32a7fe260f8c"
        },
        "date": 1761240347227,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 5329230,
            "range": "± 50579",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125092270,
            "range": "± 2389303",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 810166580,
            "range": "± 17690822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 666096456,
            "range": "± 2117279",
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
          "id": "4fc538303baab3d42434dafd51fd20f9661f37ce",
          "message": "chore(deps): update helper python scripts",
          "timestamp": "2025-11-01T01:35:39Z",
          "tree_id": "36e0837ee417ac917a3d94b597047eb1be289b97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4fc538303baab3d42434dafd51fd20f9661f37ce"
        },
        "date": 1761981019863,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9910815,
            "range": "± 186334",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139569146,
            "range": "± 4129113",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 816590531,
            "range": "± 10966178",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695768299,
            "range": "± 2802166",
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
          "id": "245cec63ac9f572a553fffc8e8a113aa53da725b",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-11-01T06:47:02Z",
          "tree_id": "d940a271e34d2186f8d7b4029b31f2f7f00cb5e7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/245cec63ac9f572a553fffc8e8a113aa53da725b"
        },
        "date": 1761990888189,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9834835,
            "range": "± 764918",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139279207,
            "range": "± 4560335",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 830480779,
            "range": "± 11290614",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 698369679,
            "range": "± 3302336",
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
          "id": "e51485016e519f1722d787f78b336e4611ac9d7b",
          "message": "chore(deps): update rust dependencies",
          "timestamp": "2025-11-01T09:37:32Z",
          "tree_id": "86c7629336ee3b7694314c5dc083b01791e79cba",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e51485016e519f1722d787f78b336e4611ac9d7b"
        },
        "date": 1762002199923,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9881473,
            "range": "± 341111",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 137399217,
            "range": "± 2979774",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 814671863,
            "range": "± 12844903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 686522361,
            "range": "± 4552447",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zebulanstanphill@protonmail.com",
            "name": "Zebulan Stanphill",
            "username": "ZebulanStanphill"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0bb2b2161c9471cd4157ee94da1877d8fdca3949",
          "message": "Add support for `function` folder introduced in 24w21a. (#372)",
          "timestamp": "2025-11-09T13:04:57Z",
          "tree_id": "076e6f8fb850d56e9d4b773f18fae62d6ca75998",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0bb2b2161c9471cd4157ee94da1877d8fdca3949"
        },
        "date": 1762694835214,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9785757,
            "range": "± 219185",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140049760,
            "range": "± 3425374",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 818742840,
            "range": "± 11913297",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695894193,
            "range": "± 4123384",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zebulanstanphill@protonmail.com",
            "name": "Zebulan Stanphill",
            "username": "ZebulanStanphill"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0bb2b2161c9471cd4157ee94da1877d8fdca3949",
          "message": "Add support for `function` folder introduced in 24w21a. (#372)",
          "timestamp": "2025-11-09T13:04:57Z",
          "tree_id": "076e6f8fb850d56e9d4b773f18fae62d6ca75998",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0bb2b2161c9471cd4157ee94da1877d8fdca3949"
        },
        "date": 1762695751456,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9589202,
            "range": "± 986048",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139996656,
            "range": "± 2921662",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 818701818,
            "range": "± 10213419",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693830314,
            "range": "± 3387718",
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
          "id": "36ee88e57ae16fadd0d8c1c5972d0919db5ed50d",
          "message": "chore: fix new compilation warning due to stabilization of `lazy_get`",
          "timestamp": "2025-12-27T20:42:47+01:00",
          "tree_id": "4f6b0cd42b9f3ee364ab900fb3bfef35b5973345",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36ee88e57ae16fadd0d8c1c5972d0919db5ed50d"
        },
        "date": 1766865669423,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9544542,
            "range": "± 974935",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144038786,
            "range": "± 4740706",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 784566314,
            "range": "± 12909514",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 589197048,
            "range": "± 3872163",
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
          "id": "eb4ab984897ae9bba946bd6a504374e629857262",
          "message": "fix(ci/wiki-sync): properly set up Git user configuration\n\nIt turns out that using the `GIT_AUTHOR_*` environment variables is not\nenough for Git to have a proper commit setup.",
          "timestamp": "2025-12-27T20:46:09+01:00",
          "tree_id": "2ad064d4f6595a078e6185cd2fa1f0492f3c66b7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eb4ab984897ae9bba946bd6a504374e629857262"
        },
        "date": 1766865941009,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9749449,
            "range": "± 515996",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141888391,
            "range": "± 1407345",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 774212295,
            "range": "± 12238260",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 585600982,
            "range": "± 2997502",
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
          "id": "c80810a23754076252fb354cccaa5d98659ab407",
          "message": "tweak(ci/wiki-sync): prettify wiki commit message format",
          "timestamp": "2025-12-27T20:55:08+01:00",
          "tree_id": "b098d89afad557369c43c31d2b33b15b712b8fb4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c80810a23754076252fb354cccaa5d98659ab407"
        },
        "date": 1766866427852,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9955322,
            "range": "± 182507",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141345075,
            "range": "± 3345258",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 770155175,
            "range": "± 13010005",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 584816177,
            "range": "± 3049543",
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
          "id": "0919de831852dea0a8acb314f3436d4260f65a15",
          "message": "docs(wiki): update ZIP file reuse feature design doc according to latest builds",
          "timestamp": "2025-12-27T21:51:41+01:00",
          "tree_id": "daba10aaf7258bda20def413b85bf012c26306b3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0919de831852dea0a8acb314f3436d4260f65a15"
        },
        "date": 1766869832278,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9627800,
            "range": "± 946151",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141896766,
            "range": "± 2887749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 774258283,
            "range": "± 12689546",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 587425350,
            "range": "± 2471768",
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
          "id": "7e485f6cbf94b5fb4b1c0380feebe023c56c174f",
          "message": "chore(deps): pin dependencies (#376)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-27T21:54:36Z",
          "tree_id": "94360f6fd3a1be6aa19d338c980b9025743bd9ec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7e485f6cbf94b5fb4b1c0380feebe023c56c174f"
        },
        "date": 1766873577457,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10203436,
            "range": "± 1082433",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142582252,
            "range": "± 3176767",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 760882780,
            "range": "± 6153822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 590132811,
            "range": "± 1859954",
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
          "id": "7e485f6cbf94b5fb4b1c0380feebe023c56c174f",
          "message": "chore(deps): pin dependencies (#376)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-27T21:54:36Z",
          "tree_id": "94360f6fd3a1be6aa19d338c980b9025743bd9ec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7e485f6cbf94b5fb4b1c0380feebe023c56c174f"
        },
        "date": 1766874203676,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9471629,
            "range": "± 250751",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142042558,
            "range": "± 3283969",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 769136394,
            "range": "± 16177602",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 584860418,
            "range": "± 2260051",
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
          "id": "d6964d40b8f2f2ffe4d269fb2d0c9a1e5523476d",
          "message": "tweak(ci): improve maintainability of Debian package desc generation code\n\n`html2text` is a tool whose maintenance status is quite uncertain, and\nhas introduced breaking changes in its interface that will bite us as\nsoon as we dare to update the Debian container distribution version.\n\nSo, instead of stitching together sed scripts with obsolete CLI tools,\nlet's just converge on using the `markdown-it` Python port from a small\nPython script, which should be much more robust and flexible moving\nforward, as the Markdown is now parsed and rendered directly to plain\ntext by a proper and well-maintained library to the effect. Providing\nPython 3 is already available on the environment, this even slightly\nreduces the total size of Debian dependency packages needed for the\nscript to work.",
          "timestamp": "2025-12-31T18:39:35+01:00",
          "tree_id": "408345856981e4eaccffdc84c8344ea73eb0fd5a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d6964d40b8f2f2ffe4d269fb2d0c9a1e5523476d"
        },
        "date": 1767204134702,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9641538,
            "range": "± 83880",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 145473531,
            "range": "± 3806950",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 770625930,
            "range": "± 10232668",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 588241030,
            "range": "± 1604592",
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
          "id": "fc9b394538ee3a3d8e819769ce1042a63e8b75f6",
          "message": "tweak(ci/wiki-sync): do not create wiki commits when nothing changes",
          "timestamp": "2025-12-31T18:56:21+01:00",
          "tree_id": "569965a6b16a2f273eea165e09d521d65279083d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fc9b394538ee3a3d8e819769ce1042a63e8b75f6"
        },
        "date": 1767204848566,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9540906,
            "range": "± 60475",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 145085589,
            "range": "± 4044282",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 768221142,
            "range": "± 11655013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 588211642,
            "range": "± 1958549",
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
          "id": "eb91e912c5a9c6923b3628b0aa4cf514572d4a09",
          "message": "docs(wiki): update for PackSquash v0.4.1",
          "timestamp": "2025-12-31T18:59:57+01:00",
          "tree_id": "4d81580e912f5b33a37ef10815464172c7800234",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eb91e912c5a9c6923b3628b0aa4cf514572d4a09"
        },
        "date": 1767205089802,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9715867,
            "range": "± 967106",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144491064,
            "range": "± 5593640",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 789760343,
            "range": "± 10959541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 594041758,
            "range": "± 2942197",
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
          "distinct": false,
          "id": "e9fcb3390a4ec242f8ddb3e11ad33d5fedf922b5",
          "message": "docs: update new release checklist, fix typo in code comment",
          "timestamp": "2025-12-31T18:58:20+01:00",
          "tree_id": "5ad66dd9af91392d30c3555216407fe0bf58ba58",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e9fcb3390a4ec242f8ddb3e11ad33d5fedf922b5"
        },
        "date": 1767205495750,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9935376,
            "range": "± 669487",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142620572,
            "range": "± 4726419",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 790333010,
            "range": "± 13203834",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 591466822,
            "range": "± 3010810",
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
          "id": "e58924ee6ce10e47e04d1913ebba0c7e4feee128",
          "message": "chore(deps): update helper python scripts",
          "timestamp": "2025-12-31T18:26:58Z",
          "tree_id": "19be364c34cff02fa0ebf5a46af3fa699dfd5be0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e58924ee6ce10e47e04d1913ebba0c7e4feee128"
        },
        "date": 1767209643002,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10003400,
            "range": "± 900486",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143289524,
            "range": "± 5334893",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 785851215,
            "range": "± 10534439",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 592192642,
            "range": "± 2336438",
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
          "id": "fe122ed6f63358f0bb7ba7847e29f167ee35e878",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-12-31T19:16:00Z",
          "tree_id": "e8c4cce075551d4e1ec420bf9c85e0e7fdacc78f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fe122ed6f63358f0bb7ba7847e29f167ee35e878"
        },
        "date": 1767211045637,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9767757,
            "range": "± 297273",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142163031,
            "range": "± 2674428",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 790641471,
            "range": "± 11262328",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 590984284,
            "range": "± 2490967",
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
          "id": "b8aa0c781a70e6e438201dc4b8c8d7f8446ecc2d",
          "message": "chore(deps): update dependency certifi to v2025",
          "timestamp": "2025-12-31T19:39:33Z",
          "tree_id": "9068297d0cae92ed99009ce8aedd318b55bd0b68",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b8aa0c781a70e6e438201dc4b8c8d7f8446ecc2d"
        },
        "date": 1767217352969,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9698975,
            "range": "± 198889",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142859223,
            "range": "± 5441516",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 783098363,
            "range": "± 12114183",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 592192247,
            "range": "± 2569597",
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
            "email": "7822554+AlexTMjugador@users.noreply.github.com",
            "name": "Alejandro González",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "2754fbe12bfbf01da9687a77762ca058464f21af",
          "message": "docs(wiki): update for PackSquash v0.4.1",
          "timestamp": "2026-01-01T19:40:15+01:00",
          "tree_id": "4fdc3b998fdc97678acb22af51f27949b6831a72",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2754fbe12bfbf01da9687a77762ca058464f21af"
        },
        "date": 1767294166501,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10292564,
            "range": "± 265696",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142655459,
            "range": "± 5815447",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 770113474,
            "range": "± 11853257",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 591633019,
            "range": "± 3375424",
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
          "id": "12fb4061b9c229d23aa98b591a47f3b5ce59fdaa",
          "message": "docs(CHANGELOG): fix typo",
          "timestamp": "2026-01-01T20:12:32+01:00",
          "tree_id": "cbd775994fcd12de2d5f7c2f73e63c4a3f0bf84b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12fb4061b9c229d23aa98b591a47f3b5ce59fdaa"
        },
        "date": 1767297160543,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10151010,
            "range": "± 447676",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142504865,
            "range": "± 1941297",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 768242559,
            "range": "± 8421114",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 590277379,
            "range": "± 2525104",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "me@alegon.dev"
          },
          "committer": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "me@alegon.dev"
          },
          "id": "12fb4061b9c229d23aa98b591a47f3b5ce59fdaa",
          "message": "docs(CHANGELOG): fix typo",
          "timestamp": "2026-01-01T19:12:32Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12fb4061b9c229d23aa98b591a47f3b5ce59fdaa"
        },
        "date": 1767297196557,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10053656,
            "range": "± 1147708",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 142507211,
            "range": "± 2976783",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 773950108,
            "range": "± 15842147",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 591358023,
            "range": "± 2293513",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "me@alegon.dev"
          },
          "committer": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "me@alegon.dev"
          },
          "id": "7fcd681e89e61f6914e7a0eb3baea15ef22ed4e1",
          "message": "ci: do not get target APT arch for non-Linux targets",
          "timestamp": "2026-01-01T21:44:50Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7fcd681e89e61f6914e7a0eb3baea15ef22ed4e1"
        },
        "date": 1767305023075,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9636928,
            "range": "± 455757",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143379840,
            "range": "± 4016063",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 778172236,
            "range": "± 12731423",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 592833171,
            "range": "± 3601481",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}