PackSquash strives to optimize your packs substantially, but it can't make packs
for you. It's your responsibility to use it with packs that work with Minecraft
in the first place, and also make them _correct_ according to what you want to
achieve, as PackSquash can't read your mind (yet?).

To assist you in that endeavor, this page contains helpful tidbits of
information that are meant to improve your already existing pack authoring
workflows and, in general, help you get the most out of your resources, your
ideas, Minecraft and PackSquash.

## Table of contents

- [💡 Educate yourself about how packs
  work](#-educate-yourself-about-how-packs-work)
- [🗒️ Keep an eye on the Minecraft logs](#️-keep-an-eye-on-the-minecraft-logs)
- [🗂️ Use a version control system, even if you work
  alone](#️-use-a-version-control-system-even-if-you-work-alone)
- [🌎 If you need to add or replace language strings for every Minecraft
  language, use a script like this
  one](#-if-you-need-to-add-or-replace-language-strings-for-every-minecraft-language-use-a-script-like-this-one)
- [🔁 Big item model files due to lots of overrides? Use this `jq` filter to
  merge smaller override files into an item model
  file!](#-big-item-model-files-due-to-lots-of-overrides-use-this-jq-filter-to-merge-smaller-override-files-into-an-item-model-file)

## 💡 Educate yourself about how packs work

If you were to carefully analyze many packs distributed in the wild as the
PackSquash authors do, you would find out that many of them have a considerable
number of non-conformances and errors with respect to how Minecraft works and
file format standards. Some of these non-conformances are likely intentional, as
they seem to be the result of a conscious, thought-out decision. However, many
of them are not, and there are objective reasons why they might cause problems
in the future or be suboptimal in several aspects, including ease of change and
game performance. It is not rare for first-time PackSquash users to have to
tweak their pack a bit to fix some of its most egregious errors.

A good way to improve on this situation is to ask yourself the following
questions while you are editing a pack file and try to answer them thoroughly in
your head with facts:

- What can I do with this file?
- What does this file do? What is this file meant to do?
- Why does this work?
- In what format should I write or do something?
- What are the implications of choosing one way of doing something with this
  file over other ways?

It is useful to approach the whole pack authoring process with [critical
thinking](https://en.wikipedia.org/wiki/Critical_thinking). Avoid being
_spoon-fed_ concrete solutions to what you want to achieve without understanding
them, as understanding is eventually required to move forward in an efficient
and satisfactory way.

A good starting point to know more about Minecraft packs is the [Minecraft
Wiki](https://minecraft.wiki/w/Minecraft_Wiki) and its articles about [resource
packs](https://minecraft.wiki/w/Resource_Pack) and [data
packs](https://minecraft.wiki/w/Data_pack).

## 🗒️ Keep an eye on the Minecraft logs

The Minecraft client sends a bunch of useful information to its logs, including
warnings and errors about packs. Keeping an eye on such logs is extremely
helpful to know why some pack does not work fine with Minecraft or how a pack
can be improved, even if it seems to work fine.

Remarkably, you should not only bother to fix the pack errors that the console
shows until it seems to work: warnings also highlight matters that deserve your
attention, like problems you have overlooked or conditions that may negatively
impact your pack or your workflow sooner or later. Also, if you let the number
of warnings or errors that your pack generates grow out of control, it will be
harder for you to notice any new potential problems, and the game will run
slower due to it having to write all those extra messages.

You can watch the Minecraft logs by opening the game output window as explained
[here](https://minecrafthopper.net/help/guides/getting-minecraft-game-output-log/),
or reading the `latest.log` file in the `logs` subfolder at the
[`.minecraft`](https://minecraft.wiki/w/.minecraft) folder.

## 🗂️ Use a version control system, even if you work alone

[Version control
systems](https://betterexplained.com/articles/a-visual-guide-to-version-control/)
(VCS) are prevalent in the software engineering industry, in part due to the
complex nature of computer code and the need for automation in software
configuration management processes. However, such systems are also used in less
technical applications, like cloud office suites (think in Google Docs), wikis,
and blogs.

Namely, using a version control system provides benefits for pack authoring such
as the following:

- More efficient collaboration with other people.
- Ability to track what change caused a problem or effect in your pack.
- Implicit backups: if you do something that breaks your pack, a VCS easily
  allows restoring a previous known-good version. Also, in the most common VCS
  deployments, the pack data is stored in several places and over different
  computers.
- They associate textual descriptions to changesets, making it easier to dig
  into the rationale and thought process that motivated a certain change.
- VCSs are at the heart of other productivity software. For example, by using
  Git and GitHub, you can take advantage of GitHub Actions workflows to
  automatically optimize your pack on the cloud via the [PackSquash GitHub
  action](https://github.com/ComunidadAylas/PackSquash-action).
- They streamline processes that otherwise you would have to do manually,
  arguably in a worse, less consistent, and less tidy way.

In fact, due to these benefits, knowledgeable, professional, and/or
productivity-focused pack authors tend to use version control systems. They are
also widely used in the game industry, with engines such as
[Unity](https://docs.unity3d.com/Manual/VersionControl.html) or
[Unreal](https://docs.unrealengine.com/4.27/en-US/Basics/SourceControl/) having
built-in support for them.

## 🌎 If you need to add or replace language strings for every Minecraft language, use a script like this one

It is common for pack authors to want to add or replace a language string in a
resource pack, no matter what language the player has selected in the Minecraft
options. However, language strings are added or replaced per language, and each
language has its own file, named after its language code. To achieve this goal,
pack authors manually copy the same language file with different names, so the
language strings change for every language. This is cumbersome, prone to errors,
and susceptible to the pack author being satisfied with partial results ("who is
going to select this language, anyway?"). In the end, this way to achieve that
goal results in a suboptimal experience for both players and pack authors.

[There are posts in the Minecraft feedback
website](https://feedback.minecraft.net/hc/en-us/community/posts/360072066772-Resource-pack-A-way-to-override-all-languages)
suggesting better support for this pattern, but it does not seem likely that
such an improvement will be implemented soon. In the meantime, to save you from
this tedious work, you can resort to automation.

Over Discord, [@Derkades](https://github.com/Derkades) shared this Python script
you can use to make all the needed copies of a language file, so its changes are
applied to every language the player can select in the Minecraft options. It
gets the list of copies to make for a given Minecraft version by looking at its
assets, so it will work well in past and future Minecraft versions.

```python
#!/usr/bin/env python3
import json
import shutil

# Change these four variables accordingly
DOTMINECRAFT = '/home/robin/.minecraft'
VERSION = '1.17'
INPUT_FILE = 'lang.json'
OUTPUT_DIR = 'out/'

language_codes = []

with open(f'{DOTMINECRAFT}/assets/indexes/{VERSION}.json') as f:
    indexes: dict = json.load(f)
    keys: [str] = indexes['objects'].keys()
    for key in keys:
        if key.startswith('minecraft/lang/'):
            # remove minecraft/lang/ and .json
            language_codes.append(key[15:-5])

print(language_codes)

for code in language_codes:
    shutil.copy(INPUT_FILE, f'{OUTPUT_DIR}/{code}.json')
```

Source: https://gist.github.com/Derkades/e6aab6ffb311208c7a6f10eb9d56788b

## 🔁 Big item model files due to lots of overrides? Use this `jq` filter to merge smaller override files into an item model file!

When using the custom item models feature of Minecraft, the list of overrides in
an item model may get unwieldy and hard to read. Splitting the custom item
models over several items might not be a good solution if you desire all of the
custom items to share the same properties.

However, it is a false dichotomy that more custom items mean harder-to-read item
model files. You can keep each override in a separate JSON file and merge them
to a single model file before running PackSquash with the
[`jq`](https://stedolan.github.io/jq/) command-line tool:

```shell
jq -s '.[0] + { overrides: [ .[1:][].overrides[] ] }' paper.json *_paper_part.json
```

That `jq` filter works by:

- Keeping the first input JSON file as-is (`.[0]`).
- Adding or replacing its `overrides` key (`+ { overrides: ... }`) with an array
  that is generated from concatenating together the values of the `overrides`
  array in the second and next JSON files (`[ .[1:][].overrides[] ]`).

As an example, given the following files:

### `paper.json` file contents

```json
{
  "parent": "item/generated",
  "textures": {
    "layer0": "item/paper"
  }
}
```

### `*_paper_part.json` files contents

```json
{
  "overrides": [
    {
      "predicate": {
        "custom_model_data": 1
      },
      "model": "custom/ui/buttons/play_button"
    }
  ]
}
```

```json
{
  "overrides": [
    {
      "predicate": {
        "custom_model_data": 12000
      },
      "model": "custom/npc/bank_piglin"
    }
  ]
}
```

It will generate the following valid item model file that can be included in
your pack as usual:

```json
{
  "parent": "item/generated",
  "textures": {
    "layer0": "item/paper"
  },
  "overrides": [
    {
      "predicate": {
        "custom_model_data": 1
      },
      "model": "custom/ui/buttons/play_button"
    },
    {
      "predicate": {
        "custom_model_data": 12000
      },
      "model": "custom/npc/bank_piglin"
    }
  ]
}
```

For more information and context about this filter, check out this [GitHub
issue](https://github.com/ComunidadAylas/PackSquash/issues/96#issuecomment-1096556922).
