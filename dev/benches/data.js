window.BENCHMARK_DATA = {
  "lastUpdate": 1752419146019,
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
          "id": "08c2d57ba495a7aa2bf98b393d76776bce44b8a2",
          "message": "perf: use newer Zopfli commits with performance improvements\n\nIn some preliminary tests with a server resource pack and the\n`recompress_compressed_files` option enabled, this reduced runtime by\n15%.",
          "timestamp": "2025-03-31T01:49:21+02:00",
          "tree_id": "690c95bc870fa58a010909b96b2c4b09eb551b84",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/08c2d57ba495a7aa2bf98b393d76776bce44b8a2"
        },
        "date": 1743380054433,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9805415,
            "range": "± 126850",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115261293,
            "range": "± 2254309",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 789963467,
            "range": "± 18152107",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 755962219,
            "range": "± 3003626",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b86d3bd7a651c1292d5c7fe89a9cfb9b9031e217",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-03-31T03:28:04Z",
          "tree_id": "4950f5303e1ef4ebd43935ff2127cc363106b8f0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b86d3bd7a651c1292d5c7fe89a9cfb9b9031e217"
        },
        "date": 1743400651966,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9859707,
            "range": "± 254613",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114325124,
            "range": "± 2624596",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 788427969,
            "range": "± 19222359",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 750186351,
            "range": "± 4234268",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8ee98690e7f51d3d66cb2455e22a9b51ced020af",
          "message": "fix(deps): update rust crate sysinfo to 0.34.0",
          "timestamp": "2025-03-31T05:36:04Z",
          "tree_id": "71cf426f897d0b9335a5d313fb56355fc10e0318",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8ee98690e7f51d3d66cb2455e22a9b51ced020af"
        },
        "date": 1743422606795,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10007911,
            "range": "± 130358",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115731047,
            "range": "± 2294755",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 784654353,
            "range": "± 19416007",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 755758737,
            "range": "± 2650478",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "443935a411299b9fced39e57a2a1b9e978c750ca",
          "message": "chore(deps): update taiki-e/install-action digest to daa3c1f",
          "timestamp": "2025-03-31T16:06:04Z",
          "tree_id": "fc4adef5f8d8f4d997c044aa4bbb997d95fd01d1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/443935a411299b9fced39e57a2a1b9e978c750ca"
        },
        "date": 1743451131383,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9693566,
            "range": "± 74542",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114116389,
            "range": "± 2545443",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 795621176,
            "range": "± 19814583",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 755913204,
            "range": "± 3502093",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e849cbcaeddddd90d679da2a0f0d52d81ae1f29f",
          "message": "fix(deps): update rust crate wmi to v0.15.2",
          "timestamp": "2025-03-31T19:39:35Z",
          "tree_id": "49cab2b7b42c248682d8e8a1b96f3593539336b9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e849cbcaeddddd90d679da2a0f0d52d81ae1f29f"
        },
        "date": 1743463413474,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9932723,
            "range": "± 1432210",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114555164,
            "range": "± 890454",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 794067524,
            "range": "± 17177602",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 753911934,
            "range": "± 5175926",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d8314f14e538b3b6fa7c88a0fdad8b64ae8e2c2",
          "message": "chore(deps): update taiki-e/install-action digest to 575f713",
          "timestamp": "2025-04-01T16:50:49Z",
          "tree_id": "b757742f521947bb467873935ca9ead07a45334f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0d8314f14e538b3b6fa7c88a0fdad8b64ae8e2c2"
        },
        "date": 1743550700442,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9822534,
            "range": "± 102480",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114920053,
            "range": "± 2200677",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 786576969,
            "range": "± 14563873",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 757275608,
            "range": "± 4476400",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "262e76f4703ee5688eea776e10fbb22df53a2126",
          "message": "chore(deps): update taiki-e/install-action digest to f1390fd",
          "timestamp": "2025-04-04T02:34:27Z",
          "tree_id": "797ab24961d51982b095d8d20362276affa55063",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/262e76f4703ee5688eea776e10fbb22df53a2126"
        },
        "date": 1743745314035,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9767138,
            "range": "± 350300",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114084815,
            "range": "± 2591753",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 776446415,
            "range": "± 14408120",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 740202995,
            "range": "± 3171994",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f86d3d4656f87e41b201c2269c65002402d54a53",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2025-04-04T05:19:11Z",
          "tree_id": "baa7f7bd1ab816c7d235248b359474cfee3af069",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f86d3d4656f87e41b201c2269c65002402d54a53"
        },
        "date": 1743768875819,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9642571,
            "range": "± 119160",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113221581,
            "range": "± 1572422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 782936669,
            "range": "± 16084910",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 754461737,
            "range": "± 5731108",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "a5689ff116b8bbec037c7fc21b2d8fa826c18a7e",
          "message": "chore(deps): update taiki-e/install-action digest to d4635f2",
          "timestamp": "2025-04-07T03:24:33Z",
          "tree_id": "38afd83acb214e0dca6d52bc529a6556dbbde631",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a5689ff116b8bbec037c7fc21b2d8fa826c18a7e"
        },
        "date": 1744011076217,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9683639,
            "range": "± 95831",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114573651,
            "range": "± 2016832",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 768456865,
            "range": "± 12419483",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 751836363,
            "range": "± 4048397",
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
          "id": "caa71426bc6d39d05a7bb1268e7f3720e5a259f7",
          "message": "chore: fix build for `musl` targets after dependency updates",
          "timestamp": "2025-04-09T22:47:47+02:00",
          "tree_id": "1e9a83d3c8fb8c65e71426e2f735b04bce729d64",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/caa71426bc6d39d05a7bb1268e7f3720e5a259f7"
        },
        "date": 1744232957009,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9788643,
            "range": "± 1367442",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114683640,
            "range": "± 1351749",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 761457568,
            "range": "± 13414684",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748172905,
            "range": "± 2695937",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c12d9dded380d85ea3deb7c2c3f62819d01f44c",
          "message": "chore(deps): update ci dependencies (#357)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-10T02:49:38Z",
          "tree_id": "d5055526261e13af55ce3eb9d3d55ed0c4c11f25",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8c12d9dded380d85ea3deb7c2c3f62819d01f44c"
        },
        "date": 1744254745204,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10231358,
            "range": "± 201093",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115550308,
            "range": "± 2391481",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 773457496,
            "range": "± 30752542",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 752004590,
            "range": "± 4372236",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "92783d21175e13504ace201f6bfe3c3997d9f361",
          "message": "chore(deps): update taiki-e/install-action digest to 5e434d4",
          "timestamp": "2025-04-13T02:45:45Z",
          "tree_id": "113eda370fdc26f09784d838e4119b0f05cf4682",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/92783d21175e13504ace201f6bfe3c3997d9f361"
        },
        "date": 1744526875201,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9683083,
            "range": "± 95276",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113843013,
            "range": "± 2778462",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 784839844,
            "range": "± 13691195",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748858044,
            "range": "± 3093280",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ea5913708291f2fcb60877a85609af03e0b72864",
          "message": "chore(deps): update dependency urllib3 to v2.4.0",
          "timestamp": "2025-04-13T06:23:52Z",
          "tree_id": "7524ab2e57aa8ffe250de462c86016a34e91a97a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ea5913708291f2fcb60877a85609af03e0b72864"
        },
        "date": 1744543104557,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9850330,
            "range": "± 1039550",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115329135,
            "range": "± 1741170",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 781330688,
            "range": "± 15851328",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 755611149,
            "range": "± 3698275",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3942232ea055d888270853ed102fa15aeadaebe3",
          "message": "chore(deps): update taiki-e/install-action digest to be7c31b",
          "timestamp": "2025-04-13T18:23:16Z",
          "tree_id": "b607892b05cccbb03b4f2e6d1c650401afac7edf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3942232ea055d888270853ed102fa15aeadaebe3"
        },
        "date": 1744582103040,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9637803,
            "range": "± 103604",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114215246,
            "range": "± 1322380",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 786891980,
            "range": "± 13460479",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748821333,
            "range": "± 4110955",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4da54b10366902a459f7881ccbc09924e7229db6",
          "message": "chore(deps): update taiki-e/install-action digest to 09dc018",
          "timestamp": "2025-04-16T03:27:48Z",
          "tree_id": "632aa7eb7b1c9ea22792c4b2a9f5d13eca06ad78",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4da54b10366902a459f7881ccbc09924e7229db6"
        },
        "date": 1744790199472,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9728336,
            "range": "± 431795",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114295092,
            "range": "± 788023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 788708539,
            "range": "± 13183247",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 756160353,
            "range": "± 4329447",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "53696978dd3598d3a60ddb5c657ac8689c3ee3a5",
          "message": "fix(deps): update rust crate wmi to 0.16.0",
          "timestamp": "2025-04-16T16:07:06Z",
          "tree_id": "30d3da580091fb9d46d28fe2ef73718065c28aa6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/53696978dd3598d3a60ddb5c657ac8689c3ee3a5"
        },
        "date": 1744829880577,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9709484,
            "range": "± 223352",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115126291,
            "range": "± 1899043",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 786018990,
            "range": "± 18136679",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 757381775,
            "range": "± 4648590",
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
          "id": "9794a648d7ba45599a715777620172c2b0398412",
          "message": "chore: update dependencies, use new `zopfli` release",
          "timestamp": "2025-04-18T17:57:19+02:00",
          "tree_id": "8fd5ce9d319dff16f026e42d808e09c57dc95691",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9794a648d7ba45599a715777620172c2b0398412"
        },
        "date": 1744993283321,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9813289,
            "range": "± 1296649",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113392259,
            "range": "± 932566",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 765829734,
            "range": "± 14358058",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748653010,
            "range": "± 4908352",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b15dfbce37c28a0bf61810284ab441347d24f33b",
          "message": "chore(deps): update dependency beautifulsoup4 to v4.13.4",
          "timestamp": "2025-04-19T01:40:05Z",
          "tree_id": "c0ebd3ad81139a926c7c65641370e1f483fa2c48",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b15dfbce37c28a0bf61810284ab441347d24f33b"
        },
        "date": 1745047990769,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10019929,
            "range": "± 82551",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115913953,
            "range": "± 2653312",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 786337691,
            "range": "± 14022063",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 760983243,
            "range": "± 4802010",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4c80b0313af3b4304cb4a2ad282880b6fb97abdd",
          "message": "chore(deps): update taiki-e/install-action digest to 9903ab6",
          "timestamp": "2025-04-22T03:39:44Z",
          "tree_id": "eafefa648f7ca19eec4c5222cbfa3d6240ac6161",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4c80b0313af3b4304cb4a2ad282880b6fb97abdd"
        },
        "date": 1745304795527,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9512077,
            "range": "± 70721",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 113580091,
            "range": "± 1024517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 768533363,
            "range": "± 12707889",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 745786999,
            "range": "± 3143717",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a36c41646e65a8aab2c63aa3f1a9cbe52efbfc1",
          "message": "fix(deps): update rust crate wmi to 0.17.0",
          "timestamp": "2025-04-22T06:30:25Z",
          "tree_id": "402f3c86beedf99dc7e5bc70dce720cfd846eff0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0a36c41646e65a8aab2c63aa3f1a9cbe52efbfc1"
        },
        "date": 1745321076319,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9575168,
            "range": "± 2917746",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 114807613,
            "range": "± 869371",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 780291813,
            "range": "± 12467650",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 753830084,
            "range": "± 2931757",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d278dbd99ade341f9af3745c73d0bee9eac58d11",
          "message": "chore(deps): update dependency soupsieve to v2.7",
          "timestamp": "2025-04-22T19:07:15Z",
          "tree_id": "e5063e67f62f1032feda747d121be9d2ce737263",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d278dbd99ade341f9af3745c73d0bee9eac58d11"
        },
        "date": 1745363600953,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9673225,
            "range": "± 511999",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140088806,
            "range": "± 3235115",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 746784511,
            "range": "± 10516979",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 685900113,
            "range": "± 2103383",
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
          "id": "a78ed4a3a526eb0b3f89fb7e75edbb7e18a2cf80",
          "message": "chore(rust): dependency updates",
          "timestamp": "2025-04-26T01:13:10+02:00",
          "tree_id": "9a4b13bc76936e657a1a5fbd8128fc20bd817ef5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a78ed4a3a526eb0b3f89fb7e75edbb7e18a2cf80"
        },
        "date": 1745624245609,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10192565,
            "range": "± 305997",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 141155541,
            "range": "± 2140409",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 762354961,
            "range": "± 9692189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 691729175,
            "range": "± 1332366",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1dbb6286e8c56a20e8372018ae01d889c2e772ef",
          "message": "chore(deps): update ci dependencies (#358)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-26T03:45:53Z",
          "tree_id": "c57e33f16c77046bfa6ed2971724ddf268d8ed42",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1dbb6286e8c56a20e8372018ae01d889c2e772ef"
        },
        "date": 1745640640764,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9495484,
            "range": "± 99327",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138485453,
            "range": "± 1692757",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 729904281,
            "range": "± 11072169",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 678103120,
            "range": "± 2061358",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c253c89babba04e7f455cd00366e50f2094c1843",
          "message": "chore(deps): update taiki-e/install-action digest to ab3728c",
          "timestamp": "2025-04-28T01:59:32Z",
          "tree_id": "228f157a90e209f1ef4047462d8315f48d807003",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c253c89babba04e7f455cd00366e50f2094c1843"
        },
        "date": 1745828807531,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9805513,
            "range": "± 142673",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140790113,
            "range": "± 1292634",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 757064136,
            "range": "± 7499554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 695445461,
            "range": "± 1837410",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5236496a0d09d71dc85550bd2cfdeaf8ed930317",
          "message": "fix(deps): update rust crate oxipng to v9.1.5 (#359)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-04-28T11:57:23Z",
          "tree_id": "9fcf04adadfbb5810fd0e66cf3a8e94184c10515",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/5236496a0d09d71dc85550bd2cfdeaf8ed930317"
        },
        "date": 1745842702271,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10012610,
            "range": "± 1336802",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139634296,
            "range": "± 2329418",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 755709247,
            "range": "± 7207613",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 688971803,
            "range": "± 2868784",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9365d195005a7b3ceac4a0a997504c544ea4c6e0",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2025-05-01T01:54:16Z",
          "tree_id": "937e0ec19f5d4f1b8ec71e299d81817968109c94",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9365d195005a7b3ceac4a0a997504c544ea4c6e0"
        },
        "date": 1746085346904,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9665826,
            "range": "± 1081175",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139251917,
            "range": "± 2458070",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 752864473,
            "range": "± 12002676",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 691558489,
            "range": "± 2564937",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6a774482cb896bb697c7611730f83ae43d7c5efc",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-05-01T07:18:31Z",
          "tree_id": "fd6f0995d7ab42aac4fa51f8d173c65aef6d0a3f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6a774482cb896bb697c7611730f83ae43d7c5efc"
        },
        "date": 1746099204124,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10087142,
            "range": "± 246458",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139832260,
            "range": "± 2042224",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 749558118,
            "range": "± 17126541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 683670974,
            "range": "± 1745602",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0cb9fe055f877f45ff5bd640eb7a17eaed0c1ea6",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2025-05-04T03:03:41Z",
          "tree_id": "3a9a52c4e059337a731a4527c0e0905c65ab3b95",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0cb9fe055f877f45ff5bd640eb7a17eaed0c1ea6"
        },
        "date": 1746338914389,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9742535,
            "range": "± 1016242",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139069620,
            "range": "± 1924429",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 746503537,
            "range": "± 10224836",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 690166133,
            "range": "± 3292071",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cca16c72c5d28b89c0b10c6f0ccf5d058a7660a5",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-05-04T05:45:03Z",
          "tree_id": "35369c21f3efdc72c9ff976fd4eee604f7c05424",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cca16c72c5d28b89c0b10c6f0ccf5d058a7660a5"
        },
        "date": 1746355972182,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10142078,
            "range": "± 995676",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138989422,
            "range": "± 2725425",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 740971170,
            "range": "± 18797645",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 681135061,
            "range": "± 1951787",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "76918c703002009030be1f3f3e5d9a31c50c615c",
          "message": "chore(deps): update dependency charset-normalizer to v3.4.2",
          "timestamp": "2025-05-04T10:32:59Z",
          "tree_id": "d109c27d02a1ac9934c68b5c58de035b6d34d5b1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/76918c703002009030be1f3f3e5d9a31c50c615c"
        },
        "date": 1746368467686,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9774071,
            "range": "± 106730",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139284159,
            "range": "± 3259654",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 744138942,
            "range": "± 12543903",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 688983777,
            "range": "± 2293237",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d5d889508d87a07e42667e965f3ee5bd996d51f8",
          "message": "chore(deps): update taiki-e/install-action digest to f285525",
          "timestamp": "2025-05-07T00:04:39Z",
          "tree_id": "98e5b78c1e1d2c175ffe810102593ca0afd19526",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d5d889508d87a07e42667e965f3ee5bd996d51f8"
        },
        "date": 1746591978263,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9592111,
            "range": "± 62702",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140091966,
            "range": "± 1170756",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 753456759,
            "range": "± 20764464",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 689673358,
            "range": "± 4047811",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e55d0985932a85fc7edf96cc4b43ea4395b340a0",
          "message": "chore(deps): update taiki-e/install-action digest to 97a83ae",
          "timestamp": "2025-05-07T17:07:03Z",
          "tree_id": "4248dc9ff96e00e8b0dd3d066fa77a55ad1a4c52",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e55d0985932a85fc7edf96cc4b43ea4395b340a0"
        },
        "date": 1746658992314,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9639914,
            "range": "± 71318",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138196770,
            "range": "± 1364664",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 878882482,
            "range": "± 12233155",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 694568885,
            "range": "± 4145944",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "842d30b9e1a7bc5ae28df684bad904425d9825da",
          "message": "fix(deps): update rust crate enumset to v1.1.6",
          "timestamp": "2025-05-07T22:42:36Z",
          "tree_id": "01f3a83327d4e86611fe217fce198fabab8583bb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/842d30b9e1a7bc5ae28df684bad904425d9825da"
        },
        "date": 1746675635294,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10237291,
            "range": "± 218020",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139477716,
            "range": "± 2461475",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 883673014,
            "range": "± 18174257",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 688474723,
            "range": "± 2329860",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1e1387601eddfb4e90dda5883a2637d7922fa22a",
          "message": "chore(deps): update taiki-e/install-action digest to 83254c5",
          "timestamp": "2025-05-10T03:06:42Z",
          "tree_id": "ef1fe5d5546e4a9b098e9df124286126f8abaf02",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1e1387601eddfb4e90dda5883a2637d7922fa22a"
        },
        "date": 1746856855110,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9501783,
            "range": "± 57685",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 137359510,
            "range": "± 1047101",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 885276596,
            "range": "± 15757978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 691138482,
            "range": "± 2403732",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bdca82964339d8695bd5912e49c037fbcbac04f0",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2025-05-10T05:38:01Z",
          "tree_id": "bedc0567e6dadf1e0a972c2e61623b049e2da785",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bdca82964339d8695bd5912e49c037fbcbac04f0"
        },
        "date": 1746872295320,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9570688,
            "range": "± 132811",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 139738241,
            "range": "± 1570809",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 865915856,
            "range": "± 10497900",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 685639104,
            "range": "± 2853717",
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
          "id": "63a5938e87d5e5a83b8815252b30b17e3da1944d",
          "message": "fix(json_file): support arrays and objects nested >128 levels deep\n\nic22487's Hypixel Plus resource pack is creatively (ab)using model\noverrides in a way that works with the game, can't be refactored, and is\nnecessary to achieve the functional goals is wants, so let's support it.\nThanks for getting in touch over Discord!",
          "timestamp": "2025-05-10T21:01:57+02:00",
          "tree_id": "f9ab4bddb1c64c843d5c99bad88240879c5aeaae",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63a5938e87d5e5a83b8815252b30b17e3da1944d"
        },
        "date": 1746905196691,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10074951,
            "range": "± 1224759",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 140611260,
            "range": "± 2715283",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 906254675,
            "range": "± 10195936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 698154098,
            "range": "± 2618681",
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
          "id": "9fdf89ace8c0448131aa932da9e0b0266059fc0e",
          "message": "fix(json_file): support arrays and objects nested >128 levels deep\n\nic22487's Hypixel Plus resource pack is creatively (ab)using model\noverrides in a way that works with the game, can't be refactored, and is\nnecessary to achieve the functional goals is wants, so let's support it.\nThanks for getting in touch over Discord!",
          "timestamp": "2025-05-10T21:04:07+02:00",
          "tree_id": "569f561d677203abcb0fbf86f7ed0012e3115462",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9fdf89ace8c0448131aa932da9e0b0266059fc0e"
        },
        "date": 1746905344952,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10207038,
            "range": "± 449655",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138966304,
            "range": "± 2300733",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 889949266,
            "range": "± 17230667",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 693838210,
            "range": "± 2660557",
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
          "id": "1077eafa655aee3c164fb92d32ba6a3dfe96ceda",
          "message": "feat(json_file): JSON object key sorting",
          "timestamp": "2025-05-10T21:23:03+02:00",
          "tree_id": "a490ba7f964602087057e0f6e0f2ea09e985c459",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1077eafa655aee3c164fb92d32ba6a3dfe96ceda"
        },
        "date": 1746906275872,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9494490,
            "range": "± 81763",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 137892352,
            "range": "± 2528237",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 849224645,
            "range": "± 12649417",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 682294606,
            "range": "± 4694157",
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
          "id": "d6b5723abb563c756920bb8a012c25498e0ef751",
          "message": "feat(json_file): JSON object key sorting",
          "timestamp": "2025-05-10T21:27:11+02:00",
          "tree_id": "1b9bd74121e3deeec18316fc2311443d175baa6a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d6b5723abb563c756920bb8a012c25498e0ef751"
        },
        "date": 1746906467469,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9935708,
            "range": "± 1135505",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138368470,
            "range": "± 2186933",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 836875487,
            "range": "± 16604621",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 678528999,
            "range": "± 1889727",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "73456d673f28009b825afd16dafee1c1d159595e",
          "message": "fix(deps): update rust dependencies",
          "timestamp": "2025-05-13T22:50:41Z",
          "tree_id": "cc238b3a18740e8da3c0f8093c199120be9ceb78",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/73456d673f28009b825afd16dafee1c1d159595e"
        },
        "date": 1747191897888,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9466295,
            "range": "± 92368",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 138235260,
            "range": "± 1963936",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 711356377,
            "range": "± 8655081",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 684506835,
            "range": "± 3315904",
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
          "id": "d2fddd4e0837b04535d5f5c48a979f11beaf851e",
          "message": "chore: fix new Clippy lints",
          "timestamp": "2025-05-18T22:22:11+02:00",
          "tree_id": "8ec7acd81d187b70b9eebbaf0114a887afb39aca",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d2fddd4e0837b04535d5f5c48a979f11beaf851e"
        },
        "date": 1747601200652,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9871535,
            "range": "± 92416",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 128161858,
            "range": "± 1740436",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 793943898,
            "range": "± 10364914",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748311987,
            "range": "± 2389327",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ce6139e58413bf4610240fd03ac633a7f14a342",
          "message": "chore(deps): update ci dependencies (#360)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-19T04:13:33Z",
          "tree_id": "5963b0cecff6b28ed0d3e4b35d16e09d677ee049",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7ce6139e58413bf4610240fd03ac633a7f14a342"
        },
        "date": 1747629442650,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9703196,
            "range": "± 68748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127746297,
            "range": "± 3349868",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 796971817,
            "range": "± 10470605",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 748631277,
            "range": "± 3299520",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8311b594961f80698c43f060975491f6b96c66d5",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-05-25T00:04:50Z",
          "tree_id": "14d20766332841a3dd7bea0ff202b9a87356fcce",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8311b594961f80698c43f060975491f6b96c66d5"
        },
        "date": 1748144112074,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9714375,
            "range": "± 897461",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 126911410,
            "range": "± 3246220",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 799224281,
            "range": "± 10127551",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 749448718,
            "range": "± 2003159",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "65aeff3b400164937a3604bdca3467cc11a2de01",
          "message": "chore(deps): update taiki-e/install-action digest to 6c6479b",
          "timestamp": "2025-05-25T22:58:25Z",
          "tree_id": "6b42194728462f3cc4534d8bacd8ce4fdf8fd447",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/65aeff3b400164937a3604bdca3467cc11a2de01"
        },
        "date": 1748226176848,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9861009,
            "range": "± 86859",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127702741,
            "range": "± 2931714",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 799770293,
            "range": "± 13211075",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 749364481,
            "range": "± 2325138",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9f6194da59cdf9d5902b3ab4d73996f7aa706715",
          "message": "chore(deps): update taiki-e/install-action digest to 84c2023",
          "timestamp": "2025-05-28T02:36:03Z",
          "tree_id": "29a767cd9123985cc6ea633f5d9c34892c81b8f3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9f6194da59cdf9d5902b3ab4d73996f7aa706715"
        },
        "date": 1748428700100,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9600531,
            "range": "± 64065",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 128680937,
            "range": "± 1283864",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 788175361,
            "range": "± 9379552",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 743818414,
            "range": "± 1846919",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "20ff7cc853b011fa1647b6899224110464ac4faa",
          "message": "chore(deps): update taiki-e/install-action digest to 2ee2c00",
          "timestamp": "2025-05-31T02:37:57Z",
          "tree_id": "60bf40d02fcdb71202f563cef4071f480277fac8",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/20ff7cc853b011fa1647b6899224110464ac4faa"
        },
        "date": 1748672102939,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10479937,
            "range": "± 80352",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127099221,
            "range": "± 2650089",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 778646125,
            "range": "± 10606130",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 742724639,
            "range": "± 3664913",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f27d01a6b944e585437ead9971aeb6db4601c720",
          "message": "chore(deps): update taiki-e/install-action digest to 735e593",
          "timestamp": "2025-05-31T14:23:05Z",
          "tree_id": "da5ef51126bc4a1a4345da657c76c2bde808977c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f27d01a6b944e585437ead9971aeb6db4601c720"
        },
        "date": 1748710319973,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9791455,
            "range": "± 107384",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 127238093,
            "range": "± 2022212",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 782992086,
            "range": "± 11242599",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 739108583,
            "range": "± 2451537",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "735f9e8db3153fe6558eb26d3a223808d4021491",
          "message": "chore(deps): update ci dependencies",
          "timestamp": "2025-06-04T05:45:15Z",
          "tree_id": "212ca27cf92cc0d98971ae57431e2a6ae015ab8d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/735f9e8db3153fe6558eb26d3a223808d4021491"
        },
        "date": 1749040816588,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9605516,
            "range": "± 55221",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 125517546,
            "range": "± 2903076",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 770575130,
            "range": "± 12442366",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 738622010,
            "range": "± 2437468",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "29367c1d42230e593c94b0c3f33c74d97273ae64",
          "message": "chore(deps): update taiki-e/install-action digest to 92f69c1",
          "timestamp": "2025-06-07T10:43:10Z",
          "tree_id": "5915df09874290480a9c4e9131bfc952e21bde94",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/29367c1d42230e593c94b0c3f33c74d97273ae64"
        },
        "date": 1749304760967,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9863196,
            "range": "± 57232",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 129888896,
            "range": "± 1775132",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 800400380,
            "range": "± 12904322",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 747319029,
            "range": "± 3789412",
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
          "id": "febc80d5377481fc724337a67155d119f5cc5e69",
          "message": "chore: non-breaking Rust dependency updates",
          "timestamp": "2025-07-13T16:42:19+02:00",
          "tree_id": "9bfdb692549c6a223f48714e1fa7ec2127c1d79b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/febc80d5377481fc724337a67155d119f5cc5e69"
        },
        "date": 1752419145432,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9737699,
            "range": "± 78828",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 115028647,
            "range": "± 1211153",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 794173122,
            "range": "± 20054222",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 758136433,
            "range": "± 4563668",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}