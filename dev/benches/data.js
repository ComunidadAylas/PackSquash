window.BENCHMARK_DATA = {
  "lastUpdate": 1712203459177,
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
          "id": "c1c99c4e98fd475fd3431f01a159b3f4c7f91a9d",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-01-10T20:38:48Z",
          "tree_id": "62b0cdfa8799931f4ff4d116f456ad7ed040ed99",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c1c99c4e98fd475fd3431f01a159b3f4c7f91a9d"
        },
        "date": 1704929805118,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10637199,
            "range": "± 1868976",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141328383,
            "range": "± 2046547",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2662148687,
            "range": "± 53645661",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 735618703,
            "range": "± 3196225",
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
          "id": "69c5eb1784f8918bf5a367c24e1e8c762be2533c",
          "message": "fix(ci): update `gdown` to make benchmarks work again\n\nGoogle Drive started being more strict on what user agent strings are\nallowed to use it, and `gdown` was just updated to reflect that.",
          "timestamp": "2024-01-16T20:58:36+01:00",
          "tree_id": "8379106418002f34f6449e9854ee1c365c89f95c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/69c5eb1784f8918bf5a367c24e1e8c762be2533c"
        },
        "date": 1705436583626,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10096872,
            "range": "± 1085970",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138809425,
            "range": "± 1965010",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2661175562,
            "range": "± 32347747",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 732650413,
            "range": "± 5154041",
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
          "id": "3cd24a38a9189b464a044a83bcc2fab1c08ceb56",
          "message": "ci: temporarily ignore `cargo deny` action run failures\n\nThis is a temporary workaround until\nhttps://github.com/EmbarkStudios/krates/issues/72 is solved.",
          "timestamp": "2024-01-22T15:15:52+01:00",
          "tree_id": "0af6e3c3064286b611fd39ca521064bb469961a2",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3cd24a38a9189b464a044a83bcc2fab1c08ceb56"
        },
        "date": 1705934375194,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10070538,
            "range": "± 2963890",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144509172,
            "range": "± 1899720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2734022851,
            "range": "± 76485160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 730296754,
            "range": "± 3049321",
            "unit": "ns/iter"
          }
        ]
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
          "id": "48ad4fe6b3c7457989b40ddd8a561bc188448502",
          "message": "chore(deps): update taiki-e/install-action digest to b09bcf0",
          "timestamp": "2024-01-22T16:37:46Z",
          "tree_id": "5554db2a8d19fb07ced9e36f84f78b20ef5632c8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/48ad4fe6b3c7457989b40ddd8a561bc188448502"
        },
        "date": 1705942780804,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10084854,
            "range": "± 4417538",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 143657476,
            "range": "± 1499519",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2687614955,
            "range": "± 77947174",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 729978806,
            "range": "± 2970785",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "3daf527770a27c0420cc85ad03dc477b914dead2",
          "message": "chore(deps): update embarkstudios/cargo-deny-action digest to 1350841",
          "timestamp": "2024-01-22T19:22:36Z",
          "tree_id": "b71605a74536d1caf654659bd115a9588a0e6029",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3daf527770a27c0420cc85ad03dc477b914dead2"
        },
        "date": 1705961717079,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10169013,
            "range": "± 77946",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 144247067,
            "range": "± 1796560",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2731712043,
            "range": "± 95153791",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 730494233,
            "range": "± 2156830",
            "unit": "ns/iter"
          }
        ]
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
          "id": "0d735bb9ee19645950291a4104922cb4287819b4",
          "message": "fix(deps): update rust crate imagequant to 4.3.0",
          "timestamp": "2024-01-23T06:03:34Z",
          "tree_id": "3e6471571643d7871e727320ceb05f6d757ab785",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0d735bb9ee19645950291a4104922cb4287819b4"
        },
        "date": 1705991234572,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10191002,
            "range": "± 323402",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 146368688,
            "range": "± 2378061",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2684387278,
            "range": "± 64512033",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 739651325,
            "range": "± 3024353",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "718307948b1480dba1810a8cf6a258aac8f3b7f5",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-01-25T03:23:59Z",
          "tree_id": "40798794e73618c69474c16d8449fbd289c76170",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/718307948b1480dba1810a8cf6a258aac8f3b7f5"
        },
        "date": 1706169903658,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10142176,
            "range": "± 215888",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 146147884,
            "range": "± 2817822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2735612396,
            "range": "± 96722960",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 735109640,
            "range": "± 3476015",
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
          "id": "97a9e5a70d6be0f4d1f65a20a1eafcaea826531b",
          "message": "Revert \"ci: temporarily ignore `cargo deny` action run failures\"\n\nThis reverts commit 3cd24a38a9189b464a044a83bcc2fab1c08ceb56.",
          "timestamp": "2024-01-25T18:42:53+01:00",
          "tree_id": "6eb0a6bea5c5aa64941e88fa66caf71be4ba5e7f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/97a9e5a70d6be0f4d1f65a20a1eafcaea826531b"
        },
        "date": 1706208156048,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10207387,
            "range": "± 90034",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127421900,
            "range": "± 2053118",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3020841176,
            "range": "± 29461724",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 737353907,
            "range": "± 2647039",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "1717b3e68dd5f3ed5461ecaef76ae2543f01e731",
          "message": "chore(deps): update taiki-e/install-action digest to 1f501f0",
          "timestamp": "2024-01-26T02:01:35Z",
          "tree_id": "69e00554c12f3cf191c4e97ca9d741ed3d7ad464",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1717b3e68dd5f3ed5461ecaef76ae2543f01e731"
        },
        "date": 1706241001572,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9996423,
            "range": "± 76019",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121743921,
            "range": "± 8764907",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2897696145,
            "range": "± 25700868",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 717276810,
            "range": "± 10923011",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "348abc13ea6baff5d6136c35643fa4d31b840ab6",
          "message": "chore(deps): update taiki-e/install-action digest to bee85d7",
          "timestamp": "2024-01-28T02:04:12Z",
          "tree_id": "58c817092e143ecd22edae7785667daa25ebdd97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/348abc13ea6baff5d6136c35643fa4d31b840ab6"
        },
        "date": 1706417133794,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10149909,
            "range": "± 180459",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125535365,
            "range": "± 3357806",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3013720274,
            "range": "± 31696522",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 735802116,
            "range": "± 3230090",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "b0aac587868b83fb82818ddca8f64632d6263aac",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-01-28T04:22:32Z",
          "tree_id": "635412715c52c3cfa977b59b2f4029ba77c5c0e8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b0aac587868b83fb82818ddca8f64632d6263aac"
        },
        "date": 1706433124652,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10127772,
            "range": "± 3804974",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120690706,
            "range": "± 2606695",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2975133006,
            "range": "± 49668372",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 713677196,
            "range": "± 10558731",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "e6199a5ad0fa0296a5b67e7854449f173a22b847",
          "message": "fix(deps): update rust crate indexmap to 2.2.0",
          "timestamp": "2024-01-28T17:42:25Z",
          "tree_id": "27eea4871f44888ce198773adb58001f3ef9c8c6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e6199a5ad0fa0296a5b67e7854449f173a22b847"
        },
        "date": 1706471222571,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10244590,
            "range": "± 947024",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129558654,
            "range": "± 4091272",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3018007497,
            "range": "± 31201705",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 737991406,
            "range": "± 2074710",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "3e527808fcc9f47572f8a853e6adf95c74937a53",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-01-31T00:57:53Z",
          "tree_id": "144c9fd586591bf2612f17cbf8c92c1c1af3d134",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3e527808fcc9f47572f8a853e6adf95c74937a53"
        },
        "date": 1706676696860,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10215917,
            "range": "± 2591078",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 123299583,
            "range": "± 1985071",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2902235967,
            "range": "± 19599242",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 739189771,
            "range": "± 1968590",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "f48992b43c3dae93954d33fad223eb1ce17259c1",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-01-31T15:57:19Z",
          "tree_id": "a8a8592b24fcbd6a45dee44333d8c95e4a3bf861",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f48992b43c3dae93954d33fad223eb1ce17259c1"
        },
        "date": 1706726909669,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10204269,
            "range": "± 51965",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129971905,
            "range": "± 3007342",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2659696121,
            "range": "± 93022317",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 827684544,
            "range": "± 3401220",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "9e7ca2f268d8f5c76e964ad604abf6f95a46a3c1",
          "message": "fix(deps): update rust crate toml to 0.8.9",
          "timestamp": "2024-01-31T18:23:12Z",
          "tree_id": "e1ebec28a763b59edd574351e479474ba91536ab",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9e7ca2f268d8f5c76e964ad604abf6f95a46a3c1"
        },
        "date": 1706743122206,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10172908,
            "range": "± 119845",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 130710469,
            "range": "± 1220749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2596821904,
            "range": "± 80465723",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 828585274,
            "range": "± 3482903",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "76d55130500e9cceafb78a3da486cd8d6a445a74",
          "message": "chore(deps): update debian:bullseye-slim docker digest to 8481471",
          "timestamp": "2024-02-01T01:36:47Z",
          "tree_id": "4a78bdbda88a24196de42d4ac91bee4f06acd45b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/76d55130500e9cceafb78a3da486cd8d6a445a74"
        },
        "date": 1706763069427,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10168880,
            "range": "± 1308125",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 128497625,
            "range": "± 2529750",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2625110404,
            "range": "± 81124160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 819515983,
            "range": "± 3158345",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "8d4dbee8252e86ca08c1dfa2950a2bf6ff0ce6d8",
          "message": "fix(deps): update rust crate indexmap to 2.2.2",
          "timestamp": "2024-02-01T04:28:59Z",
          "tree_id": "030411ad0bf952e55b9d359b40f9884caca7ab44",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8d4dbee8252e86ca08c1dfa2950a2bf6ff0ce6d8"
        },
        "date": 1706770207780,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10064318,
            "range": "± 74665",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129117485,
            "range": "± 3570441",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2670533597,
            "range": "± 87382641",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 818441901,
            "range": "± 3128852",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "e9157a347b4a8ab371b9866169e452690dcc8209",
          "message": "chore(deps): update dependency urllib3 to v2.2.0",
          "timestamp": "2024-02-01T16:30:36Z",
          "tree_id": "f5053c617dfc9cf9157da5c2c20cacb13463c949",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e9157a347b4a8ab371b9866169e452690dcc8209"
        },
        "date": 1706813608146,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10107719,
            "range": "± 1699646",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129248465,
            "range": "± 1704251",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2663511380,
            "range": "± 111298628",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 819791910,
            "range": "± 2696768",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "2feeccc2fd5a226cf0af2544f49fd583d4845ba1",
          "message": "chore(deps): update taiki-e/install-action digest to f0940d2",
          "timestamp": "2024-02-01T18:27:43Z",
          "tree_id": "ca7da09e2e1e17f1726daabe6a6e53c14ef87de3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2feeccc2fd5a226cf0af2544f49fd583d4845ba1"
        },
        "date": 1706825322412,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10064860,
            "range": "± 161902",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129411117,
            "range": "± 1382066",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2665554036,
            "range": "± 145184227",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 816104023,
            "range": "± 3039101",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "95058dbb335b6b39292def6b9642623629814c3e",
          "message": "chore(deps): update taiki-e/install-action digest to cf31de7",
          "timestamp": "2024-02-04T00:10:49Z",
          "tree_id": "9468c072d67fc7cdf53ab10e49e8d993285b20a1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/95058dbb335b6b39292def6b9642623629814c3e"
        },
        "date": 1707017833143,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10198174,
            "range": "± 419608",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 131458185,
            "range": "± 2607849",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2614300318,
            "range": "± 94334922",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 829495430,
            "range": "± 2187283",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "4c4cbfb6794c5d1ec36e5dae43635bbf4825db49",
          "message": "fix(deps): update rust crate tokio to 1.36.0",
          "timestamp": "2024-02-04T03:10:06Z",
          "tree_id": "dadd366a3e0fecaaa5e5582659380590c7f8c1b6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4c4cbfb6794c5d1ec36e5dae43635bbf4825db49"
        },
        "date": 1707034695196,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10155559,
            "range": "± 163445",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129529976,
            "range": "± 2944049",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2759041127,
            "range": "± 110993456",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 824613092,
            "range": "± 3729301",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "8b45b4bda6f8e90557d70dea176d2570318a38dc",
          "message": "chore(deps): update taiki-e/install-action digest to e17a4e2",
          "timestamp": "2024-02-04T07:57:59Z",
          "tree_id": "19043c5cf40efe9eabdf8358ede0117acebbde9a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8b45b4bda6f8e90557d70dea176d2570318a38dc"
        },
        "date": 1707039230085,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10167655,
            "range": "± 471003",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129875782,
            "range": "± 3493203",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2722081300,
            "range": "± 92894503",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 825072739,
            "range": "± 2308720",
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
          "id": "d3817adad76d97fd9ffcacfcb6438e9c6da3b9c8",
          "message": "chore: fix build on latest nightlies\n\nThis is due to `simd-adler32` `nightly` feature depending on a recently\nremoved nightly feature. `crc32fast` is also affected.",
          "timestamp": "2024-02-11T20:51:14+01:00",
          "tree_id": "5e1be41d6303797d51675bc02fc9bc1834d5eedb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d3817adad76d97fd9ffcacfcb6438e9c6da3b9c8"
        },
        "date": 1707682663097,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10109161,
            "range": "± 1491753",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 148090421,
            "range": "± 1673602",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2630091176,
            "range": "± 49865980",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 732402811,
            "range": "± 2827194",
            "unit": "ns/iter"
          }
        ]
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
          "id": "0b6fe11edc0f1ab7ab9ca1882821acc469e9585d",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-02-11T22:19:10Z",
          "tree_id": "cfd1f63638effd826c1fac2317463ee07be18670",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0b6fe11edc0f1ab7ab9ca1882821acc469e9585d"
        },
        "date": 1707691332490,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10094230,
            "range": "± 180634",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 147427770,
            "range": "± 2027182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2649861763,
            "range": "± 42612234",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 732996376,
            "range": "± 3203324",
            "unit": "ns/iter"
          }
        ]
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
          "id": "cf8041989b371f5aca8126103274919eb079f6d4",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-02-12T03:17:09Z",
          "tree_id": "bcb29cc192bc3fd978e4866a7b0cdb7e53ae1577",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cf8041989b371f5aca8126103274919eb079f6d4"
        },
        "date": 1707709235432,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10228662,
            "range": "± 1685714",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 147983558,
            "range": "± 2049631",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2664643173,
            "range": "± 41521208",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 742795184,
            "range": "± 3786622",
            "unit": "ns/iter"
          }
        ]
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
          "id": "e9d7c868354e02ef56def90769f8a9323edc3503",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-02-13T22:16:02Z",
          "tree_id": "03df279c148bed70051e8d4c7558f25b22eb9296",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e9d7c868354e02ef56def90769f8a9323edc3503"
        },
        "date": 1707864162232,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10221232,
            "range": "± 4427871",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121356243,
            "range": "± 2914686",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2940206755,
            "range": "± 28964315",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 746849802,
            "range": "± 1560981",
            "unit": "ns/iter"
          }
        ]
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
          "id": "770a7cad43fe8a1a205303bddbc8809e3cc33bd8",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-03-03T21:29:52Z",
          "tree_id": "334ed44dcc16ee50e119d587a46e4fdedd5f4964",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/770a7cad43fe8a1a205303bddbc8809e3cc33bd8"
        },
        "date": 1709502703540,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10443962,
            "range": "± 131460",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 121786483,
            "range": "± 4604292",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2907263217,
            "range": "± 26505189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 750072891,
            "range": "± 1558897",
            "unit": "ns/iter"
          }
        ]
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
          "id": "90df571f782fc52e8e2b937eeeec25d18d95bdd6",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-03-04T00:12:48Z",
          "tree_id": "e2740e4ddc833524cd3a9b89f88d0c6ce64d5bec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/90df571f782fc52e8e2b937eeeec25d18d95bdd6"
        },
        "date": 1709512446324,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10133146,
            "range": "± 1771887",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119371364,
            "range": "± 3282481",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2903405168,
            "range": "± 21400223",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 746525673,
            "range": "± 1405059",
            "unit": "ns/iter"
          }
        ]
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
          "id": "f47f7f0d246df60e1616cf1287314d15e6a27114",
          "message": "chore(deps): update gcr.io/distroless/static-debian11 docker digest to 072d78b",
          "timestamp": "2024-03-04T04:24:52Z",
          "tree_id": "97cad0e46dcb47bec5ef718392ef95522d32af3d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f47f7f0d246df60e1616cf1287314d15e6a27114"
        },
        "date": 1709527833496,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10249917,
            "range": "± 1522268",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120559925,
            "range": "± 4485031",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2909485636,
            "range": "± 21203159",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 750367954,
            "range": "± 2746139",
            "unit": "ns/iter"
          }
        ]
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
          "id": "9cb7e2d121a51a11c96eb8f6bb0fd7e32991fa9d",
          "message": "chore(deps): update dependency urllib3 to v2.2.1",
          "timestamp": "2024-03-04T07:35:53Z",
          "tree_id": "4f504ef1f12e4a37c67a2b860490c93e18da970e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9cb7e2d121a51a11c96eb8f6bb0fd7e32991fa9d"
        },
        "date": 1709539078568,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10437441,
            "range": "± 1769758",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 120450633,
            "range": "± 1863429",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2899833908,
            "range": "± 26009377",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 749230480,
            "range": "± 2028137",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "d4bc20cd5508f27d478299d6e3091e388c9938a1",
          "message": "fix(deps): update rust crate const-random to 0.1.18",
          "timestamp": "2024-03-04T07:36:33Z",
          "tree_id": "c10a19fb0a6fdc6c4c0cd1954a7247318e13e042",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d4bc20cd5508f27d478299d6e3091e388c9938a1"
        },
        "date": 1709550599301,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10094533,
            "range": "± 36813",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 119433966,
            "range": "± 3360493",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2910925587,
            "range": "± 23490768",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 746711770,
            "range": "± 2221080",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "5057c653fa4aed0444f8e66f86930064db0c9352",
          "message": "chore(deps): update taiki-e/install-action digest to dabb9c1",
          "timestamp": "2024-03-04T17:14:55Z",
          "tree_id": "372cbb2124479ea804a8d4356f0fa5feb108e588",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5057c653fa4aed0444f8e66f86930064db0c9352"
        },
        "date": 1709584573187,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10167000,
            "range": "± 1435903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118770731,
            "range": "± 1601928",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2902735443,
            "range": "± 24687962",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 746748806,
            "range": "± 1672045",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "622f42d32f56385213d01a57d1af953e78ea7f6c",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-03-16T00:50:49Z",
          "tree_id": "8e2ff5f4102ca5360d162663da05e979395e6971",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/622f42d32f56385213d01a57d1af953e78ea7f6c"
        },
        "date": 1710567959991,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10332980,
            "range": "± 89748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114416945,
            "range": "± 4684038",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2635847797,
            "range": "± 95427885",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 833746218,
            "range": "± 2792712",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "244889bdbbda43d295e22de6b5098e48db2a7192",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-03-16T05:23:57Z",
          "tree_id": "ac5ad1e9c7f799c4561691869d41128595cfd520",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/244889bdbbda43d295e22de6b5098e48db2a7192"
        },
        "date": 1710577070532,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10176185,
            "range": "± 200757",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 112831367,
            "range": "± 1323367",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2625085865,
            "range": "± 93800063",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 832445081,
            "range": "± 8222850",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "d5c4596fb500b84bdf6b49d12189f54848a89c54",
          "message": "chore(deps): update taiki-e/install-action digest to a4be0bd",
          "timestamp": "2024-03-16T10:59:15Z",
          "tree_id": "662aa7a02f87e31857f38357a3c4ea8d6c8ad3e3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d5c4596fb500b84bdf6b49d12189f54848a89c54"
        },
        "date": 1710600241110,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10237903,
            "range": "± 557559",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114862045,
            "range": "± 2766147",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2632258870,
            "range": "± 34359922",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 834186906,
            "range": "± 2622764",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "9f926fe88445722c1eaa23b37288d5cebbbeaa26",
          "message": "chore(deps): update taiki-e/install-action digest to ffad143",
          "timestamp": "2024-03-19T02:15:02Z",
          "tree_id": "7f9c8f375a834449a756c7572a358ebc4490a015",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9f926fe88445722c1eaa23b37288d5cebbbeaa26"
        },
        "date": 1710826746520,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10324074,
            "range": "± 70523",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 116496768,
            "range": "± 2688246",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2577842912,
            "range": "± 39663052",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 840408315,
            "range": "± 2716442",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "ec820682e019e1f2a0033a04c2a73983d7a85732",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-03-19T05:16:21Z",
          "tree_id": "f43e08823160b93d6f386c7f64cfe2a6fe520006",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ec820682e019e1f2a0033a04c2a73983d7a85732"
        },
        "date": 1710830706381,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10230588,
            "range": "± 76056",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113684650,
            "range": "± 2172738",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2598639110,
            "range": "± 44489918",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 839157331,
            "range": "± 3200359",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "3bbabbcbe99dc25ed2bb89daf25252c8335a22e4",
          "message": "chore(deps): update taiki-e/install-action digest to f03bd5e",
          "timestamp": "2024-03-19T13:53:21Z",
          "tree_id": "c06e669c92fbdbbee9b2e22ec20939da6da41f8d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3bbabbcbe99dc25ed2bb89daf25252c8335a22e4"
        },
        "date": 1710872942164,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10392568,
            "range": "± 625310",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114611451,
            "range": "± 2081666",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2579403791,
            "range": "± 49230288",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 837243536,
            "range": "± 3497765",
            "unit": "ns/iter"
          }
        ]
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
          "id": "a88e6bd6175d3556d57dcaeb144fc9d797dbc4e9",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2024-03-23T20:11:35Z",
          "tree_id": "0074a520d7232cbe862205ba3b6d0c97cf912ce1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a88e6bd6175d3556d57dcaeb144fc9d797dbc4e9"
        },
        "date": 1711226055965,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10183370,
            "range": "± 1560714",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113374300,
            "range": "± 1446415",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2576654086,
            "range": "± 54145691",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 832677169,
            "range": "± 3228249",
            "unit": "ns/iter"
          }
        ]
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
          "id": "af47019991d2c0562fb24f202e158ae91d64cd30",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2024-03-23T22:38:22Z",
          "tree_id": "a647ff1764498e9be31b5b11c20535b1bec940e7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/af47019991d2c0562fb24f202e158ae91d64cd30"
        },
        "date": 1711234738741,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10092926,
            "range": "± 37061",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113450028,
            "range": "± 2521578",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2558490371,
            "range": "± 51177395",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 833110041,
            "range": "± 3468805",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "7063edb59d9b2d4abc9c1a614805043b6428eff1",
          "message": "chore(deps): update taiki-e/install-action digest to 3068b7d",
          "timestamp": "2024-03-25T02:08:39Z",
          "tree_id": "b5343183a48b38d65201ea82be16d40c71ca6a75",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7063edb59d9b2d4abc9c1a614805043b6428eff1"
        },
        "date": 1711338464641,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10187909,
            "range": "± 36191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113358193,
            "range": "± 2872002",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2564872471,
            "range": "± 29016393",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 835798850,
            "range": "± 3778146",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "83e2acff1ac759bc96c214d15c537f648a3ab834",
          "message": "chore(deps): update gcr.io/distroless/static-debian11 docker digest to 046b92c",
          "timestamp": "2024-03-28T01:55:02Z",
          "tree_id": "7655c730a392fb2f338258aa05a1e2cf56822601",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/83e2acff1ac759bc96c214d15c537f648a3ab834"
        },
        "date": 1711602456252,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10047436,
            "range": "± 58903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113689933,
            "range": "± 2972329",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2643905190,
            "range": "± 107375789",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 825601552,
            "range": "± 3635084",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "c3c0dd6e2faf90336db274b8ca1f2e01e82bebbb",
          "message": "chore(deps): update taiki-e/install-action digest to 0329ca5",
          "timestamp": "2024-03-28T04:44:50Z",
          "tree_id": "b88369c4fba83f2956a1d89f3730039d36525677",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c3c0dd6e2faf90336db274b8ca1f2e01e82bebbb"
        },
        "date": 1711608242588,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10248976,
            "range": "± 76976",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 118954937,
            "range": "± 1704555",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2618267689,
            "range": "± 32194712",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 828699783,
            "range": "± 2111537",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "541f96835548291bc1650833b1890a8d076baa96",
          "message": "chore(deps): update dependency filelock to v3.13.3",
          "timestamp": "2024-03-28T06:21:29Z",
          "tree_id": "adccda6354b035c469d9a8a2b831891a57fdd271",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/541f96835548291bc1650833b1890a8d076baa96"
        },
        "date": 1711623150614,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10161198,
            "range": "± 946948",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114172068,
            "range": "± 2017721",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2588441586,
            "range": "± 35515743",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 826570098,
            "range": "± 3149148",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "70fa3405dc90bbc8cb9e7e24610cf92ce4900748",
          "message": "fix(deps): update rust crate serde_json to 1.0.115",
          "timestamp": "2024-03-28T10:32:28Z",
          "tree_id": "03f460b054f632a712c244f6dec55e6d3da60a2e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/70fa3405dc90bbc8cb9e7e24610cf92ce4900748"
        },
        "date": 1711638541827,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10392362,
            "range": "± 1535565",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 116339637,
            "range": "± 1688104",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2669184640,
            "range": "± 65899068",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 828853989,
            "range": "± 4011755",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "4326225f6ba740cc7c1200551ecb13632ff1fd6b",
          "message": "chore(deps): update taiki-e/install-action digest to 10b774e",
          "timestamp": "2024-03-28T14:46:58Z",
          "tree_id": "caf5b8fb0e2ab8b4f03b8572f3f8c3dc54b6df73",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4326225f6ba740cc7c1200551ecb13632ff1fd6b"
        },
        "date": 1711645178077,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10492106,
            "range": "± 1399905",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115262920,
            "range": "± 1687044",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2621656108,
            "range": "± 48363988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 826249894,
            "range": "± 3527681",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "c143bd46b11794cc12a074f0ace2449ec1576179",
          "message": "fix(deps): update rust crate tokio to 1.37.0",
          "timestamp": "2024-03-28T16:39:28Z",
          "tree_id": "6eabaf96f48eadf4266cb4120ce79747de97ce77",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c143bd46b11794cc12a074f0ace2449ec1576179"
        },
        "date": 1711653915364,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10284774,
            "range": "± 142901",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115546297,
            "range": "± 3292498",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2637020179,
            "range": "± 69490767",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 827623881,
            "range": "± 2697049",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "d77ab6e3f8246e91d6e63b1c39f42a10e5203d46",
          "message": "chore(deps): update taiki-e/install-action digest to b23c839",
          "timestamp": "2024-03-31T01:46:15Z",
          "tree_id": "4a460f94ca4bf7b9c4a5cc79f4ac1601c92c18b0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d77ab6e3f8246e91d6e63b1c39f42a10e5203d46"
        },
        "date": 1711864089816,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10201466,
            "range": "± 57051",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114685412,
            "range": "± 2562598",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2590813596,
            "range": "± 48238139",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 833729332,
            "range": "± 1904866",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "12f76ff066a3aa6c23406f1337c835e8ebb9e5d8",
          "message": "chore(deps): update taiki-e/install-action digest to d475def",
          "timestamp": "2024-04-01T20:43:24Z",
          "tree_id": "863e63cc5cbbfb44370831cfe22ca3bd801d0ab8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12f76ff066a3aa6c23406f1337c835e8ebb9e5d8"
        },
        "date": 1712012156112,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10207882,
            "range": "± 64514",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113839415,
            "range": "± 2076301",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2611374066,
            "range": "± 55531182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 838611674,
            "range": "± 3178160",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "fbecae8a15886e3f2f3c1e02597db84fc54917b4",
          "message": "chore(deps): update gcr.io/distroless/static-debian11 docker digest to 6d31326",
          "timestamp": "2024-04-04T01:57:09Z",
          "tree_id": "df341bcec75f3cc55f425302e14d81bced986e8f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fbecae8a15886e3f2f3c1e02597db84fc54917b4"
        },
        "date": 1712203458431,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10206233,
            "range": "± 56346",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113550932,
            "range": "± 2567245",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2623758242,
            "range": "± 76748936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 838157570,
            "range": "± 3681544",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}