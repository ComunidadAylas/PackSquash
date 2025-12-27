# Documentation

This directory contains project documentation.

## Wiki

The `wiki` subdirectory holds the source files for the PackSquash [GitHub Wiki](https://docs.github.com/en/communities/documenting-your-project-with-wikis/about-wikis), powered by a [Gollum](https://github.com/gollum/gollum)-like system.

These files are the source of truth for the wiki content and are kept in the main repository to benefit from the usual pull request and branch-based workflows. The [`wiki-sync`](../.github/workflows/wiki-sync.yml) GitHub Actions workflow automatically synchronizes them to the ancillary `.wiki` repository.
