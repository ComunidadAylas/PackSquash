window.BENCHMARK_DATA = {
  "lastUpdate": 1643818806761,
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
          "id": "cb67cf04fbe0a1fa4a04ac5e06a024e4683d7b7b",
          "message": "bench: minor tweaks to make it behave nicer",
          "timestamp": "2022-01-30T19:36:35+01:00",
          "tree_id": "e2c5b16db24169235601048fcb6b71b1583a09de",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cb67cf04fbe0a1fa4a04ac5e06a024e4683d7b7b"
        },
        "date": 1643569414584,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15636473,
            "range": "± 540268",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1347088048,
            "range": "± 18596201",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 8948381333,
            "range": "± 79685396",
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
          "id": "cb67cf04fbe0a1fa4a04ac5e06a024e4683d7b7b",
          "message": "bench: minor tweaks to make it behave nicer",
          "timestamp": "2022-01-30T18:36:35Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cb67cf04fbe0a1fa4a04ac5e06a024e4683d7b7b"
        },
        "date": 1643591414224,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15095728,
            "range": "± 847463",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1299990795,
            "range": "± 10756511",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 8498065538,
            "range": "± 78846578",
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
          "id": "d14489f6101ba61f02ca6df789f7d54d9355aec7",
          "message": "chore(legacy_lang_file): fix tests not building due to changes\n\nIn a previous commit we did some changes to the legacy language file\nprocessing code that made a file size hint unnecessary, but forgot to\nupdate the tests accordingly.\n\nRefs: a0216fb884b705571503008c3744171f36a269dd",
          "timestamp": "2022-02-02T16:37:01+01:00",
          "tree_id": "d896483438478c15ae153726e82ac92b923afc81",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d14489f6101ba61f02ca6df789f7d54d9355aec7"
        },
        "date": 1643818806017,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14285221,
            "range": "± 575498",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 970018377,
            "range": "± 33082868",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10537447725,
            "range": "± 181676237",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1443014907,
            "range": "± 19341628",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}