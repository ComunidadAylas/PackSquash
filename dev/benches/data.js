window.BENCHMARK_DATA = {
  "lastUpdate": 1688579938940,
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
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "58475a5aa5cb3213eff36cc7e5f612bf10da2c1e",
          "message": "chore(deps): update rust crate time to 0.3.22",
          "timestamp": "2023-06-07T21:59:44Z",
          "tree_id": "8e1feea837775be7a8e69aed2de8172e41ea8ead",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/58475a5aa5cb3213eff36cc7e5f612bf10da2c1e"
        },
        "date": 1686195608727,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12363981,
            "range": "± 804157",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 215084261,
            "range": "± 3514344",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3419983856,
            "range": "± 116533397",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 356271738,
            "range": "± 6706939",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "90f963e3bcb32efd73c5e1da105f90dad8c6cfd9",
          "message": "fix(deps): update rust crate serde to 1.0.164",
          "timestamp": "2023-06-08T07:17:35Z",
          "tree_id": "e5a33e5d1a0fc895de51feec962fd70dcdd85564",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/90f963e3bcb32efd73c5e1da105f90dad8c6cfd9"
        },
        "date": 1686223773736,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9103591,
            "range": "± 378357",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 165322803,
            "range": "± 10042762",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2653606260,
            "range": "± 18968304",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 242709052,
            "range": "± 1915576",
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
          "id": "173d0630581999ed6cb94457ac2d5a6c45e0fe9d",
          "message": "refactor: drop `open_files_limit` option\n\nThis option is rather advanced to use, and so far I have not seen a\nlegitimate reason for users to change its default value. The only times\nit's been included in an options file is because people copied the\ncomplete example on the wiki, despite my advice to the contrary.\n\nTo prevent people from shooting themselves in the foot while\naccomodating advanced needs, let's drop the option and make PackSquash\ntry to raise the file descriptor limit as needed for the desired\nconcurrency level. If that fails, fall back to throttling concurrency\nand output an user-friendly warning explaining what happened and how to\nget the intended performance.",
          "timestamp": "2023-06-09T22:01:32+02:00",
          "tree_id": "e382f7988c3929b572ab17545982b5bc9f7a232c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/173d0630581999ed6cb94457ac2d5a6c45e0fe9d"
        },
        "date": 1686343340786,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9366960,
            "range": "± 237182",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 175376133,
            "range": "± 14625875",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3052554483,
            "range": "± 52396161",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 273845486,
            "range": "± 5180443",
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
          "id": "3007b5a37fab2081de0070e1d0613aa721df1bb2",
          "message": "refactor: drop `open_files_limit` option\n\nThis option is rather advanced to use, and so far I have not seen a\nlegitimate reason for users to change its default value. The only times\nit's been included in an options file is because people copied the\ncomplete example on the wiki, despite my advice to the contrary.\n\nTo prevent people from shooting themselves in the foot while\naccomodating advanced needs, let's drop the option and make PackSquash\ntry to raise the file descriptor limit as needed for the desired\nconcurrency level. If that fails, fall back to throttling concurrency\nand output an user-friendly warning explaining what happened and how to\nget the intended performance.\n\nA convenient way to test these changes is by using a command like this\non a Linux workstation:\n\n$ systemd-run --user -GPdt -p LimitNOFILE=<soft>:<hard> target/release/packsquash /tmp/packsquash.toml",
          "timestamp": "2023-06-09T22:07:35+02:00",
          "tree_id": "e382f7988c3929b572ab17545982b5bc9f7a232c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3007b5a37fab2081de0070e1d0613aa721df1bb2"
        },
        "date": 1686344960994,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8409166,
            "range": "± 225788",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 166410616,
            "range": "± 2293022",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2746654739,
            "range": "± 43589134",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259873077,
            "range": "± 9131347",
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
          "id": "b1fd52e75473b06e6cff3c4a49bfb253e85149e9",
          "message": "refactor: drop `open_files_limit` option\n\nThis option is rather advanced to use, and so far I have not seen a\nlegitimate reason for users to change its default value. The only times\nit's been included in an options file is because people copied the\ncomplete example on the wiki, despite my advice to the contrary.\n\nTo prevent people from shooting themselves in the foot while\naccomodating advanced needs, let's drop the option and make PackSquash\ntry to raise the file descriptor limit as needed for the desired\nconcurrency level. If that fails, fall back to throttling concurrency\nand output an user-friendly warning explaining what happened and how to\nget the intended performance.\n\nA convenient way to test these changes is by using a command like this\non a Linux workstation:\n\n$ systemd-run --user -GPdt -p LimitNOFILE=<soft>:<hard> target/release/packsquash /tmp/packsquash.toml",
          "timestamp": "2023-06-09T22:38:21+02:00",
          "tree_id": "78d972a6a29d6c9eba635403e4f6b99bd76170e7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b1fd52e75473b06e6cff3c4a49bfb253e85149e9"
        },
        "date": 1686345548954,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11111625,
            "range": "± 494351",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 208175289,
            "range": "± 2847753",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3574314469,
            "range": "± 142444561",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 341458163,
            "range": "± 7130420",
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
          "id": "2c44a5c6f6153a46e8862beb90823f60d513c41b",
          "message": "refactor: drop `open_files_limit` option\n\nThis option is rather advanced to use, and so far I have not seen a\nlegitimate reason for users to change its default value. The only times\nit's been included in an options file is because people copied the\ncomplete example on the wiki, despite my advice to the contrary.\n\nTo prevent people from shooting themselves in the foot while\naccomodating advanced needs, let's drop the option and make PackSquash\ntry to raise the file descriptor limit as needed for the desired\nconcurrency level. If that fails, fall back to throttling concurrency\nand output an user-friendly warning explaining what happened and how to\nget the intended performance.\n\nA convenient way to test these changes is by using a command like this\non a Linux workstation:\n\n$ systemd-run --user -GPdt -p LimitNOFILE=<soft>:<hard> target/release/packsquash /tmp/packsquash.toml",
          "timestamp": "2023-06-09T22:30:12+02:00",
          "tree_id": "b888a4727738a2e1c28c2d21b6f7cbe373d77791",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2c44a5c6f6153a46e8862beb90823f60d513c41b"
        },
        "date": 1686345727561,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9624612,
            "range": "± 101848",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 177518274,
            "range": "± 6667589",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3068061814,
            "range": "± 57494195",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 281634968,
            "range": "± 8638235",
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
          "id": "aa202456fa21df131462e0127c74dab92d1adf9b",
          "message": "refactor: drop `open_files_limit` option\n\nThis option is rather advanced to use, and so far I have not seen a\nlegitimate reason for users to change its default value. The only times\nit's been included in an options file is because people copied the\ncomplete example on the wiki, despite my advice to the contrary.\n\nTo prevent people from shooting themselves in the foot while\naccomodating advanced needs, let's drop the option and make PackSquash\ntry to raise the file descriptor limit as needed for the desired\nconcurrency level. If that fails, fall back to throttling concurrency\nand output an user-friendly warning explaining what happened and how to\nget the intended performance.\n\nA convenient way to test these changes is by using a command like this\non a Linux workstation:\n\n$ systemd-run --user -GPdt -p LimitNOFILE=<soft>:<hard> target/release/packsquash /tmp/packsquash.toml",
          "timestamp": "2023-06-09T22:40:43+02:00",
          "tree_id": "06eac3385adf5a705bec0d037904cc85dfe3430b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/aa202456fa21df131462e0127c74dab92d1adf9b"
        },
        "date": 1686346948816,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11795619,
            "range": "± 678283",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 207055267,
            "range": "± 3913485",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3227237964,
            "range": "± 83197395",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 331841073,
            "range": "± 7673960",
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
          "id": "df32489d2ef3027ebdacf0b09400f3d9f9bc2cf1",
          "message": "refactor: remove global audio options\n\nThese were a bit weird and complicated, and the file-specific options\nmechanism is powerful enough to implement advanced use cases, even\nthough it might be verbose.\n\nWhile at it, let's assert that no error happens when creating a\nresampler, as such a thing signals an internal logic error worth of\npanicking.",
          "timestamp": "2023-06-09T23:14:48+02:00",
          "tree_id": "0ed715b57629fd2cf8d6102e0165c03bb0e5f752",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/df32489d2ef3027ebdacf0b09400f3d9f9bc2cf1"
        },
        "date": 1686347525818,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9253153,
            "range": "± 1560165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 171598822,
            "range": "± 7107775",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2938299172,
            "range": "± 24652601",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 273352605,
            "range": "± 6416076",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2745f3adb6ad157ab1bfe7e9bc35345a2dcaa8f9",
          "message": "chore(deps): update dependency filelock to v3.12.1",
          "timestamp": "2023-06-10T06:42:56Z",
          "tree_id": "a70c066cdb5826ea5a6a97700ba265002b34f73f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2745f3adb6ad157ab1bfe7e9bc35345a2dcaa8f9"
        },
        "date": 1686397096975,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9313153,
            "range": "± 160920",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 178969224,
            "range": "± 7223247",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3303730081,
            "range": "± 45153872",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 280038459,
            "range": "± 6812904",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "689f9ed49a7f801595aa1bc507ad7998bb98b883",
          "message": "fix(deps): update rust crate log to 0.4.19",
          "timestamp": "2023-06-11T04:01:11Z",
          "tree_id": "4df9d59ded1425fd87efcb6592929afc6dd77871",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/689f9ed49a7f801595aa1bc507ad7998bb98b883"
        },
        "date": 1686471554773,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8991642,
            "range": "± 117604",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173374159,
            "range": "± 23000746",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3061156790,
            "range": "± 99947045",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 293240507,
            "range": "± 7146085",
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
          "id": "2dcc8cea2bf7c2b5aaa62b48f7878574f0e19bed",
          "message": "chore: update rubato to 0.14\n\nThis update entailed some breaking changes. I was careful to test that\naudio files were resampled correctly after the upgrade.",
          "timestamp": "2023-06-11T13:15:18+02:00",
          "tree_id": "f4afaf9172b4d469513c8ced021f468f705b818e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2dcc8cea2bf7c2b5aaa62b48f7878574f0e19bed"
        },
        "date": 1686484035522,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9161370,
            "range": "± 267049",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 170525711,
            "range": "± 663023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2988083960,
            "range": "± 43945692",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 274182014,
            "range": "± 4914374",
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
          "id": "ec85d33f876a6f1774c87feab753888a818d6890",
          "message": "ci: fix AArch64 workflow failures\n\nYou do you, sid-slim package repositories.",
          "timestamp": "2023-06-13T17:02:14+02:00",
          "tree_id": "9c9a617524b7a25eec02ce4bdff52a07a4d67efd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ec85d33f876a6f1774c87feab753888a818d6890"
        },
        "date": 1686671821873,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8365348,
            "range": "± 54038",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167896828,
            "range": "± 11957957",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2739233519,
            "range": "± 51361711",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 263891010,
            "range": "± 9538484",
            "unit": "ns/iter"
          }
        ]
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
          "id": "85fbfb177e7796ec4b57534b16881f66e01c7bc8",
          "message": "chore(deps): update dependency filelock to v3.12.2",
          "timestamp": "2023-06-13T19:45:09Z",
          "tree_id": "5c5b570b0fff49f4202e4610bbc451449be75cfb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/85fbfb177e7796ec4b57534b16881f66e01c7bc8"
        },
        "date": 1686688662825,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11903772,
            "range": "± 522476",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 198900992,
            "range": "± 5637479",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3183468454,
            "range": "± 43731477",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 297358160,
            "range": "± 4706496",
            "unit": "ns/iter"
          }
        ]
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
          "id": "b59330c56a1c8cdf6ef0d5028bcf2e35813e14d7",
          "message": "fix(deps): update rust crate uuid to 1.3.4",
          "timestamp": "2023-06-13T23:18:36Z",
          "tree_id": "990255ff94e929c62cac138cbdb097cbce9e7d15",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b59330c56a1c8cdf6ef0d5028bcf2e35813e14d7"
        },
        "date": 1686700155258,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11668362,
            "range": "± 725256",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 213287089,
            "range": "± 3233513",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3652411693,
            "range": "± 85609604",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 336144783,
            "range": "± 8120844",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "88db84b6eb26a7714e1fc5b7fc4f4f92bbee777a",
          "message": "fix(deps): update rust crate serde_json to 1.0.97",
          "timestamp": "2023-06-16T08:21:12Z",
          "tree_id": "55d8e06aba896040aaa6293ed6d6067720976415",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/88db84b6eb26a7714e1fc5b7fc4f4f92bbee777a"
        },
        "date": 1686913386422,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8375372,
            "range": "± 56791",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 155865467,
            "range": "± 22889465",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2404792858,
            "range": "± 18735752",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 233874876,
            "range": "± 2250476",
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
          "id": "b0b8503c6482a8e30c127478813ec65ae90aaccb",
          "message": "chore: fix Cargo warning due to unspecified workspace resolver version",
          "timestamp": "2023-06-17T13:58:42+02:00",
          "tree_id": "c09fdca6dc3094ce7c63ccbf0c66977b52b6e7f1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b0b8503c6482a8e30c127478813ec65ae90aaccb"
        },
        "date": 1687005154432,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8379362,
            "range": "± 126959",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157738671,
            "range": "± 10831667",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2396551440,
            "range": "± 31116347",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 234070865,
            "range": "± 2318538",
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
          "id": "12598e02b06a0df236fdd5a8be1ec835af22770c",
          "message": "chore: tweak some option comments",
          "timestamp": "2023-06-17T23:20:08+02:00",
          "tree_id": "fb95b0b271f364262f126b0e3d86cf5e084f04b6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12598e02b06a0df236fdd5a8be1ec835af22770c"
        },
        "date": 1687038757618,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10529896,
            "range": "± 395941",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 182572132,
            "range": "± 3585676",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2746763636,
            "range": "± 63563902",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 284533935,
            "range": "± 5867782",
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
          "id": "3bce2a5bfe81662637909039f6e4a3c8e447a46c",
          "message": "chore: bump version to v0.4.0 🎉\n\nI'd like to finish writing some docs, but the new release is coming\nalong nicely!",
          "timestamp": "2023-06-17T23:26:48+02:00",
          "tree_id": "31ebc0720a7022c687790a8c25ea91744339778f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3bce2a5bfe81662637909039f6e4a3c8e447a46c"
        },
        "date": 1687038980569,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8371484,
            "range": "± 147266",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156079457,
            "range": "± 1693269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2384309126,
            "range": "± 23696177",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 234987549,
            "range": "± 1520128",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2e751256f8e42f2af3176241b9ef8c30159f492e",
          "message": "fix(deps): update rust crate aes to 0.8.3",
          "timestamp": "2023-06-18T03:26:01Z",
          "tree_id": "3a761d4c777c8a99b57294a1a780f3826d25eb8f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2e751256f8e42f2af3176241b9ef8c30159f492e"
        },
        "date": 1687086325262,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8669652,
            "range": "± 79001",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156822538,
            "range": "± 32729425",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2415137599,
            "range": "± 19870074",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 236269648,
            "range": "± 1643871",
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
          "id": "18e0c4c8a6bd53a72c765c6be5b5f42a7cf7799d",
          "message": "chore: update some dependencies\n\nThis gets rid of a security advisory reported by cargo-deny relative to\nOptiVorbis' transitive dependency on Ouroboros.",
          "timestamp": "2023-06-18T15:48:14+02:00",
          "tree_id": "54f27724d38b53aa377f668663013e9282bda1bd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/18e0c4c8a6bd53a72c765c6be5b5f42a7cf7799d"
        },
        "date": 1687117166131,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9079973,
            "range": "± 286815",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 161474555,
            "range": "± 11602388",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2451356058,
            "range": "± 26390295",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 238615353,
            "range": "± 15514715",
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
          "id": "2bfec40366a03deb7e1c19066a39214d832ef57f",
          "message": "docs(README): update for upcoming release",
          "timestamp": "2023-06-18T21:08:44+02:00",
          "tree_id": "936ebf024c22cbe0874ad07acb5e7e18dcd47e97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2bfec40366a03deb7e1c19066a39214d832ef57f"
        },
        "date": 1687117249453,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9267430,
            "range": "± 501114",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164929779,
            "range": "± 20274852",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2598062932,
            "range": "± 21976928",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 242658425,
            "range": "± 2779034",
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
          "id": "80019899079baace7d193ea3c41d6dcb1bed95c1",
          "message": "chore(deps): update to latest upstream OxiPNG commit\n\nThese commits contain interesting changes we might as well squeeze in\nbefore v0.4.0 is released.",
          "timestamp": "2023-06-18T21:18:03+02:00",
          "tree_id": "a4118067f143c43539702314a42490c43eb4c200",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/80019899079baace7d193ea3c41d6dcb1bed95c1"
        },
        "date": 1687117741422,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11634792,
            "range": "± 671422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 185195679,
            "range": "± 6307311",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2515162847,
            "range": "± 91371189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266282800,
            "range": "± 5650383",
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
          "id": "90da87ba54ee39285c0407ab8f1a427abce9114e",
          "message": "docs(contributors): add @MKMakroM",
          "timestamp": "2023-06-18T22:24:07+02:00",
          "tree_id": "d2cdf16f34186a575aef5ea6be0789562021139c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/90da87ba54ee39285c0407ab8f1a427abce9114e"
        },
        "date": 1687122723033,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10238192,
            "range": "± 171812",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169914958,
            "range": "± 10087139",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2808183733,
            "range": "± 14917294",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 249384532,
            "range": "± 3080017",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e33c07dccdf3f767ba2144fa8f570cd6889ad999",
          "message": "fix(deps): update rust crate itertools to 0.11.0 (#232)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-06-23T00:29:02+02:00",
          "tree_id": "8ccd7256415cb5807368177266f6b907456076e5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e33c07dccdf3f767ba2144fa8f570cd6889ad999"
        },
        "date": 1687476020178,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9708396,
            "range": "± 160915",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 165783638,
            "range": "± 9173739",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3171517043,
            "range": "± 9320420",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 265940469,
            "range": "± 2817248",
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
          "id": "ac401148d0710c69a5681c2fab88d66bb569b4bd",
          "message": "chore: fix Clippy lint",
          "timestamp": "2023-06-24T00:35:27+02:00",
          "tree_id": "d3e5189dbc77155e5b5a0d3bae769e0c7d4b0d13",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ac401148d0710c69a5681c2fab88d66bb569b4bd"
        },
        "date": 1687561718200,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9397768,
            "range": "± 105223",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167315195,
            "range": "± 10272612",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3020918063,
            "range": "± 52482249",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 264085849,
            "range": "± 3112326",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "63f535ef5c1dfee30ba48cc2b0c801a34d45957d",
          "message": "fix(deps): update rust crate toml to 0.7.5",
          "timestamp": "2023-06-24T03:42:11Z",
          "tree_id": "6f2b3e35b4fc44885a1f01e681d3ceee526026b7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63f535ef5c1dfee30ba48cc2b0c801a34d45957d"
        },
        "date": 1687591397988,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10620753,
            "range": "± 439423",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 191943565,
            "range": "± 3032989",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3735795838,
            "range": "± 85592517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 307489489,
            "range": "± 8058764",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cdcfc59b96e3e01019b506306cb58764a896b83f",
          "message": "fix(deps): update rust crate serde_json to 1.0.99",
          "timestamp": "2023-06-24T06:32:11Z",
          "tree_id": "14cc85743e54a5bcb1ebfd3f5d985c1c83450184",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cdcfc59b96e3e01019b506306cb58764a896b83f"
        },
        "date": 1687602393291,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8400910,
            "range": "± 336013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 153629305,
            "range": "± 5203555",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2737463697,
            "range": "± 9537631",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 247407598,
            "range": "± 3033118",
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
          "id": "f3dd46788e5214d06f4e13ec6366fc74e78bda17",
          "message": "chore: update indexmap to v2.0.0",
          "timestamp": "2023-06-24T12:03:19+02:00",
          "tree_id": "7acd6dac123d0f4ed62b2ada6603d0fb3bee7a86",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f3dd46788e5214d06f4e13ec6366fc74e78bda17"
        },
        "date": 1687604158926,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8695847,
            "range": "± 148783",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 154788450,
            "range": "± 6518066",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2758855518,
            "range": "± 12271109",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245523012,
            "range": "± 3151575",
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
          "id": "e2c949f1cb6ab60a6c5ab05ec2a810d1cf1264da",
          "message": "chore: shorten indexmap dependency declaration",
          "timestamp": "2023-06-24T12:11:09+02:00",
          "tree_id": "3e7c373d7775602f80fc7639580d59df8787968c",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e2c949f1cb6ab60a6c5ab05ec2a810d1cf1264da"
        },
        "date": 1687604195284,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9732796,
            "range": "± 433450",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 168441037,
            "range": "± 10383348",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3105574621,
            "range": "± 11891452",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 270437156,
            "range": "± 2756815",
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
          "id": "bf795a5592c1f048de39ae8a6b84c345bdd59029",
          "message": "docs(contributors): add @MKMakroM",
          "timestamp": "2023-06-24T12:31:55+02:00",
          "tree_id": "cee863a81461e360580d9cc391f0ac3d6ea0548d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/bf795a5592c1f048de39ae8a6b84c345bdd59029"
        },
        "date": 1687604609717,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8352393,
            "range": "± 269604",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 155020043,
            "range": "± 12184389",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2752652788,
            "range": "± 7158352",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245529864,
            "range": "± 3657866",
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
          "id": "d712a4955b27fdc300cf541e8dc21444c1973c6a",
          "message": "docs(README): add contributor, tweak contribution types for a person",
          "timestamp": "2023-06-24T13:55:45+02:00",
          "tree_id": "a7d754498a688dcd178c7e414b6e621461aef39e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d712a4955b27fdc300cf541e8dc21444c1973c6a"
        },
        "date": 1687609606416,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9312485,
            "range": "± 318396",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167253386,
            "range": "± 1019298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3097740219,
            "range": "± 15483938",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 267690252,
            "range": "± 4640426",
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
          "id": "80b8ce9405044317243992d4a37df92edbba8462",
          "message": "docs: add CHANGELOG",
          "timestamp": "2023-06-24T21:37:28+02:00",
          "tree_id": "b00f03dbade17c1acd23525f7803e8c353a0b538",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/80b8ce9405044317243992d4a37df92edbba8462"
        },
        "date": 1687637237399,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9975168,
            "range": "± 136602",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 168169595,
            "range": "± 8568866",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3099055831,
            "range": "± 21642223",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 267916681,
            "range": "± 4519484",
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
          "id": "4ad892a787eaf07af97e2305f85e78e308c84349",
          "message": "docs(README): prettify header",
          "timestamp": "2023-06-24T21:59:09+02:00",
          "tree_id": "8473d5e0b9cc3e02859d166440248d8bd1526721",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4ad892a787eaf07af97e2305f85e78e308c84349"
        },
        "date": 1687639359601,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8291802,
            "range": "± 278684",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 154247427,
            "range": "± 7266172",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2750708521,
            "range": "± 10442869",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 244092539,
            "range": "± 1885729",
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
          "id": "b69f5c955001202c539be32b831e7b536678cd15",
          "message": "docs(README): make Discord join call to action text less targeted to Spanish-speaking audiences\n\nWe welcome anyone regardless of their native language, although for\npractical reasons communication can only happen either in English or\nSpanish.",
          "timestamp": "2023-06-24T22:14:01+02:00",
          "tree_id": "370d60504bbaefc758b23f6e01a3062bdc27a451",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b69f5c955001202c539be32b831e7b536678cd15"
        },
        "date": 1687639485050,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10779549,
            "range": "± 534953",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 183234403,
            "range": "± 2685338",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2918668572,
            "range": "± 29401180",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 287191208,
            "range": "± 4770463",
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
          "distinct": false,
          "id": "9ae83cb03242c883cd13ed223e0e1f492dac7257",
          "message": "docs(CHANGELOG): add missing audio processing change",
          "timestamp": "2023-06-25T12:55:49+02:00",
          "tree_id": "418eaa216e0dca1e6cd4d9f314bddaeb76286311",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9ae83cb03242c883cd13ed223e0e1f492dac7257"
        },
        "date": 1687693643880,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8292123,
            "range": "± 119138",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 155327649,
            "range": "± 12486397",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2766205269,
            "range": "± 8785061",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 245794306,
            "range": "± 1665074",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "AlexTMjugador",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "id": "9ae83cb03242c883cd13ed223e0e1f492dac7257",
          "message": "docs(CHANGELOG): add missing audio processing change",
          "timestamp": "2023-06-25T10:55:49Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9ae83cb03242c883cd13ed223e0e1f492dac7257"
        },
        "date": 1687695264225,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9239483,
            "range": "± 290800",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 165853974,
            "range": "± 1693418",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3029223636,
            "range": "± 8559082",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266217630,
            "range": "± 3525339",
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
          "id": "9ae83cb03242c883cd13ed223e0e1f492dac7257",
          "message": "docs(CHANGELOG): add missing audio processing change",
          "timestamp": "2023-06-25T12:55:49+02:00",
          "tree_id": "418eaa216e0dca1e6cd4d9f314bddaeb76286311",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9ae83cb03242c883cd13ed223e0e1f492dac7257"
        },
        "date": 1687700853745,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9109687,
            "range": "± 104498",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 163731193,
            "range": "± 17648307",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2957091333,
            "range": "± 9710389",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 260297786,
            "range": "± 3730067",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64988d931aec5ebfa806fb2f513db02c8f26678e",
          "message": "fix(deps): update rust crate tokio to 1.29.0",
          "timestamp": "2023-06-27T23:24:25Z",
          "tree_id": "4fe1d5ef667c3976142f2d93115d19479d63d557",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/64988d931aec5ebfa806fb2f513db02c8f26678e"
        },
        "date": 1687922573522,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11990838,
            "range": "± 868756",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 196769514,
            "range": "± 4780732",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3188018858,
            "range": "± 79764554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 319280658,
            "range": "± 9735521",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2cfc2858eccdd21ab429435ee2963a1f68ace622",
          "message": "fix(deps): update rust crate uuid to 1.4.0",
          "timestamp": "2023-06-28T02:29:12Z",
          "tree_id": "805a8a6827ae88e20168dd4c983b869ea1f5e9bb",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2cfc2858eccdd21ab429435ee2963a1f68ace622"
        },
        "date": 1687929779117,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8509497,
            "range": "± 227892",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156528092,
            "range": "± 7327560",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2729297130,
            "range": "± 47281584",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 252327758,
            "range": "± 6419853",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
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
          "id": "37bbbf3d5119dc818a8461d3324d346a9466a2c5",
          "message": "fix(deps): update rust crate sysinfo to 0.29.3",
          "timestamp": "2023-06-29T21:43:28Z",
          "tree_id": "a56ec58c6c7ac775025026293dc6539b501fd5d0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/37bbbf3d5119dc818a8461d3324d346a9466a2c5"
        },
        "date": 1688091159681,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8566005,
            "range": "± 124556",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 175302458,
            "range": "± 7241592",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2751366331,
            "range": "± 31681962",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 254657032,
            "range": "± 5770125",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2936f9c404b968786de6c51e1b428b5aa2ed31b3",
          "message": "fix(deps): update rust crate tokio to 1.29.1",
          "timestamp": "2023-06-30T01:35:01Z",
          "tree_id": "8dd52d1900718e5bafc334149a1982eecefda26f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/2936f9c404b968786de6c51e1b428b5aa2ed31b3"
        },
        "date": 1688099270000,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8417688,
            "range": "± 281672",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 178793985,
            "range": "± 19625626",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2792159579,
            "range": "± 43894958",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 256834921,
            "range": "± 6618462",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e4f38e33bfffe919604732c061a250b4ce45b488",
          "message": "fix(deps): update rust crate rubato to 0.14.1",
          "timestamp": "2023-07-02T22:46:55Z",
          "tree_id": "c7eaeb3bec4fd68e575523d976cef1c318bfb97d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e4f38e33bfffe919604732c061a250b4ce45b488"
        },
        "date": 1688346090637,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9233024,
            "range": "± 89950",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 181114360,
            "range": "± 1099341",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3046374853,
            "range": "± 67845911",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 268355433,
            "range": "± 5696052",
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
      }
    ]
  }
}