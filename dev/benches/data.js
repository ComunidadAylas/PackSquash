window.BENCHMARK_DATA = {
  "lastUpdate": 1695066114016,
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
          "id": "97ffe325c469fb9e39c9d91206ab2f60a1ef0ae6",
          "message": "chore(deps): pin dependencies (#244)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-30T14:06:25+02:00",
          "tree_id": "8effd26a5e2b957a8c7dbcc1027de9151445fd91",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/97ffe325c469fb9e39c9d91206ab2f60a1ef0ae6"
        },
        "date": 1693399379754,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11357669,
            "range": "± 616467",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202587816,
            "range": "± 9674002",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3798339323,
            "range": "± 49678703",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 311647915,
            "range": "± 2808737",
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
          "id": "a08f953c73ce065b6e4f348e2acb75f38e77f8f8",
          "message": "ci(slsa): do not use GNU-specific `base64` command flag\n\nThe POSIX standard does not define the `base64` command, and it turns\nout that its macOS flavor does not have the `-w` flag, which is\nLinux-specific. Let's use `tr` instead to delete newlines, which is\nspecified by POSIX.",
          "timestamp": "2023-08-30T14:47:36+02:00",
          "tree_id": "1edc31b41d0fa71680b119c7938c91212980099b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a08f953c73ce065b6e4f348e2acb75f38e77f8f8"
        },
        "date": 1693401672681,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9338693,
            "range": "± 239599",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 166283687,
            "range": "± 6592389",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3177960269,
            "range": "± 33497023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259788785,
            "range": "± 2334975",
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
          "id": "22d48092f8530d78cda0283ed336c428524364b0",
          "message": "ci(slsa): more macOS command portability fixes",
          "timestamp": "2023-08-30T15:31:05+02:00",
          "tree_id": "6fb46102aaedeecf9663ed33d55c49ad92a43546",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/22d48092f8530d78cda0283ed336c428524364b0"
        },
        "date": 1693404276014,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9401506,
            "range": "± 496251",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169233615,
            "range": "± 11951801",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3137221041,
            "range": "± 53863990",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 257741409,
            "range": "± 1797717",
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
          "id": "818ecbe54fb016d072739048db4ca8c798a7569b",
          "message": "ci(slsa): test fix for missing subjects in attestations\n\nMatrix jobs overwritted the SLSA subjects output, meaning that not every\ndesired artifact was included.",
          "timestamp": "2023-08-30T16:07:28+02:00",
          "tree_id": "ace295af5e4a1d87ddb6d6ee4f24ceabdaaafe48",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/818ecbe54fb016d072739048db4ca8c798a7569b"
        },
        "date": 1693407081712,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11930108,
            "range": "± 647155",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 197512429,
            "range": "± 3392692",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3283807525,
            "range": "± 92497358",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 326688445,
            "range": "± 7794279",
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
          "id": "50730c92fd2e7fdc23e7d2971cd853527e3f17c7",
          "message": "docs(CHANGELOG): add SLSA provenance",
          "timestamp": "2023-08-30T22:20:17+02:00",
          "tree_id": "724b8bba6dfc985976c672d0e5307cbb0e6085bd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/50730c92fd2e7fdc23e7d2971cd853527e3f17c7"
        },
        "date": 1693428939019,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12171130,
            "range": "± 823986",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 203360788,
            "range": "± 2790904",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3225683150,
            "range": "± 53701270",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 326274083,
            "range": "± 4823051",
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
          "id": "58afce3dc2fa1de800b7b8a0ad0d62143d728b4f",
          "message": "docs(CHANGELOG): reflect different SLSA attestation for containers",
          "timestamp": "2023-08-30T23:55:56+02:00",
          "tree_id": "7712a2392cb1e8679683580048cf4e7965dcdbde",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/58afce3dc2fa1de800b7b8a0ad0d62143d728b4f"
        },
        "date": 1693434422832,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11022673,
            "range": "± 394500",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202458944,
            "range": "± 3610163",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3782695230,
            "range": "± 58614270",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 310857785,
            "range": "± 2654571",
            "unit": "ns/iter"
          }
        ]
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
          "id": "018ac2a95a4626568b2aedd7932b1a72e27de608",
          "message": "chore(deps): update gcr.io/distroless/static-debian11 docker digest to e7e79fb",
          "timestamp": "2023-08-31T00:26:54Z",
          "tree_id": "2d93af45443fc648c6e31fe9e40b821b46143ade",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/018ac2a95a4626568b2aedd7932b1a72e27de608"
        },
        "date": 1693443341935,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11319070,
            "range": "± 263239",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 204815806,
            "range": "± 2716826",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3817961745,
            "range": "± 42560011",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 311434918,
            "range": "± 1870144",
            "unit": "ns/iter"
          }
        ]
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
          "id": "97ddf4006ef9f0b216b8a9f6803285414a7a6684",
          "message": "chore(deps): update rust crate chrono to 0.4.28",
          "timestamp": "2023-08-31T03:36:17Z",
          "tree_id": "e80cf591554840be6c4d5126e6fbb69fab85c28e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/97ddf4006ef9f0b216b8a9f6803285414a7a6684"
        },
        "date": 1693455063092,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9204617,
            "range": "± 153567",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 164229765,
            "range": "± 12693384",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2951870573,
            "range": "± 28499343",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259446635,
            "range": "± 2858119",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eab0dc059c1db4b5db8144b464023e05933d739c",
          "message": "chore(deps): update taiki-e/install-action digest to 6801bd5",
          "timestamp": "2023-09-01T04:09:46Z",
          "tree_id": "98e93160c0544bd16ef0e3f6b619cddd5506e3e1",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eab0dc059c1db4b5db8144b464023e05933d739c"
        },
        "date": 1693556845976,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9410109,
            "range": "± 118124",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 166257663,
            "range": "± 10982443",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2953190437,
            "range": "± 27679317",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 264223454,
            "range": "± 2286303",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a77f6ae13e65e53c52d3a1949fc05171d0c5757a",
          "message": "chore(deps): update dependency soupsieve to v2.5",
          "timestamp": "2023-09-02T13:15:43Z",
          "tree_id": "de31e8ea006eed0ecd726e487846a8924c99f290",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a77f6ae13e65e53c52d3a1949fc05171d0c5757a"
        },
        "date": 1693676929326,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10980425,
            "range": "± 280295",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 206195328,
            "range": "± 3261086",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3884216647,
            "range": "± 60216405",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 315763260,
            "range": "± 2885244",
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
          "id": "eedf2935118541a97ab0b425ad0ad158c910787d",
          "message": "perf: use latest OptiVorbis perf. improvements, reduce build time\n\nBuild time and transitive dependency tree size was greatly reduced by\nnot using `git2` on build-time to gather version metadata, instead\nrelying on CI scripts for that.",
          "timestamp": "2023-09-02T20:30:43+02:00",
          "tree_id": "c13dc9617f4d731ec1759e8fc775325fab23bda6",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/eedf2935118541a97ab0b425ad0ad158c910787d"
        },
        "date": 1693681132008,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9449039,
            "range": "± 160643",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 174821414,
            "range": "± 11298612",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3314975683,
            "range": "± 47016554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 268735334,
            "range": "± 1900809",
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
          "id": "450d8553a09904498bf7e0be24a29e09cb045d75",
          "message": "ci: test fix for dubious ownership on `git describe` invocation",
          "timestamp": "2023-09-02T21:27:59+02:00",
          "tree_id": "143a2022431c24bfdcf36925d854ffcfa4d81b1d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/450d8553a09904498bf7e0be24a29e09cb045d75"
        },
        "date": 1693684359336,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8541888,
            "range": "± 61939",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 158211553,
            "range": "± 19237615",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2730133894,
            "range": "± 18063743",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 246843332,
            "range": "± 3072907",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3052d1042c22e5e9b8a041d47942c57ba7f47eb9",
          "message": "fix(deps): update rust crate thiserror to 1.0.48",
          "timestamp": "2023-09-02T22:49:30Z",
          "tree_id": "1d1225a2e1403afb5099d929981a053c169e1217",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3052d1042c22e5e9b8a041d47942c57ba7f47eb9"
        },
        "date": 1693708195317,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9449534,
            "range": "± 114478",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173200564,
            "range": "± 7011407",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3302167985,
            "range": "± 46922427",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 267474790,
            "range": "± 2518940",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c8f49a23590a5c6c3cbf8a5408e612c108412617",
          "message": "chore(deps): update actions/checkout action to v4 (#247)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2023-09-04T18:08:29+02:00",
          "tree_id": "de00f8fa92383133da920bb4ef3ae1f8e2fd3cb9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c8f49a23590a5c6c3cbf8a5408e612c108412617"
        },
        "date": 1693845617053,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10984583,
            "range": "± 496328",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 201832779,
            "range": "± 3760490",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3654165340,
            "range": "± 77877914",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 317660669,
            "range": "± 5721936",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef5d8df58f677912a08cc7c6297608cee66f5c4d",
          "message": "chore(deps): update taiki-e/install-action digest to 5692c40",
          "timestamp": "2023-09-04T16:08:45Z",
          "tree_id": "cce20300051f1be8309ca6843a97cb69250bfe31",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ef5d8df58f677912a08cc7c6297608cee66f5c4d"
        },
        "date": 1693858674138,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8501572,
            "range": "± 147254",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156537862,
            "range": "± 7172194",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2701251558,
            "range": "± 15652518",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 241525667,
            "range": "± 2533740",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "882413a6d3a3b598ece42171e9d86b82cfc4af6b",
          "message": "chore(deps): update taiki-e/install-action digest to 6cd53f7",
          "timestamp": "2023-09-05T14:31:48Z",
          "tree_id": "3215d42179413e54c234e69daa3899cb5db129f3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/882413a6d3a3b598ece42171e9d86b82cfc4af6b"
        },
        "date": 1693928703826,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12812976,
            "range": "± 878073",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 209489353,
            "range": "± 15557457",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3348848868,
            "range": "± 43888015",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 345047854,
            "range": "± 7508886",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3d42d8b5ba67ec68b8a6ac5dc6dbb1c60cb25775",
          "message": "fix(deps): update rust crate walkdir to 2.4.0",
          "timestamp": "2023-09-05T15:11:56Z",
          "tree_id": "4731a8b87541e5f7ce26ef5192f3277fbe6c3f69",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3d42d8b5ba67ec68b8a6ac5dc6dbb1c60cb25775"
        },
        "date": 1693939480681,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9268220,
            "range": "± 266787",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169195855,
            "range": "± 2063116",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3243635847,
            "range": "± 55265674",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 257851199,
            "range": "± 2115952",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f7bd25cab699217036b9aa7ac069a2f4ba12fc54",
          "message": "fix(deps): update rust crate sysinfo to 0.29.10",
          "timestamp": "2023-09-06T11:43:03Z",
          "tree_id": "ce67e19292747fd6a48121a6e064c46078badd12",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f7bd25cab699217036b9aa7ac069a2f4ba12fc54"
        },
        "date": 1694008678215,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13290079,
            "range": "± 1283885",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 212851051,
            "range": "± 4303357",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3242299671,
            "range": "± 111995684",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 333545258,
            "range": "± 6416134",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c35eab9fb5020f291bf67269849bbd67ce4ff45a",
          "message": "chore(deps): update actions/upload-artifact digest to a8a3f3a",
          "timestamp": "2023-09-06T20:12:40Z",
          "tree_id": "832c631fa4b0d128b3bd95cab9e6991281b3ec37",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c35eab9fb5020f291bf67269849bbd67ce4ff45a"
        },
        "date": 1694043660470,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13466825,
            "range": "± 1544666",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 234887704,
            "range": "± 18607643",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3755372367,
            "range": "± 97568262",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 331207142,
            "range": "± 10317902",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "659337926d38a3c3dbbaa49f8ea9dfac29a37ee4",
          "message": "chore(deps): update debian:bullseye-slim docker digest to 3bc5e94",
          "timestamp": "2023-09-07T03:15:13Z",
          "tree_id": "5021db0e5c5c323bc80d9b700246c997bff6df0d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/659337926d38a3c3dbbaa49f8ea9dfac29a37ee4"
        },
        "date": 1694070451076,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12137238,
            "range": "± 429681",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 202263881,
            "range": "± 2293016",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3265414759,
            "range": "± 39521516",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 325012645,
            "range": "± 4777266",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "049135cdef93f99885f377d14993f44bdd2db007",
          "message": "fix(deps): update rust crate bytes to 1.5.0",
          "timestamp": "2023-09-07T10:00:23Z",
          "tree_id": "a80f9337ab2505e1fce788dc6f55ca768950bd4d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/049135cdef93f99885f377d14993f44bdd2db007"
        },
        "date": 1694097350558,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9333353,
            "range": "± 273587",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 173865288,
            "range": "± 7228452",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3290647457,
            "range": "± 51129476",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266485658,
            "range": "± 1949849",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad67d963678d41eaedde4ca799eda034ba647529",
          "message": "chore(deps): update taiki-e/install-action digest to a6b28c3",
          "timestamp": "2023-09-08T04:22:52Z",
          "tree_id": "cf6e346df13099a9d7eaa5caa2ded1730f3bc0ce",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ad67d963678d41eaedde4ca799eda034ba647529"
        },
        "date": 1694156426820,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9792912,
            "range": "± 322127",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 170678323,
            "range": "± 7905971",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3043468007,
            "range": "± 43247941",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 266204021,
            "range": "± 1667224",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ebd8a9d84049dd0f80691304a78768680330ede8",
          "message": "fix(deps): update rust crate toml to 0.7.7",
          "timestamp": "2023-09-08T06:30:21Z",
          "tree_id": "67a993eedb439432185e6795f022be7a4d79de8b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ebd8a9d84049dd0f80691304a78768680330ede8"
        },
        "date": 1694167687744,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9292859,
            "range": "± 214673",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167398255,
            "range": "± 8383900",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3033096207,
            "range": "± 30774876",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 263875797,
            "range": "± 2628231",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "63d19081a415083dbe190e2e0ad3497b939d5e8a",
          "message": "chore(deps): update docker/build-push-action digest to 0a97817",
          "timestamp": "2023-09-08T14:30:36Z",
          "tree_id": "0d00a676ba6555ca8d8a4c86312e17f23c9c695d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/63d19081a415083dbe190e2e0ad3497b939d5e8a"
        },
        "date": 1694195152538,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8402459,
            "range": "± 9380100",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 157081256,
            "range": "± 2572254",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2696300758,
            "range": "± 34907571",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 244442125,
            "range": "± 3646965",
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
          "id": "e3a69ee7b1fee3168109a9215a266de69ce66988",
          "message": "docs(CHANGELOG): add entry for eedf2935118541a97ab0b425ad0ad158c910787d",
          "timestamp": "2023-09-08T22:00:55+02:00",
          "tree_id": "2e98c72a8e2b723848c4364438c9cb3af5e4fbe3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e3a69ee7b1fee3168109a9215a266de69ce66988"
        },
        "date": 1694205251796,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8445301,
            "range": "± 114520",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 158992470,
            "range": "± 11383021",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2698432432,
            "range": "± 27280427",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 244958938,
            "range": "± 2436858",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4527b469d8ba1f3d29e9441c68ea0cf28e4b578b",
          "message": "chore(deps): update taiki-e/install-action digest to f3f0bc9",
          "timestamp": "2023-09-08T20:26:47Z",
          "tree_id": "e3dcd4d9b757bc8b8789a35933ac495bcce74394",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4527b469d8ba1f3d29e9441c68ea0cf28e4b578b"
        },
        "date": 1694216802326,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9245249,
            "range": "± 63202",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 169440233,
            "range": "± 1280941",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3034440462,
            "range": "± 47271108",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 262525797,
            "range": "± 2863774",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "072fda8bb08e625d14838f8a4ed172ff0f7bd517",
          "message": "fix(deps): update rust crate toml to 0.7.8",
          "timestamp": "2023-09-09T03:08:31Z",
          "tree_id": "0bf01b18b69b40a5dfd1de03cd16a697526f5b8b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/072fda8bb08e625d14838f8a4ed172ff0f7bd517"
        },
        "date": 1694247961316,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 9814537,
            "range": "± 2822702",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 167365920,
            "range": "± 5234602",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2999936261,
            "range": "± 43550311",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 259283972,
            "range": "± 3055836",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3ca845c42230231b6e43882612bc39142a703cf9",
          "message": "chore(deps): update taiki-e/install-action digest to c2391e8",
          "timestamp": "2023-09-09T07:58:24Z",
          "tree_id": "262f71b09bc3443cf152cd5a2d1507444255faad",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/3ca845c42230231b6e43882612bc39142a703cf9"
        },
        "date": 1694252206366,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13576192,
            "range": "± 671861",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 213355101,
            "range": "± 3600509",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 3444269575,
            "range": "± 59685301",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 339279769,
            "range": "± 4628360",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1844c6bfc31230edbcb91ad7307d7a7344826ea",
          "message": "chore(deps): update taiki-e/install-action digest to 230cf1a",
          "timestamp": "2023-09-09T12:47:33Z",
          "tree_id": "6284368d1ce9d89b506c6a2d7b38d8c858234716",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a1844c6bfc31230edbcb91ad7307d7a7344826ea"
        },
        "date": 1694277260826,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 8292743,
            "range": "± 271656",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 156803222,
            "range": "± 5553258",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2658600828,
            "range": "± 27837407",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 240526156,
            "range": "± 2696874",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1512447cdd801aeb3803424b078a3787d2d905bb",
          "message": "fix(deps): update rust crate serde_json to 1.0.106",
          "timestamp": "2023-09-09T22:25:38Z",
          "tree_id": "93ecb8ab71669ec710cccd6dfecace2598256eb0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1512447cdd801aeb3803424b078a3787d2d905bb"
        },
        "date": 1694309479291,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11685456,
            "range": "± 733084",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 195239463,
            "range": "± 2113954",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 2955859230,
            "range": "± 63920585",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 301031640,
            "range": "± 7912852",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
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
      }
    ]
  }
}