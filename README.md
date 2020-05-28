# PackSquash
A fast Minecraft resource pack lossy compressor. It is able to reduce resource pack ZIP file sizes up to 7 times, which allows for efficient distribution and slightly improved game performance.

## Potentials for improvement
These are some tweaks to the application which could further improve the compression it achieves, but are not scheduled for a release. Feel free to submit a PR with some (or all) of these changes!

* Parse JSON files to determine unused assets (models, textures, sounds...), and skip them from the result ZIP file
* Understand the TTF and OGG formats more, to remove metadata and further compress them
* Implement an "append mode", which only adds to a result ZIP file resource pack files which are newer than it (so the entire ZIP file isn't generated again if a single file changes)
