window.BENCHMARK_DATA = {
  "lastUpdate": 1682478166480,
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
          "id": "ba9902675ac1faa36f47df3e9a734b682d389229",
          "message": "Revert \"ci: speed up Cargo registry fetch\"\n\nThis reverts commit 7c1c4903598cc4969efd8c31936d9e4f461dec4e.\n\nactions-rs/toolchain already handles this.",
          "timestamp": "2023-03-15T18:14:48Z",
          "tree_id": "515fec89d7934a6fee116f2d46c600f22cf0880b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ba9902675ac1faa36f47df3e9a734b682d389229"
        },
        "date": 1678907211673,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13130290,
            "range": "± 429335",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 460088066,
            "range": "± 17578960",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4735074794,
            "range": "± 86148871",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 922388936,
            "range": "± 14930790",
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
          "id": "526fd5add778b940ef7235344cc1ec053ea9a593",
          "message": "chore(deps): update dependency filelock to v3.10.0",
          "timestamp": "2023-03-15T23:06:11Z",
          "tree_id": "54792303efd15ad1f054234f0968761f3169c9d2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/526fd5add778b940ef7235344cc1ec053ea9a593"
        },
        "date": 1678932733909,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9660519,
            "range": "± 88971",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 367444051,
            "range": "± 2194280",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4717443335,
            "range": "± 14674622",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 800170211,
            "range": "± 5901725",
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
          "id": "14fa4ea4200b9a83b1a384807cbfb3dcfc0b6c0f",
          "message": "fix(deps): update rust crate walkdir to 2.3.3",
          "timestamp": "2023-03-16T14:14:00Z",
          "tree_id": "c1da65ef4bb27d5abcdc66370cc57aec3ddb15b1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/14fa4ea4200b9a83b1a384807cbfb3dcfc0b6c0f"
        },
        "date": 1678989533160,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10335999,
            "range": "± 124845",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 333371750,
            "range": "± 2083408",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5032262171,
            "range": "± 58357749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 648689002,
            "range": "± 10541979",
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
          "id": "af0cbf2a5be4ce831aae9ac2f6de6cd0242d2a55",
          "message": "fix(deps): update rust crate sysinfo to 0.28.3",
          "timestamp": "2023-03-17T21:49:24Z",
          "tree_id": "f089aa02d0cac279d5afc4005b7650cd42eb067a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/af0cbf2a5be4ce831aae9ac2f6de6cd0242d2a55"
        },
        "date": 1679103176780,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10803254,
            "range": "± 273817",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 338900390,
            "range": "± 7707999",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5061573479,
            "range": "± 65388129",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 651979634,
            "range": "± 5595461",
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
          "id": "e8b3d3478572d29a5e970cc0d75d0f8dcd223608",
          "message": "fix(deps): update rust crate serde to 1.0.157",
          "timestamp": "2023-03-18T01:02:29Z",
          "tree_id": "1e7da161d394deccdffecd493b3978ae67ca7ee1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e8b3d3478572d29a5e970cc0d75d0f8dcd223608"
        },
        "date": 1679109526345,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9907533,
            "range": "± 102833",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 304431010,
            "range": "± 6784379",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4622904167,
            "range": "± 88091152",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 637202582,
            "range": "± 2897824",
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
          "id": "6b35bde4464ae5ee522479eaa8ecedf0fadb7332",
          "message": "fix(deps): update rust crate thiserror to 1.0.40",
          "timestamp": "2023-03-18T02:47:03Z",
          "tree_id": "e4e7af075ebd03c0f2d0c16e18f60b7d8a43b5a4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6b35bde4464ae5ee522479eaa8ecedf0fadb7332"
        },
        "date": 1679115889086,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12065881,
            "range": "± 777158",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 376133180,
            "range": "± 8396472",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5194951999,
            "range": "± 94632717",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 728657031,
            "range": "± 15819016",
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
          "id": "ba21c7821c50f012fb5e4fd627f13bc7876b5612",
          "message": "fix(deps): update rust crate wmi to 0.12.1",
          "timestamp": "2023-03-18T11:47:58Z",
          "tree_id": "0b506f549007ed4752b6de0322782529d5b3f63f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ba21c7821c50f012fb5e4fd627f13bc7876b5612"
        },
        "date": 1679148140174,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9928716,
            "range": "± 132917",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 300548548,
            "range": "± 14418023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4650637817,
            "range": "± 70786693",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 643568482,
            "range": "± 6626984",
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
          "id": "6374fb4ea14081824b7f63adbb991beed4dc5f82",
          "message": "fix(deps): update rust crate is-terminal to 0.4.5",
          "timestamp": "2023-03-18T16:40:22Z",
          "tree_id": "8801861b397f606481e8d4198505e77c837c1e1b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6374fb4ea14081824b7f63adbb991beed4dc5f82"
        },
        "date": 1679166441845,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9965329,
            "range": "± 112025",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 299663931,
            "range": "± 7752502",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4653953162,
            "range": "± 62023710",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 644946269,
            "range": "± 5173951",
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
          "id": "9a322b2e8e59b13806f7417f0bc7270634356691",
          "message": "fix(deps): update rust crate serde to 1.0.158",
          "timestamp": "2023-03-20T13:21:56Z",
          "tree_id": "501d6e4b3095aff6d5089c9fed5e67b45b9c36be",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9a322b2e8e59b13806f7417f0bc7270634356691"
        },
        "date": 1679331945670,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13602121,
            "range": "± 775172",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 405273892,
            "range": "± 14259756",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5276328192,
            "range": "± 103788089",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 765583597,
            "range": "± 28557870",
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
          "id": "202db6bcdb8f315f00600ff0ff33e7a7567a21b6",
          "message": "chore(deps): update dependency beautifulsoup4 to v4.12.0",
          "timestamp": "2023-03-20T16:33:00Z",
          "tree_id": "7eaf2a9bb54574abe162866ecb59b963bfc4b82f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/202db6bcdb8f315f00600ff0ff33e7a7567a21b6"
        },
        "date": 1679349571833,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9610407,
            "range": "± 191067",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 306091345,
            "range": "± 6846724",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4661423118,
            "range": "± 30288419",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 636572816,
            "range": "± 5055333",
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
          "id": "7fce49a13dc609ce95de1d59968b6c4fccae3beb",
          "message": "fix(deps): update rust crate regex to 1.7.2",
          "timestamp": "2023-03-21T15:26:40Z",
          "tree_id": "0163177fa0bd33362a9a9245d3b5407446b70774",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7fce49a13dc609ce95de1d59968b6c4fccae3beb"
        },
        "date": 1679427670247,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12220075,
            "range": "± 839451",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 330071220,
            "range": "± 18520351",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4321243850,
            "range": "± 101260978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 640806700,
            "range": "± 16413125",
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
          "id": "d6f1fdae59994a3a18d1cc9060967c196c89482f",
          "message": "fix(deps): update rust crate serde_path_to_error to 0.1.11",
          "timestamp": "2023-03-22T07:29:58Z",
          "tree_id": "c5615387505cc62d7455852cb5907ff984ac76f2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d6f1fdae59994a3a18d1cc9060967c196c89482f"
        },
        "date": 1679478842293,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12783873,
            "range": "± 986146",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 412543541,
            "range": "± 9231928",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 6230352088,
            "range": "± 126005493",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 815715990,
            "range": "± 12788029",
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
          "id": "1d318d0909b080d3596ea0067ff49d7bcc46d853",
          "message": "chore(deps): update dependency filelock to v3.10.1",
          "timestamp": "2023-03-22T17:53:39Z",
          "tree_id": "63d43d3dbf2b1df17c4b0bc91d2d02900d5ff000",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1d318d0909b080d3596ea0067ff49d7bcc46d853"
        },
        "date": 1679525976374,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13191454,
            "range": "± 639685",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 390235883,
            "range": "± 12180787",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5318264741,
            "range": "± 113456636",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 743313242,
            "range": "± 13110834",
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
          "id": "96f2ac2814e16df540606e3c26cf275eb990df2c",
          "message": "chore(deps): update dependency filelock to v3.10.2",
          "timestamp": "2023-03-23T02:51:14Z",
          "tree_id": "1a3db302829421aac1552fe567cfd8dfa6c835e5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/96f2ac2814e16df540606e3c26cf275eb990df2c"
        },
        "date": 1679556147137,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9925217,
            "range": "± 172141",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 298643247,
            "range": "± 13828133",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4637505995,
            "range": "± 71097749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 638837671,
            "range": "± 8989861",
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
          "id": "05163522bb5ae86158cef97fc04943a7d781ddb4",
          "message": "chore(deps): update dependency filelock to v3.10.3",
          "timestamp": "2023-03-24T00:22:52Z",
          "tree_id": "af9d54bf7d064487118bb1b6c86e4a2956325436",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/05163522bb5ae86158cef97fc04943a7d781ddb4"
        },
        "date": 1679637176911,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14401046,
            "range": "± 917770",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 392200040,
            "range": "± 12062779",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5501750758,
            "range": "± 149888002",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 786229778,
            "range": "± 14758931",
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
          "id": "3f815744232471b6517ccc1ba6e79be75518f340",
          "message": "chore(deps): update helper python scripts",
          "timestamp": "2023-03-24T15:26:47Z",
          "tree_id": "c38d68480e95b62d7e67e9230d455a0504a99339",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3f815744232471b6517ccc1ba6e79be75518f340"
        },
        "date": 1679690154229,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12396344,
            "range": "± 369846",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 404098758,
            "range": "± 3019128",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 6371717983,
            "range": "± 69380244",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 776576858,
            "range": "± 18453103",
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
          "id": "fba324393233aad2f2fab698a3e6371e08754609",
          "message": "fix(deps): update rust crate sysinfo to 0.28.4",
          "timestamp": "2023-03-24T20:10:51Z",
          "tree_id": "a43b8522382e6174038c0c62652c396cd61a747b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fba324393233aad2f2fab698a3e6371e08754609"
        },
        "date": 1679707369781,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10686498,
            "range": "± 10122316",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 343148535,
            "range": "± 3143383",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5312131116,
            "range": "± 21037158",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 664315982,
            "range": "± 11455834",
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
          "id": "2d17cfa25c7086c2e6fa207178ee5207720c2090",
          "message": "fix(deps): update rust crate indexmap to 1.9.3",
          "timestamp": "2023-03-25T00:53:18Z",
          "tree_id": "bb8f62a3d86bfd50554a494003136641b0901309",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2d17cfa25c7086c2e6fa207178ee5207720c2090"
        },
        "date": 1679726764179,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10544751,
            "range": "± 135307",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 342791773,
            "range": "± 6240195",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5314681587,
            "range": "± 48371348",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 661393924,
            "range": "± 5987198",
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
          "id": "44899793c535422ce5d7bf75ad2d7626b77fbc3c",
          "message": "fix(deps): update rust crate regex to 1.7.3",
          "timestamp": "2023-03-25T06:20:03Z",
          "tree_id": "53d7dae9d8dc012aa9368cb1684ee8924c44eb3a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/44899793c535422ce5d7bf75ad2d7626b77fbc3c"
        },
        "date": 1679746170120,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10671032,
            "range": "± 280075",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 342276514,
            "range": "± 2533064",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5319969973,
            "range": "± 48977460",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 662727399,
            "range": "± 8334775",
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
          "id": "685daddd899619a5d836a91dfb10f0f3403abe57",
          "message": "chore(deps): update dependency gdown to v4.7.1",
          "timestamp": "2023-03-25T11:41:42Z",
          "tree_id": "17ad81604f745419cddf47ee8aac8c7512752216",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/685daddd899619a5d836a91dfb10f0f3403abe57"
        },
        "date": 1679757493237,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10556875,
            "range": "± 171707",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 342038711,
            "range": "± 18840952",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5325061206,
            "range": "± 27700491",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 660745319,
            "range": "± 5804113",
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
          "id": "ce990ea7297fa285de6d105849af6f878f4c7a4e",
          "message": "chore(deps): update dependency filelock to v3.10.6",
          "timestamp": "2023-03-26T04:01:22Z",
          "tree_id": "312f3380907bcad5b43270d25b795bd7b9287166",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ce990ea7297fa285de6d105849af6f878f4c7a4e"
        },
        "date": 1679817184767,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9649889,
            "range": "± 181959",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 292841095,
            "range": "± 10700881",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4328536330,
            "range": "± 34994642",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 630483842,
            "range": "± 5720692",
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
          "id": "331b41b5b6111b4a511b3afa53daf32a75a15467",
          "message": "fix(deps): update rust crate serde_json to 1.0.95",
          "timestamp": "2023-03-28T04:24:53Z",
          "tree_id": "6fa125f4c76725be0192829aff861b99db7f832e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/331b41b5b6111b4a511b3afa53daf32a75a15467"
        },
        "date": 1680006544112,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12070140,
            "range": "± 313689",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 384966255,
            "range": "± 4341946",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 6002983354,
            "range": "± 40566473",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 749492178,
            "range": "± 10407281",
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
          "id": "33ab449fe3d259b2621560d2fad011863deca677",
          "message": "fix(deps): update rust crate serde to 1.0.159",
          "timestamp": "2023-03-28T11:37:10Z",
          "tree_id": "5ff6bc462ebc158bd4afd83a0e2d942235b417a1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/33ab449fe3d259b2621560d2fad011863deca677"
        },
        "date": 1680028451938,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10338675,
            "range": "± 126297",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 321852789,
            "range": "± 1809738",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4654272154,
            "range": "± 56666453",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 640185476,
            "range": "± 12889470",
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
          "id": "ad0f1b10013756091f57eb96128a827eb18db35b",
          "message": "chore(ci): simplify non-fork PR exclusion condition",
          "timestamp": "2023-03-28T22:13:06+02:00",
          "tree_id": "171b035fd333b8e90dbb4a6369172eadff1372ec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ad0f1b10013756091f57eb96128a827eb18db35b"
        },
        "date": 1680036165629,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13081662,
            "range": "± 1327584",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 353440384,
            "range": "± 9025234",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4573776503,
            "range": "± 122280898",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 707262775,
            "range": "± 21294017",
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
          "id": "b32125ef95d55d6530e2de2ccfe79734cd87e016",
          "message": "fix(deps): update rust crate tokio to 1.27.0",
          "timestamp": "2023-03-28T23:21:04Z",
          "tree_id": "b6c0e3e589b11ffd3310ca48d043adfcc7cf58ee",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b32125ef95d55d6530e2de2ccfe79734cd87e016"
        },
        "date": 1680075951395,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9413327,
            "range": "± 48433",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 282764932,
            "range": "± 8115436",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4275918125,
            "range": "± 55094143",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 617808745,
            "range": "± 6624707",
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
          "id": "4d8a2278c745603d4efefa62e5e3761bdcf58dcb",
          "message": "fix(deps): update rust crate tempfile to 3.5.0 (#199)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-03-30T19:00:22+02:00",
          "tree_id": "86372234e698ace3c6d50072d5a40eb6ccf4ec5a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4d8a2278c745603d4efefa62e5e3761bdcf58dcb"
        },
        "date": 1680197798147,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11929379,
            "range": "± 449663",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 379903069,
            "range": "± 3249937",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5591761993,
            "range": "± 50984554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 758069558,
            "range": "± 10646300",
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
          "id": "33cb4b6f6b0794abad8be18a5effec0bf0dfcabe",
          "message": "chore: update dependencies\n\nThis should fix several Renovate failures.",
          "timestamp": "2023-04-03T13:52:28+02:00",
          "tree_id": "85aa2fcfcf12f3f26bda96c37631c1aa703b0bb1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/33cb4b6f6b0794abad8be18a5effec0bf0dfcabe"
        },
        "date": 1680525654828,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10513010,
            "range": "± 132018",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 334239863,
            "range": "± 15407195",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4736411500,
            "range": "± 89949387",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 649484541,
            "range": "± 7301614",
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
          "id": "bb8b6aeccc20b4aa65aa3ff765f6c5c8e98a8960",
          "message": "fix(deps): update rust crate winreg to 0.50.0",
          "timestamp": "2023-04-04T16:32:31Z",
          "tree_id": "b2a8d6ef2dfd1382f896098e4e9ed0cf1111840f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bb8b6aeccc20b4aa65aa3ff765f6c5c8e98a8960"
        },
        "date": 1680627914716,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12045716,
            "range": "± 13649786",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 385153201,
            "range": "± 9001700",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5955574086,
            "range": "± 69651414",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 742520102,
            "range": "± 8878185",
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
          "id": "a237d1859d5b4ad40907cd27f455db3abfc55955",
          "message": "chore(deps): update dependency beautifulsoup4 to v4.12.1",
          "timestamp": "2023-04-05T17:01:58Z",
          "tree_id": "1b59a166ce1ba7e1c0608fdc12faee1c2b24cb4d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a237d1859d5b4ad40907cd27f455db3abfc55955"
        },
        "date": 1680731992352,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12789635,
            "range": "± 13582696",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 403055480,
            "range": "± 8400742",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 6149862211,
            "range": "± 153255317",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 775632950,
            "range": "± 17842076",
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
          "id": "b12c2f341a13ffe3cc83b4e850136e553bd09862",
          "message": "feat: overhaul shader processing code\n\nAs described in issue #187, PackSquash does not support shaders that use\npreprocessor directives in external declaration position.\n\nTo properly address this issue, it is necessary to equip PackSquash with\na GLSL preprocessor that runs before parsing the source GLSL, as the\nGLSL specification mandates for proper GLSL compilation. (Building a\nGLSL parser that accepts preprocessor directives anywhere, even in the\nmiddle of tokens, is, if not impossible, an exceedingly complex task.)\n\nThis commit improves on the situation by adding such preprocessing\ncapabilities to PackSquash, so that the preprocessing step accepts\ndirectives anywhere that are not seen by the downstream parser. However,\nif they appear in external declaration position, it is possible to\ninject them back into the AST, in order to preserve them in the\ntransformed GLSL source. Only directives that are needed by Minecraft\nare injected; others are resolved and removed by PackSquash.\n\nIn addition, include shaders now need not be full translation units:\nthey might be standalone statements or expressions, too. Functionality\nto prettify shaders was also added, revamping the `minify_shader` option\nto a more powerful `shader_source_transformation_strategy`.\n\nThese changes allow PackSquash to deal properly with a wider variety of\nshaders. The only ones that still trigger failure modes are:\n\n- Shaders which depend on `#moj_import`ed preprocessor variables to be\n  syntactically correct.\n- Shaders which depend on `#moj_import`ed preprocessor variables to\n  build the expected source code, but are syntactically correct\n  otherwise. (This can be worked around by setting\n  `source_transformation_strategy = 'keep_as_is'` for the affected\n  shaders.)\n- Shaders that include vertex or fragment shaders, relying on their\n  preprocessor variables or syntax to be valid. (This can be partially\n  worked around by setting `is_top_level_shader = false` for the\n  included shaders, although including vertex or fragment shaders is an\n  extreme edge case that we never saw before, and arguably a bad idea.)\n\nThe first two kinds of shaders should be able to be properly parsed for\nPackSquash v0.4.0, because its design allows pack files to resolve\nreferences to other files.",
          "timestamp": "2023-04-06T17:54:11+02:00",
          "tree_id": "cfc3949cf147af2843b1e5d5c9e15e6d0a7250ab",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b12c2f341a13ffe3cc83b4e850136e553bd09862"
        },
        "date": 1680800031885,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12310093,
            "range": "± 11549413",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 398754998,
            "range": "± 3376013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5617587527,
            "range": "± 45455593",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 766546399,
            "range": "± 8595821",
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
          "id": "807d8fa5c9e74880201048ab38d2883e2e63cd9d",
          "message": "fix(deps): update rust crate is-terminal to 0.4.7",
          "timestamp": "2023-04-06T17:28:54Z",
          "tree_id": "c397522d8c5e46468446e5dc42ddc84c4dfd78c4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/807d8fa5c9e74880201048ab38d2883e2e63cd9d"
        },
        "date": 1680819310441,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10254514,
            "range": "± 144745",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 325166283,
            "range": "± 7280024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4649795903,
            "range": "± 35215618",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 635164249,
            "range": "± 6392152",
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
          "id": "89b9dff884d33472989cad64bf1309c6c7ee31f0",
          "message": "chore(deps): update dependency filelock to v3.11.0",
          "timestamp": "2023-04-06T21:43:13Z",
          "tree_id": "15315fd679427be9a685b5c01814712f4186e96e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/89b9dff884d33472989cad64bf1309c6c7ee31f0"
        },
        "date": 1680833343067,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9452406,
            "range": "± 13683869",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 287357797,
            "range": "± 3173682",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4297310163,
            "range": "± 28739037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 618833477,
            "range": "± 6650683",
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
          "id": "ca4dae092ad8294044c3c49e86d7ae112ce108ae",
          "message": "chore(deps): update dependency beautifulsoup4 to v4.12.2",
          "timestamp": "2023-04-07T18:15:26Z",
          "tree_id": "0d27a75469d877106b8145bad96f4843831203a6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ca4dae092ad8294044c3c49e86d7ae112ce108ae"
        },
        "date": 1680910395577,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13200465,
            "range": "± 11591176",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 382516771,
            "range": "± 6682846",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4975124429,
            "range": "± 44874585",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 729724099,
            "range": "± 9594511",
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
          "id": "bd012bb78ea6b9ff1dddda5f503d6e1821eb55ff",
          "message": "fix(deps): update rust crate uuid to 1.3.1",
          "timestamp": "2023-04-09T04:47:23Z",
          "tree_id": "073d499984f0f57996a6ad8228a3bd8894ab5320",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bd012bb78ea6b9ff1dddda5f503d6e1821eb55ff"
        },
        "date": 1681030802801,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13316073,
            "range": "± 9988572",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 377356177,
            "range": "± 5017502",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4980494556,
            "range": "± 51234519",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 730098276,
            "range": "± 9785451",
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
          "id": "8098ce4cb8ed1cb062717562e2715bf6a6259ac6",
          "message": "chore(deps): do not pin to yanked crossbeam-channel version",
          "timestamp": "2023-04-12T01:04:22+02:00",
          "tree_id": "4f945262c7783ab89ea334e738949bf092e4348a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8098ce4cb8ed1cb062717562e2715bf6a6259ac6"
        },
        "date": 1681256406267,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9433314,
            "range": "± 9473361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 284962929,
            "range": "± 3405860",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4308549370,
            "range": "± 31982528",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 623614411,
            "range": "± 11079762",
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
          "id": "1b1904776c68cc6ccf5f8e9a44742cc19b9e9d89",
          "message": "fix(deps): update rust crate png to 0.17.8",
          "timestamp": "2023-04-12T06:52:20Z",
          "tree_id": "d69bc856938fe8b462d1a3048c3547007abc0544",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1b1904776c68cc6ccf5f8e9a44742cc19b9e9d89"
        },
        "date": 1681285645332,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10575794,
            "range": "± 11631711",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 327942140,
            "range": "± 2802756",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 4709259329,
            "range": "± 40367437",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 645004160,
            "range": "± 7947179",
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
          "id": "03121c94f69c3f45261ce92cf5167f802b514ded",
          "message": "fix(deps): update rust crate serde to 1.0.160",
          "timestamp": "2023-04-12T10:24:36Z",
          "tree_id": "b98492deb67ba4a4b6b0c12e399a349159923ff3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/03121c94f69c3f45261ce92cf5167f802b514ded"
        },
        "date": 1681298310240,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12710233,
            "range": "± 659667",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 397623321,
            "range": "± 8082797",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 5711574379,
            "range": "± 182231030",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 787563898,
            "range": "± 24761496",
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
      }
    ]
  }
}