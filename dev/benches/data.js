window.BENCHMARK_DATA = {
  "lastUpdate": 1658344173212,
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
          "id": "9f81e31d190c4adf764701a9929842f035f1124e",
          "message": "chore: rename `packsquash-cli` package. Stop using unstable Cargo feature\n\nBy defining a custom binary with the desired name instead of modifying\nthe filename of the default binary, which is the same as the name of the\npackage, we can achieve the same result as before, but in a more\nstraightforward way with less configuration.\n\nWhile at it, I've discovered that some Rust RFC recommends for words in\ncrate names to be separated by underscores instead of dashes. This\nconvention is far from being universally observed, and some prominent\ncrates use dashes, but we want to do things right, so let's use\nunderscores.",
          "timestamp": "2022-05-09T22:27:12+02:00",
          "tree_id": "265fc5bae7e09f2eeb5433c2fd1c16ee31346faf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9f81e31d190c4adf764701a9929842f035f1124e"
        },
        "date": 1652131134972,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17840692,
            "range": "± 849822",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 902262656,
            "range": "± 15259720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14295614009,
            "range": "± 50070541",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3237618131,
            "range": "± 21946390",
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
          "id": "fb5012ad72dc36b97b730f65072969a4ab79409e",
          "message": "chore(benches): fix `unused-macro-rules` Clippy lint",
          "timestamp": "2022-05-14T23:08:36+02:00",
          "tree_id": "d2802162b907b8599d8dc4aa4d7fcd99e0133a10",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fb5012ad72dc36b97b730f65072969a4ab79409e"
        },
        "date": 1652565001151,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17715076,
            "range": "± 800298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1045565968,
            "range": "± 9471832",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 17587589430,
            "range": "± 162044748",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2710830802,
            "range": "± 57347161",
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
          "id": "fb5012ad72dc36b97b730f65072969a4ab79409e",
          "message": "chore(benches): fix `unused-macro-rules` Clippy lint",
          "timestamp": "2022-05-14T21:08:36Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fb5012ad72dc36b97b730f65072969a4ab79409e"
        },
        "date": 1652665670330,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16261891,
            "range": "± 975294",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 982895331,
            "range": "± 17048556",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15348234691,
            "range": "± 174722910",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2548526691,
            "range": "± 57945344",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "36238d2413971f9b69bab11898674c1c3a683c30",
          "message": "chore(deps): bump sysinfo from 0.23.11 to 0.23.12 (#112)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.11 to 0.23.12.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T16:41:19+02:00",
          "tree_id": "eb80dbaa79b5fe5756ba62339dd063a25c8175ef",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36238d2413971f9b69bab11898674c1c3a683c30"
        },
        "date": 1652713793311,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17611213,
            "range": "± 1152409",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1004444381,
            "range": "± 6554269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 17288521114,
            "range": "± 68248175",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2603072640,
            "range": "± 98308862",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cf69f68da1176e46f556134f74d5d7885b380cab",
          "message": "chore(deps): bump tokio-util from 0.7.1 to 0.7.2 (#113)\n\nBumps [tokio-util](https://github.com/tokio-rs/tokio) from 0.7.1 to 0.7.2.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-util-0.7.1...tokio-util-0.7.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-util\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T16:40:59+02:00",
          "tree_id": "0def437895572fdc3a87d10e506bf5b6974d6ab3",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cf69f68da1176e46f556134f74d5d7885b380cab"
        },
        "date": 1652714714218,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12481439,
            "range": "± 717241",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 827033288,
            "range": "± 8599020",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15048915207,
            "range": "± 49419269",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2170130816,
            "range": "± 31029527",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "36238d2413971f9b69bab11898674c1c3a683c30",
          "message": "chore(deps): bump sysinfo from 0.23.11 to 0.23.12 (#112)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.11 to 0.23.12.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-16T14:41:19Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/36238d2413971f9b69bab11898674c1c3a683c30"
        },
        "date": 1653269868832,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14471446,
            "range": "± 328183",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 844772431,
            "range": "± 4804601",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11881421318,
            "range": "± 40636371",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2301563746,
            "range": "± 50083010",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "986f98346b8b5e13065aa962e17a3f55d6cec8e8",
          "message": "chore(deps): bump sysinfo from 0.23.12 to 0.23.13 (#115)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.12 to 0.23.13.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T10:55:57+02:00",
          "tree_id": "8b3ae80acebdbcb5b3ffd441aef8ff18c66212c7",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/986f98346b8b5e13065aa962e17a3f55d6cec8e8"
        },
        "date": 1653297759469,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10940065,
            "range": "± 388703",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 798749393,
            "range": "± 3041573",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10286961629,
            "range": "± 232733808",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2263753173,
            "range": "± 37512297",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0f3d5cbf5daba4b511d8faff56c5e527d76a4198",
          "message": "chore(deps): bump regex from 1.5.5 to 1.5.6 (#116)\n\nBumps [regex](https://github.com/rust-lang/regex) from 1.5.5 to 1.5.6.\r\n- [Release notes](https://github.com/rust-lang/regex/releases)\r\n- [Changelog](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/rust-lang/regex/compare/1.5.5...1.5.6)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: regex\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T10:55:05+02:00",
          "tree_id": "fbf0b902cd6fa7df36aee6a218ea5c34fd5c51dd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/0f3d5cbf5daba4b511d8faff56c5e527d76a4198"
        },
        "date": 1653297800266,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15217361,
            "range": "± 800856",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 968973538,
            "range": "± 6640036",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11799692135,
            "range": "± 76415206",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2434862018,
            "range": "± 41204927",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "fccf9271176d14f298d29ca52b50f3e691a9420f",
          "message": "chore(repo): add pull request template",
          "timestamp": "2022-05-24T18:06:47Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/fccf9271176d14f298d29ca52b50f3e691a9420f"
        },
        "date": 1653875133463,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14455898,
            "range": "± 306056",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 843099948,
            "range": "± 3610205",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11813732314,
            "range": "± 49724981",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2304541011,
            "range": "± 27639808",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8fd914740d637a6d70630529d3cb1c067056dc74",
          "message": "chore(deps): bump indexmap from 1.8.1 to 1.8.2 (#117)\n\nBumps [indexmap](https://github.com/bluss/indexmap) from 1.8.1 to 1.8.2.\r\n- [Release notes](https://github.com/bluss/indexmap/releases)\r\n- [Changelog](https://github.com/bluss/indexmap/blob/1.8.2/RELEASES.rst)\r\n- [Commits](https://github.com/bluss/indexmap/compare/1.8.1...1.8.2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: indexmap\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-30T10:34:32+02:00",
          "tree_id": "abac87e5022076ebb1397c48f4d177bb3ae568ec",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/8fd914740d637a6d70630529d3cb1c067056dc74"
        },
        "date": 1653901496611,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14805255,
            "range": "± 802988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 911767694,
            "range": "± 17280590",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11204192028,
            "range": "± 127996581",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2389575506,
            "range": "± 97561393",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9120ccadd5cfd7351aee9ef985a9206314fec5f0",
          "message": "chore(deps): bump uuid from 1.0.0 to 1.1.0 (#118)\n\nBumps [uuid](https://github.com/uuid-rs/uuid) from 1.0.0 to 1.1.0.\r\n- [Release notes](https://github.com/uuid-rs/uuid/releases)\r\n- [Commits](https://github.com/uuid-rs/uuid/compare/1.0.0...1.1.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uuid\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-30T10:35:37+02:00",
          "tree_id": "68842baf953a799fc6134a2a7e4d7f7f29129b51",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9120ccadd5cfd7351aee9ef985a9206314fec5f0"
        },
        "date": 1653902418976,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12521403,
            "range": "± 973027",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 760236228,
            "range": "± 30533013",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10003337162,
            "range": "± 664960343",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2057313084,
            "range": "± 137271511",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alejandro González",
            "username": "AlexTMjugador",
            "email": "AlexTMjugador@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "25ae2b83d93e64feb2f32880daf10a9e263da724",
          "message": "chore(ISSUE_TEMPLATE/feature_request): fix typo",
          "timestamp": "2022-05-30T14:36:45Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/25ae2b83d93e64feb2f32880daf10a9e263da724"
        },
        "date": 1654479911531,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15163860,
            "range": "± 372715",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 981670515,
            "range": "± 11747361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16876762899,
            "range": "± 129012265",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2588930372,
            "range": "± 34624492",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c0bd5e0b4fb01399b8ba7bfc7a506d2af1a3463d",
          "message": "chore(deps): bump uuid from 1.1.0 to 1.1.1 (#124)\n\nBumps [uuid](https://github.com/uuid-rs/uuid) from 1.1.0 to 1.1.1.\r\n- [Release notes](https://github.com/uuid-rs/uuid/releases)\r\n- [Commits](https://github.com/uuid-rs/uuid/compare/1.1.0...1.1.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: uuid\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-06T12:21:25+02:00",
          "tree_id": "a61be1b1399ec40d06f7f124c25ecec7cf26dd20",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c0bd5e0b4fb01399b8ba7bfc7a506d2af1a3463d"
        },
        "date": 1654512551480,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13328465,
            "range": "± 812251",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 806033084,
            "range": "± 40168877",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13805909009,
            "range": "± 770683126",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2205190681,
            "range": "± 116060629",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9365909009e6fd59913dc873bc3252342c956d9f",
          "message": "chore(deps): bump sysinfo from 0.23.13 to 0.24.1 (#123)\n\nBumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.23.13 to 0.24.1.\r\n- [Release notes](https://github.com/GuillaumeGomez/sysinfo/releases)\r\n- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: sysinfo\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-06T12:22:44+02:00",
          "tree_id": "e660cdf5b8ba24e336e5abfada25b07e7ab43690",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9365909009e6fd59913dc873bc3252342c956d9f"
        },
        "date": 1654512575105,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13304624,
            "range": "± 620040",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 873010192,
            "range": "± 14050270",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13565321793,
            "range": "± 117685189",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2236418669,
            "range": "± 24877859",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad0be826ed42dfa93a105c78359838c463455e0c",
          "message": "chore(deps): bump tokio from 1.18.2 to 1.19.1 (#122)\n\nBumps [tokio](https://github.com/tokio-rs/tokio) from 1.18.2 to 1.19.1.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.18.2...tokio-1.19.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T10:56:13+02:00",
          "tree_id": "7a2bfad7b0e65db724aebb462e29fc40276b7927",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/ad0be826ed42dfa93a105c78359838c463455e0c"
        },
        "date": 1654594563487,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17074707,
            "range": "± 556218",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 995557101,
            "range": "± 7442422",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16615426324,
            "range": "± 217933305",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2598312540,
            "range": "± 39601814",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9c6fc71fe22f88f004655314498c08acddce9fda",
          "message": "chore(deps): bump tokio-util from 0.7.2 to 0.7.3 (#120)\n\nBumps [tokio-util](https://github.com/tokio-rs/tokio) from 0.7.2 to 0.7.3.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-util-0.7.2...tokio-util-0.7.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-util\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T12:23:01+02:00",
          "tree_id": "41cef1225fdfe9cbc94c8dfede10d86753101d0a",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9c6fc71fe22f88f004655314498c08acddce9fda"
        },
        "date": 1654599120142,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17544001,
            "range": "± 822853",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1006453858,
            "range": "± 7487041",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16905449952,
            "range": "± 139663110",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671668402,
            "range": "± 27339099",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42495828bb9197e9a9e26f406f7b1f129bc043e0",
          "message": "chore(deps): bump tokio-stream from 0.1.8 to 0.1.9 (#121)\n\nBumps [tokio-stream](https://github.com/tokio-rs/tokio) from 0.1.8 to 0.1.9.\r\n- [Release notes](https://github.com/tokio-rs/tokio/releases)\r\n- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-stream-0.1.8...tokio-stream-0.1.9)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-stream\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-07T12:29:12+02:00",
          "tree_id": "feb5c21763f6cfcf1d5c295c3a489cf5184ed711",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/42495828bb9197e9a9e26f406f7b1f129bc043e0"
        },
        "date": 1654599388894,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14113513,
            "range": "± 924629",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 841171237,
            "range": "± 3242152",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14165729159,
            "range": "± 77986080",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2190069551,
            "range": "± 54801905",
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
            "name": "AlexTMjugador",
            "username": "AlexTMjugador"
          },
          "distinct": true,
          "id": "f68079c29bf4da75272c7ab7ff4b696c0c542491",
          "message": "chore(ci): refactor Docker workflow job",
          "timestamp": "2022-06-08T19:08:20+02:00",
          "tree_id": "be5ff8313066e878cb1f06ec41401f2b39f30087",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f68079c29bf4da75272c7ab7ff4b696c0c542491"
        },
        "date": 1654710576813,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14720317,
            "range": "± 707298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 958389917,
            "range": "± 10121423",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15179464379,
            "range": "± 61556939",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2450483515,
            "range": "± 78103758",
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
          "id": "46e7f522f37dd2cedf11a7811520cf7b06bf20a3",
          "message": "chore(ci): add Dockerfile to included push path for build workflow",
          "timestamp": "2022-06-08T19:18:25+02:00",
          "tree_id": "2f400a6431014f4fc19191213edd25f86f8d69c4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46e7f522f37dd2cedf11a7811520cf7b06bf20a3"
        },
        "date": 1654710798458,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11253379,
            "range": "± 259338",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 793473722,
            "range": "± 5499109",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12799262187,
            "range": "± 59280554",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2267196023,
            "range": "± 51756552",
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
          "id": "a0f0c195d132a1d60f0c23c0f7bde8abb6117a9e",
          "message": "chore(ci/docker): rename job and fix typo",
          "timestamp": "2022-06-08T19:59:33+02:00",
          "tree_id": "773c465e930e100e1e48f5a91bf1a88c9404ce38",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/a0f0c195d132a1d60f0c23c0f7bde8abb6117a9e"
        },
        "date": 1654712849749,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12570106,
            "range": "± 3453177",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 754849043,
            "range": "± 2697922",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13264407805,
            "range": "± 65151037",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2053127914,
            "range": "± 47030235",
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
          "id": "21dd13854b6df0699dc078c59ce78fa247536303",
          "message": "chore(ci/docker): fix GitHub expression not being evaluated to YAML bool",
          "timestamp": "2022-06-08T20:31:31+02:00",
          "tree_id": "8615690f17b4ed371bf8e9bab456a1fb86ba7f97",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/21dd13854b6df0699dc078c59ce78fa247536303"
        },
        "date": 1654714953333,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 17557390,
            "range": "± 1026165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1024867577,
            "range": "± 13249931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 18053649048,
            "range": "± 103167904",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2838314884,
            "range": "± 119759978",
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
          "id": "b4ac04d1b1d7667bfa4a8618d7652d9c2414d4c9",
          "message": "chore(deps): update dependencies",
          "timestamp": "2022-06-08T22:36:06+02:00",
          "tree_id": "a723a3937cd2826c2cace6c61952b68895c1c9e5",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b4ac04d1b1d7667bfa4a8618d7652d9c2414d4c9"
        },
        "date": 1654722371946,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15688201,
            "range": "± 1141729",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 991494167,
            "range": "± 22183057",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16740331682,
            "range": "± 178893339",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2648813890,
            "range": "± 63952792",
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
          "id": "7cc3d3675fbc6ea5fbe708d7557855aba614c3db",
          "message": "chore(ci/docker): tweak comment and add proper description label",
          "timestamp": "2022-06-08T22:47:11+02:00",
          "tree_id": "3e598535644ff1708593029f66e16ca436b82c6f",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7cc3d3675fbc6ea5fbe708d7557855aba614c3db"
        },
        "date": 1654723052153,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15909948,
            "range": "± 758191",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 997827211,
            "range": "± 13104958",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15469189839,
            "range": "± 203309023",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671755276,
            "range": "± 78327309",
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
          "id": "404d88bacb2d21ecc84a0c7d032ff49ff033bb97",
          "message": "chore(static_analysis): remove withdrawn advisory from deny.toml",
          "timestamp": "2022-06-08T22:49:03+02:00",
          "tree_id": "4eb133ae4652c70db8a8198bc1dcb14e17b6fe96",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/404d88bacb2d21ecc84a0c7d032ff49ff033bb97"
        },
        "date": 1654723932190,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16810987,
            "range": "± 834016",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1039286659,
            "range": "± 32973517",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 16323709976,
            "range": "± 314867821",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2636259248,
            "range": "± 81254762",
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
          "id": "46f5772d58963dcfd69ddab3de8213bd3295a942",
          "message": "fix(pack_file/asset_type): legacy font .bin files can be anywhere\n\nThe Minecraft wiki incorrectly pointed out that .bin files belong to\nassets/<namespace>/font, but the game interprets the resource location\nin an absolute context, so valid resource locations for those files can\npoint anywhere in the pack. Indeed, a PackSquash user reported dealing\nwith a pack that had a .bin file outside of the usual folder, which\nPackSquash broke due to me coding a too restrictive glob pattern based\non the wiki misinformation.\n\nMake the .bin glob pattern more permissive to fix the situation.",
          "timestamp": "2022-06-08T23:13:20+02:00",
          "tree_id": "14a5b1d3fc161ec5ea77fa354ad8f65e5c1934e9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46f5772d58963dcfd69ddab3de8213bd3295a942"
        },
        "date": 1654725717322,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13000168,
            "range": "± 604687",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 826767095,
            "range": "± 1712070",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13995786116,
            "range": "± 49827923",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2214103027,
            "range": "± 60926880",
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
          "id": "46f5772d58963dcfd69ddab3de8213bd3295a942",
          "message": "fix(pack_file/asset_type): legacy font .bin files can be anywhere\n\nThe Minecraft wiki incorrectly pointed out that .bin files belong to\nassets/<namespace>/font, but the game interprets the resource location\nin an absolute context, so valid resource locations for those files can\npoint anywhere in the pack. Indeed, a PackSquash user reported dealing\nwith a pack that had a .bin file outside of the usual folder, which\nPackSquash broke due to me coding a too restrictive glob pattern based\non the wiki misinformation.\n\nMake the .bin glob pattern more permissive to fix the situation.",
          "timestamp": "2022-06-08T21:13:20Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/46f5772d58963dcfd69ddab3de8213bd3295a942"
        },
        "date": 1655085841218,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14830139,
            "range": "± 993401",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 978210971,
            "range": "± 9348325",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15308909972,
            "range": "± 82251273",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2491328574,
            "range": "± 68808633",
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
          "id": "e27ebb38979a0aa82ca5eae83c13e54f1cef59ba",
          "message": "chore(deps): update globset, sysinfo and uuid to their latest versions",
          "timestamp": "2022-06-13T14:00:09+02:00",
          "tree_id": "a6cdff0f19948952e41a3ec7b6d06ab18432cebf",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/e27ebb38979a0aa82ca5eae83c13e54f1cef59ba"
        },
        "date": 1655124347134,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10997347,
            "range": "± 317731",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 790386573,
            "range": "± 1901595",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12763358946,
            "range": "± 48940367",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2255457640,
            "range": "± 74777213",
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
          "id": "cb80c9745398d79ce0a9f3a37bf5231de1540daf",
          "message": "chore(deps): update sysinfo and indexmap dependencies\n\nThe locked transitive dependencies and cargo deny configuration was also\nupdated.",
          "timestamp": "2022-06-17T14:55:37+02:00",
          "tree_id": "54d9ffedbecfad1876be1386d42f819b1645aabc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/cb80c9745398d79ce0a9f3a37bf5231de1540daf"
        },
        "date": 1655473949504,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14101526,
            "range": "± 729030",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 762389944,
            "range": "± 7702052",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15590937195,
            "range": "± 34272988",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2284793740,
            "range": "± 59136438",
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
          "id": "25e5153ae89082c9aa4c41f094fae39506f09bfe",
          "message": "chore(deps): remove transitive dependency on chrono\n\nchrono is a worrying crate to depend on, because it depends on old\nversions of the time crate with known security problems. PackSquash is\nnot affected by these, and overall upstream does not consider the issue\nvery important, but as GitHub advisories are rolled out for it, silecing\nall the security warnings in this convoluted transitive dependency mess\ngets cumbersome quickly.\n\nLuckily, chrono is only used by wmi-rs on Windows platforms. The rest of\nthe ecosystem has been moving away from it, and wmi-rs now has a feature\nflag to directly use the time crate instead. Let's enable it to pull\nless transitive dependencies and properly fix all these warnings.",
          "timestamp": "2022-06-17T15:02:47+02:00",
          "tree_id": "1f1fc528e7501405ee7334f4b078784d666c5661",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/25e5153ae89082c9aa4c41f094fae39506f09bfe"
        },
        "date": 1655474502528,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11062780,
            "range": "± 614810",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 718878236,
            "range": "± 8208731",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13454882747,
            "range": "± 32847793",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2206662268,
            "range": "± 28992872",
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
          "id": "b9246d55910c4ed1c4d85b5e270ffa35f73d76be",
          "message": "chore: update dependencies and apply recent once_cell feature renamings\n\nhttps://github.com/rust-lang/rust/pull/98165 landed in the latest Rust\nnightly version, which made Dependabot CI runs to fail. Try to get\nbuilds working again by updating the references to the old names\naccordingly.\n\nWhile at it, let's also do some minor dependency version bumps.",
          "timestamp": "2022-06-20T18:43:43+02:00",
          "tree_id": "6776a382c80ef7309319d6474a4314baa9b0eca0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b9246d55910c4ed1c4d85b5e270ffa35f73d76be"
        },
        "date": 1655747253760,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 15633822,
            "range": "± 1281737",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 886748961,
            "range": "± 7001939",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15938590979,
            "range": "± 70397125",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2446273787,
            "range": "± 23345565",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "14e28858ba458d189cbf1fc7da6e9207cf9d45c7",
          "message": "chore(deps): bump indexmap from 1.9.0 to 1.9.1 (#131)\n\nBumps [indexmap](https://github.com/bluss/indexmap) from 1.9.0 to 1.9.1.\r\n- [Release notes](https://github.com/bluss/indexmap/releases)\r\n- [Changelog](https://github.com/bluss/indexmap/blob/master/RELEASES.md)\r\n- [Commits](https://github.com/bluss/indexmap/compare/1.9.0...1.9.1)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: indexmap\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-06-27T10:55:51+02:00",
          "tree_id": "e431e337c80a4e3973bcdd5b07623ec982933769",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/14e28858ba458d189cbf1fc7da6e9207cf9d45c7"
        },
        "date": 1656323335986,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12748099,
            "range": "± 197236",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 747754033,
            "range": "± 9361115",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14594034391,
            "range": "± 45430400",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2140290088,
            "range": "± 46817803",
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
          "id": "c86bd6f326cc36e3a9b2f50a7291858e3fc931e0",
          "message": "chore(clippy): fix new lints",
          "timestamp": "2022-07-04T12:27:22+02:00",
          "tree_id": "816d3633d6803e14b060aa836a1a8cbf5e01b2fd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c86bd6f326cc36e3a9b2f50a7291858e3fc931e0"
        },
        "date": 1656932299298,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13888373,
            "range": "± 903638",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 838926684,
            "range": "± 8162782",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14838717862,
            "range": "± 245461288",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2460315406,
            "range": "± 30604162",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7d103b3543ec551ed94d052aa392654072838cab",
          "message": "chore(deps): bump serde_json from 1.0.81 to 1.0.82 (#134)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.81 to 1.0.82.\r\n- [Release notes](https://github.com/serde-rs/json/releases)\r\n- [Commits](https://github.com/serde-rs/json/compare/v1.0.81...v1.0.82)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde_json\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-07-04T12:29:52+02:00",
          "tree_id": "150a3bc7e5003617cbfcc10556c31e045a5f45c9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/7d103b3543ec551ed94d052aa392654072838cab"
        },
        "date": 1656932466536,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10612651,
            "range": "± 238520",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 705941933,
            "range": "± 8594249",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13234024783,
            "range": "± 52228858",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2198511841,
            "range": "± 5821158",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6f40332118196bbc68074f199121526dd8a013b7",
          "message": "chore(deps): bump serde from 1.0.137 to 1.0.138 (#133)\n\nBumps [serde](https://github.com/serde-rs/serde) from 1.0.137 to 1.0.138.\r\n- [Release notes](https://github.com/serde-rs/serde/releases)\r\n- [Commits](https://github.com/serde-rs/serde/compare/v1.0.137...v1.0.138)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: serde\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-07-04T13:07:20+02:00",
          "tree_id": "f2d4a477070632a76cd832edb149de71bf6889f0",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/6f40332118196bbc68074f199121526dd8a013b7"
        },
        "date": 1656934640506,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10827526,
            "range": "± 93325",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 710518177,
            "range": "± 4099866",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13427591937,
            "range": "± 42376720",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2217209235,
            "range": "± 51420320",
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
          "id": "d24646b000be731a71cbf72906369d01592cd7b4",
          "message": "chore(refactor): drop `num_cpus` dep in favor of stdlib function\n\nThis function was recently stabilized and is improving in recent Rust\nversions. At this point, it is equivalent to num_cpus for most purposes,\nbut without the added burden of another dependency.",
          "timestamp": "2022-07-06T12:35:50+02:00",
          "tree_id": "9d242497a02564dc660d18cb57f9b94a77209bcc",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d24646b000be731a71cbf72906369d01592cd7b4"
        },
        "date": 1657105820145,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11143328,
            "range": "± 170506",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 740189533,
            "range": "± 37542103",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14225243718,
            "range": "± 873327613",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 1957409775,
            "range": "± 14041179",
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
          "id": "d24646b000be731a71cbf72906369d01592cd7b4",
          "message": "chore(refactor): drop `num_cpus` dep in favor of stdlib function\n\nThis function was recently stabilized and is improving in recent Rust\nversions. At this point, it is equivalent to num_cpus for most purposes,\nbut without the added burden of another dependency.",
          "timestamp": "2022-07-06T10:34:26Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d24646b000be731a71cbf72906369d01592cd7b4"
        },
        "date": 1657504167847,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 13873436,
            "range": "± 456145",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 762046505,
            "range": "± 7147797",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 14974865304,
            "range": "± 51927188",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2213895518,
            "range": "± 15930971",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9920791d61400a538c542c16e3db10d2eac5d2d6",
          "message": "chore: add GitHub Codespaces dev container configuration\n\nGitHub Codespaces is very useful for those that have access to it, as it\noffers a web VS Code IDE backed by a powerful Linux VM with terminal\naccess, where development can be done in a breeze, without any local\nconfiguration.\n\nThis makes it easier for new contributors to get started, and serves as\na backup development machine for maintainers.",
          "timestamp": "2022-07-14T12:08:07Z",
          "tree_id": "9fa8aa2e954d9e172253d83f9d0c6cd4333d0185",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9920791d61400a538c542c16e3db10d2eac5d2d6"
        },
        "date": 1657803759901,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14464639,
            "range": "± 413665",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 752962250,
            "range": "± 4667830",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12064678280,
            "range": "± 45026519",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2671662147,
            "range": "± 8434989",
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
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "9920791d61400a538c542c16e3db10d2eac5d2d6",
          "message": "chore: add GitHub Codespaces dev container configuration\n\nGitHub Codespaces is very useful for those that have access to it, as it\noffers a web VS Code IDE backed by a powerful Linux VM with terminal\naccess, where development can be done in a breeze, without any local\nconfiguration.\n\nThis makes it easier for new contributors to get started, and serves as\na backup development machine for maintainers.",
          "timestamp": "2022-07-14T12:08:07Z",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9920791d61400a538c542c16e3db10d2eac5d2d6"
        },
        "date": 1658108806481,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 16399772,
            "range": "± 940146",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 870058745,
            "range": "± 18532867",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 13966640158,
            "range": "± 176758141",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 3077913858,
            "range": "± 52571645",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "87e6cba08f2c80a4f94bd296fec3d5dfecd044d9",
          "message": "chore(deps): bump wmi from 0.9.3 to 0.11.0 (#140)\n\nBumps [wmi](https://github.com/ohadravid/wmi-rs) from 0.9.3 to 0.11.0.\r\n- [Release notes](https://github.com/ohadravid/wmi-rs/releases)\r\n- [Commits](https://github.com/ohadravid/wmi-rs/compare/v0.9.3...v0.11.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: wmi\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-07-18T14:53:57+02:00",
          "tree_id": "94945168f79e89d32db911da47e96e3089c71dc4",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/87e6cba08f2c80a4f94bd296fec3d5dfecd044d9"
        },
        "date": 1658151640246,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14147813,
            "range": "± 274978",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 747389430,
            "range": "± 3355746",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12516171656,
            "range": "± 31358120",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2743646473,
            "range": "± 15001870",
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
          "id": "39d9e443f61184cda7710c2acf289718b97d8bf0",
          "message": "chore(clippy): fix some new lints",
          "timestamp": "2022-07-20T15:08:16Z",
          "tree_id": "76c41f08f56a5f0a86d662b8a6d86558dc69420e",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/39d9e443f61184cda7710c2acf289718b97d8bf0"
        },
        "date": 1658331295361,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10892037,
            "range": "± 152232",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 779008900,
            "range": "± 8391667",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9909967842,
            "range": "± 206679945",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2164668834,
            "range": "± 15792337",
            "unit": "ns/iter"
          }
        ]
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
          "id": "f4da3965fa4403110dcde7835ed50b08c60b002e",
          "message": "chore(deps): update dependency urllib3 to v1.26.10",
          "timestamp": "2022-07-20T15:10:10Z",
          "tree_id": "4a52b22abdbe3ecfb82a586796fd265ecc5d66bd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f4da3965fa4403110dcde7835ed50b08c60b002e"
        },
        "date": 1658331380750,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11202571,
            "range": "± 490373",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 795511352,
            "range": "± 4641332",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9950339481,
            "range": "± 40682475",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2178910183,
            "range": "± 21442774",
            "unit": "ns/iter"
          }
        ]
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
          "id": "d774141794bf17e0429bc5f45d6f53c37c0b6104",
          "message": "fix(deps): update rust crate sysinfo to 0.24.7",
          "timestamp": "2022-07-20T15:12:44Z",
          "tree_id": "afc51cf6cd2d70a7053d804e894a923a43453b6d",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/d774141794bf17e0429bc5f45d6f53c37c0b6104"
        },
        "date": 1658331988566,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12010630,
            "range": "± 590029",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 812223339,
            "range": "± 4506361",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9968752444,
            "range": "± 17276536",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2227973457,
            "range": "± 18993268",
            "unit": "ns/iter"
          }
        ]
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
          "id": "c24c6fbc653fa3f20e850da696f353864e7e55be",
          "message": "fix(deps): update rust crate imagequant to 4.0.1",
          "timestamp": "2022-07-20T15:11:47Z",
          "tree_id": "860f4493360ca3f6f0eebff086f1792a9e612bff",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/c24c6fbc653fa3f20e850da696f353864e7e55be"
        },
        "date": 1658332505573,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14424512,
            "range": "± 510503",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 860471003,
            "range": "± 6366556",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12515068617,
            "range": "± 29647509",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2277420932,
            "range": "± 28583440",
            "unit": "ns/iter"
          }
        ]
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
          "id": "12d18d15b86c94d4f3d38a17f1b44af0462737d8",
          "message": "chore(deps): update dependency soupsieve to v2.3.2.post1",
          "timestamp": "2022-07-20T15:10:05Z",
          "tree_id": "7c8c064ca0bf4c210a557563228d538aadb4d228",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/12d18d15b86c94d4f3d38a17f1b44af0462737d8"
        },
        "date": 1658332537947,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11045126,
            "range": "± 162142",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 781974947,
            "range": "± 3128931",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9921642208,
            "range": "± 215096979",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2164429393,
            "range": "± 16733019",
            "unit": "ns/iter"
          }
        ]
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
          "id": "9d6f7e9508aa04127cb1471da909f2741a354d88",
          "message": "chore(deps): update dependency beautifulsoup4 to v4.11.1",
          "timestamp": "2022-07-20T18:09:20Z",
          "tree_id": "dd73d24cbd9484d38ae428c2c5884d7b248cf994",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/9d6f7e9508aa04127cb1471da909f2741a354d88"
        },
        "date": 1658343086771,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 12434510,
            "range": "± 175138",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 830380234,
            "range": "± 10031469",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 12177037067,
            "range": "± 258731410",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2320450175,
            "range": "± 30022155",
            "unit": "ns/iter"
          }
        ]
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
          "id": "b5c819ec5919ffa44e8aabf273d8d5d6cf0a2fa5",
          "message": "chore(deps): update dependency urllib3 to v1.26.10 (#144)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2022-07-20T20:24:30+02:00",
          "tree_id": "4a52b22abdbe3ecfb82a586796fd265ecc5d66bd",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/b5c819ec5919ffa44e8aabf273d8d5d6cf0a2fa5"
        },
        "date": 1658344039672,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11135677,
            "range": "± 415137",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 799731327,
            "range": "± 3931763",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9927320848,
            "range": "± 342364121",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2170052650,
            "range": "± 9090320",
            "unit": "ns/iter"
          }
        ]
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
          "id": "1194fa292f7b7433a02462f6c5d987a24b24612f",
          "message": "fix(deps): update rust crate imagequant to 4.0.1",
          "timestamp": "2022-07-20T18:30:05Z",
          "tree_id": "71025627251297dbc8f3157efbe84c662a36b851",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/1194fa292f7b7433a02462f6c5d987a24b24612f"
        },
        "date": 1658344108978,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 11373218,
            "range": "± 299165",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 800252989,
            "range": "± 6453225",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 10111702377,
            "range": "± 215333648",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2223918520,
            "range": "± 26321394",
            "unit": "ns/iter"
          }
        ]
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
          "id": "4420feb345ff15c21d3403761a81aee2bf2f5855",
          "message": "chore(deps): update dependency soupsieve to v2.3.2.post1 (#143)\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2022-07-20T20:24:46+02:00",
          "tree_id": "2810bb6dbc647fb30f22565c850403a61943fb42",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/4420feb345ff15c21d3403761a81aee2bf2f5855"
        },
        "date": 1658344124947,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 14587241,
            "range": "± 731942",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 914325714,
            "range": "± 11506162",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 11164147914,
            "range": "± 91383542",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2363003720,
            "range": "± 94857423",
            "unit": "ns/iter"
          }
        ]
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
          "id": "204a0ddb4185b768a3bd5b308bc25d9f18ef263a",
          "message": "fix(deps): update rust crate imagequant to 4.0.1",
          "timestamp": "2022-07-20T18:26:07Z",
          "tree_id": "450e5257263aa51c4203b17d8c2710c25621adb9",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/204a0ddb4185b768a3bd5b308bc25d9f18ef263a"
        },
        "date": 1658344160622,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 10934692,
            "range": "± 286298",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 778838273,
            "range": "± 2479161",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 9931494606,
            "range": "± 22907099",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2169678368,
            "range": "± 39122158",
            "unit": "ns/iter"
          }
        ]
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
          "id": "f274b4cdb007a13dedcc214c147b605449bc92c5",
          "message": "fix(deps): update rust crate serde to 1.0.140",
          "timestamp": "2022-07-20T18:09:13Z",
          "tree_id": "7a65a0b8036d63d23751e6b51b4c7f584068046b",
          "url": "https://github.com/ComunidadAylas/PackSquash/commit/f274b4cdb007a13dedcc214c147b605449bc92c5"
        },
        "date": 1658344172135,
        "tool": "cargo",
        "benches": [
          {
            "name": "tiny_benches_wall_time/empty_pack",
            "value": 18228735,
            "range": "± 954747",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aylas_khron_micro_pack",
            "value": 1041455381,
            "range": "± 19219973",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/jilchu_chronos_micro_pack",
            "value": 15174985959,
            "range": "± 273788993",
            "unit": "ns/iter"
          },
          {
            "name": "small_benches_wall_time/aiamded_breadstick_micro_pack",
            "value": 2731752294,
            "range": "± 114976605",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}