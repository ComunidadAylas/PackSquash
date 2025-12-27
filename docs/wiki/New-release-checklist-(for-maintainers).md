Authorized maintainers must complete the following steps when making a new release. Horizontal lines separate blocks of tasks that must be completed entirely before the next block.

- [ ] Ensure that the version information in every `Cargo.toml` file is correct. For example, when publishing v0.3.1, the version of every PackSquash package must be v0.3.1.
- [ ] Make sure that the Debian package has the appropriate contents and metadata. Pay special attention to its version and dependencies.
- [ ] Write the changelog and release notes in a draft GitHub release.
- [ ] Write the necessary wiki changes to document the changes in the new release. Don't publish them yet.
- [ ] Thoroughly test the new release with real packs in the game.
- [ ] Do any necessary preparations to the GitHub action and/or its binary manifests.
***
- [ ] Make sure that the mainline passes every CI check and builds correctly shortly before publishing the release.
- [ ] Publish the draft release.
- [ ] Publish the wiki changes. Where applicable, remember to add a link to the older version of the documentation, for the older release.
- [ ] When the release CI build completes, check that the embedded version information in the executable is correct (run `packsquash --version`). If it is not correct, remove the release and its tag, fix the cause ASAP, and recreate the release.
***
- [ ] Ensure that the release event CI workflow attached the artifacts it generated to the release.
- [ ] Update the [APT repository update workflow file](https://github.com/ComunidadAylas/PackSquash-apt-repo/blob/master/.github/workflows/update_apt_repository.yml) to point to the new release.
- [ ] Make sure that the `GH_API_TOKEN` secret in the APT repository is set to a
  PAT with push access to the repository and that the organization allows the
  usage of such tokens.
- [ ] [Manually run the APT repository update workflow](https://github.com/ComunidadAylas/PackSquash-apt-repo/blob/master/.github/workflows/update_apt_repository.yml) to update the APT repository.
- [ ] Release the necessary GitHub action update and/or changes to its binary manifests.
***
- [ ] Announce the update on the Discord PackSquash updates channel.
- [ ] Announce the update on PlanetMinecraft.
- [ ] Announce the update to other stakeholders: Quiver, interested users, etc.
***
- [ ] Bump the versions in the source code to the next release.