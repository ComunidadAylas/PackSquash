window.BENCHMARK_DATA = {
  "lastUpdate": 1643913451419,
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
          "id": "f3e77a919d63f9e0406649bfcb31b33a3ad1b5a8",
          "message": "chore: fix Windows build",
          "timestamp": "2022-02-02T21:50:52+01:00",
          "tree_id": "be5d68c2d11073ed6057a35335a21b30908e7ad4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f3e77a919d63f9e0406649bfcb31b33a3ad1b5a8"
        },
        "date": 1643836886166,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16433635,
            "range": "± 988139",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1111122330,
            "range": "± 18611501",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12881803127,
            "range": "± 120843324",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1617156609,
            "range": "± 45885911",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "34268371+sya-ri@users.noreply.github.com",
            "name": "sya-ri",
            "username": "sya-ri"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c8c6be5f594fd55b1655313379025f2e0f0238dd",
          "message": "Change cli to colored output (#46)\n\n* feat: change cli to colored output\r\n\r\n* style: run rustfmt\r\n\r\n* refactor: use crossterm for styling\r\n\r\n* refactor: change variable name\r\n\r\n* style: run rustfmt\r\n\r\n* refactor: Use log crate for output\r\n\r\n* tweak: Remove usage line break\r\n\r\n* tweak: Change usage, version info output to use println\r\n\r\n* refactor: Changed to print icon in log formatter\r\n\r\n* tweak: Add an icon each message log\r\n\r\n* tweak: Improve multi-line message icon\r\n\r\n* feat: Add emoji option\r\n\r\n* tweak: Change warn icon to high_voltage\r\n\r\n* tweak: Change notice icon to right_pointing_magnifying_glass\r\n\r\n* feat: Add color, no-color option\r\n\r\n* tweak: Delete emoji option short name\r\n\r\n* tweak: Treat --emoji as --emoji=true\r\n\r\nUntil now, --emoji was treated as --emoji=default\r\n\r\n* tweak: Change to use color option in emoji parse error\r\n\r\n* style: fmt\r\n\r\n* feat: Add --no-emoji option\r\n\r\n* feat: Support env var for changing output format\r\n\r\n* chore: streamline some changes, minor refactors\r\n\r\n* chore: replace superfluous map_or_else with map_or\r\n\r\n* refactor: move new functions to their own module\r\n\r\nThis is tidier and allows keeping both modules more cohesive.\r\n\r\n* refactor: extract duplicate atty call to a variable\r\n\r\n* chore(terminal_can_display_color): fix incorrect default value\r\n\r\n* chore: revert to env. vars and switches overriding color and emoji\r\n\r\nAn important use case for those switches is being able to print colors\r\nand emojis to non-TTYs, which previously wasn't possible.\r\n\r\n* fix: flush stream to make sure title ANSI escape seqs are handed off\r\n\r\nNot doing so caused the title to not update apparently randomly.\r\n\r\nCo-authored-by: Alejandro González <AlexTMjugador@users.noreply.github.com>",
          "timestamp": "2022-02-03T18:58:12+01:00",
          "tree_id": "09c07e74d3cbfcd740f71eedf2b4290f31997a9d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c8c6be5f594fd55b1655313379025f2e0f0238dd"
        },
        "date": 1643913450725,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16033396,
            "range": "± 697794",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1143617746,
            "range": "± 28602155",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12815266203,
            "range": "± 226444210",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1555304181,
            "range": "± 39964076",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}