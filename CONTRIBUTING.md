# ❤️ Contributing Guidelines

If you are reading this document, you probably want to contribute to PackSquash,
or at least are curious about how the contribution process is. In any case,
thank you for your interest in the project!

The following sections outline contributing guidelines and related tidbits of
information deemed important to help you make better contributions:
contributions that align with the vision, goals, and ways of doing things of the
project, beneficial to its whole community of users and developers.

# Table of contents

- [❓ What is a contribution? What things can I contribute
  to?](#-what-is-a-contribution-what-things-can-i-contribute-to)
- [📌 Work organization](#-work-organization)
  - [🗺️ Roadmap](#️-roadmap)
  - [❗ Issues](#-issues)
  - [🗪 Discussions](#-discussions)
  - [🗩 Discord](#-discord)
- [👨‍💻 Technologies and development
  environment](#-technologies-and-development-environment)
  - [🦀 Rust](#-rust)
  - [⚙️ Integrated Development Environment
    (IDE)](#️-integrated-development-environment-ide)
  - [📦 Cargo](#-cargo)
  - [✔️ Cargo test runner](#️-cargo-test-runner)
  - [📎 Clippy](#-clippy)
  - [📝 rustfmt](#-rustfmt)
  - [✨ Other Cargo subcommands](#other-cargo-subcommands)
- [✏️ Submitting a pull request (PR)](#️-submitting-a-pull-request-pr)
  - [✒️ Contributor License Agreement
    (CLA)](#️-contributor-license-agreement-cla)

# ❓ What is a contribution? What things can I contribute to?

Since GitHub is a Git hosting provider with features that cater mainly to
software development, non-technical people usually see GitHub as a platform for
programmers. Some programmers agree with that perception, thinking that GitHub
is not meant for non-technical people. However, software development is a broad
activity that involves much more than writing computer code, and there is room
for people of strikingly different backgrounds to make contributions to a
software project in very different areas without leaving GitHub.

Thus, contributions are a broad concept. The [Cambridge
Dictionary](https://dictionary.cambridge.org/dictionary/english/contribution)
defines a contribution as _"something that you contribute or do to help produce
or achieve something together with other people, or to help make something
successful"_. In the case of PackSquash, the following acts are examples of
contributions:

- Opening an issue to suggest an improvement constructively or write a good bug
  report.
- Engaging with the community in a friendly manner, sharing experiences, ideas
  or showing a willingness to help the project in some meaningful way.
- Using your knowledge about PackSquash to improve the already existing
  documentation, creating new user documentation, or author educational
  resources (videos, slides, etc.) that divulge that knowledge among fellow
  users.
- Submitting answers to user questions about PackSquash based on correct and
  updated facts.
- Recommending PackSquash to others that may find it relevant for their
  endeavors (interestingly, at the time these guidelines were written,
  recommendations are one of the main sources of traffic for the project!).
- Researching Minecraft or file format internals and sharing the relevant
  results of that research, in the form of ideas, documents, or proofs of
  concept.
- Sharing packs that are useful for developers to analyze to improve PackSquash.
  For example, packs that were protected with techniques different from those of
  PackSquash, or that have a distribution of assets that highlights a problem or
  exhibits interesting performance characteristics.
- Writing or reviewing code that fixes a defect, adds a feature or implements an
  improvement.
- Submitting a patch that improves [software
  quality](https://en.wikipedia.org/wiki/Software_quality), structurally or
  functionally.
- Donating some amount of money to the project.

As you can grasp from the previous non-exhaustive list, writing code, while it
is an appreciated task, is just a fraction of everything that can be called a
contribution. In general, we love hearing about PackSquash users that have
something nice to share. We think such interactions humanize development and
make it more likely to succeed.

After all, free and open-source software is about collaborative efforts, giving
users the liberty to run, study, redistribute and improve the software, offering
a public good that everyone can use, and taking some pride in all of that, not
just making the source code available for download.

# 📌 Work organization

Predictably, any project needs some sort of organization and coordination.
PackSquash is not an exception, and being familiar with how things are organized
will come in handy for a contributor.

## 🗺️ Roadmap

We have opted to keep a simple [Kanban-inspired
board](https://en.wikipedia.org/wiki/Kanban_(development)##Kanban_boards) to
visualize the progress on the [project
roadmap](https://github.com/ComunidadAylas/PackSquash/projects/1): ideas nice
for someone to tackle at some point, work items that will be completed for the
next release, work items that are being worked on by someone, and work items
that were completed and committed to the mainline.

This roadmap is the most relevant means of planning for the whole project. The
roadmap helps potential contributors to answer the following questions:

- _Did someone think about this contribution before? Would it be a good
  contribution?_ ⇒ If an idea was accepted on the roadmap, it assuredly is a
  welcome improvement. If it is not, then chances are that you have thought of
  something novel and suitable for the project, or something that is not
  suitable for the project. In the last case, if your contribution would take
  substantial effort, and you think that there might be conceivable reasons for
  it to be rejected, the best course of action is to ask the community how they
  would feel about it first, to assess the likelihood of it being a welcome
  contribution that makes it to the mainline.
- _Who is working on what?_ ⇒ If someone is working on something, submitting an
  alternative, independently developed version of their work usually is a bad
  idea, as one of them will necessarily be rejected completely or in part. Some
  work items in the roadmap might have someone assigned to them; in this case,
  it is best to reach out and cooperate with the assigned person. If an item is
  not assigned to anyone, that usually means that nobody is working on it, but
  it still is a good idea to ask around, as maybe you find someone else
  interested in it that did not formally commit to that task yet.
- _What is the plan for the next release?_ ⇒ The scope for any given release at
  least contains the work items that were scheduled for that release in the
  roadmap. Moreover, it is usually wise to align your contributions with the
  scope of the next release. For example, if the next release is focused on
  fixing bugs, a code contribution that fixes more bugs will have higher chances
  of being accepted sooner than a code contribution that adds a complex, whole
  new feature. This does not imply that you _must_ restrain yourself from
  working on whatever you would like, though. In any case, be aware that it
  could happen that your contribution just did not come at the right time.

## ❗ Issues

[Issues](https://github.com/ComunidadAylas/PackSquash/issues) are filled out to
formally report defects in PackSquash, suggest improvements or new features, and
track work items with more detail than the cards in the roadmap Kanban board
allow.

In order to maximize the usefulness of the issues you open, please heed the
following points:

- _Each issue should be about only one thing_. Avoid expanding the scope of a
  single issue to talk about different bugs, features, or work items.
- _If you are reporting a bug, make sure it happens on the latest builds and
  that you can reproduce it_. Reporting something that has already been fixed is
  a waste of time (although it is useful to let others know that you were
  affected by the bug, so the importance of the fix can be assessed better).
  Conversely, bugs that cannot be reproduced are, at least, very difficult to
  identify and fix.
- _The title of the issue should be a good summary_. It should describe its
  topic using approximately ten words, allowing to identify it quickly and
  uniquely. "Something does not work" is a terrible title for a bug report,
  because it gives almost no information about the problem and is not a unique
  title at all.
- _Make sure that the topic you are going to open an issue about does not
  already have an issue_. Having two separate issues to talk about the same
  thing serves no purpose. Please check if there are issues with your topic
  before creating a new one.
- _Follow the issue topic templates if available_. The templates are meant to
  guide you in making a good issue, and you should not remove sections from them
  or leave them blank unless you have a good reason to do so.

An issue can be assigned to someone. When an issue is assigned to someone, it
means that person is responsible for it. A person responsible for an issue works
to put it in a resolved state, which means completing its associated work item
or making someone else complete that work item. Therefore, those assigned for an
issue should be contacted for status updates and offering help to drive it to
completion.

Issues can also have one or several labels. The following labels are especially
relevant for contributors:

- _Good first issue_. This label is added at the discretion of the project
  maintainers to highlight issues that would be nice for first-time contributors
  to tackle, given their perceived approachability for beginners, usefulness for
  the project, and learning potential. In other words, they are fairly low-hanging
  fruit that nevertheless can be a fulfilling endeavor for everyone.
- _Help wanted_. This is a generic label that means that some contribution to it
  would be appreciated and likely accepted. The issue body should detail what
  kind of contributions would be welcome.

## 🗪 Discussions

The [GitHub
Discussions](https://github.com/ComunidadAylas/PackSquash/discussions) widget is
available for the PackSquash repository. It is meant to be used as a forum of
sorts to formally and publicly discuss or announce topics that are not suitable
for an issue.

## 🗩 Discord

Discord is the central place where the PackSquash community gathers to discuss
topics informally, including those related to work organization. Even the best
contributing guidelines cannot be a substitute for actual quick communication
between contributors and users, so you probably want to join the Discord server
mentioned in the `README.md` file to ask questions, meet fellow contributors,
help others, and be helped by others.

# 👨‍💻 Technologies and development environment

Like any other software project, PackSquash uses a number of technologies and
has a particular development environment. These details usually are only
interesting for those that want to dive into the code that makes PackSquash
tick.

## 🦀 Rust

[Rust](https://www.rust-lang.org/) is the programming language PackSquash source
code was written in. Therefore, to develop code for PackSquash, you should have
some working knowledge of Rust and be familiar with its ecosystem first. If you
are new to Rust, the free [Rust Book](https://doc.rust-lang.org/book/) is the
most comprehensive and easy-to-read introduction that will teach you the basics
of the programming language and its development tools.

While it is an unusual choice within the Minecraft community, its performance
(which is roughly on par with C++), portability, design choices, community and
availability of high quality, well documented, and well maintained open-source
libraries that nicely abstract file optimization tasks justified its usage. Due
to the fact that Rust compiles to machine code, operating systems can execute it
without needing any external runtime, which simplifies deployment and avoids a
class of problems related to the management of that runtime.

PackSquash always targets the latest [nightly
Rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html##rustup-and-the-role-of-rust-nightly)
release at the time a commit is made. This is because it depends on some
unstable language features to achieve more elegant code. So your contributions
can also depend on such unstable features, but try to use good judgment when
determining whether an unstable feature is really necessary: such features might
be removed or changed at any time. Also, in the future, this will also make life
more difficult for those that use PackSquash as a Rust library because they
might desire to limit themselves to a stable, time-invariant Rust version in
their projects. If your changes depend on some unstable feature, it would be
best to at least mention how they could be refactored to work in stable Rust.
The `rust-toolchain` file sets up an
[override](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
to ensure that you are using the latest nightly Rust toolchain, but you can also
globally use the nightly toolchain by default after running `rustup default nightly`.

Finally, on Windows platforms, PackSquash targets the MinGW-w64 (GNU GCC) Rust
toolchain, instead of the Microsoft Visual C (MSVC) toolchain bundled with
Visual Studio. This is because MinGW makes Windows be more Unix-like for
building C dependencies, which helps with portability and streamlines
development across platforms.

## ⚙️ Integrated Development Environment (IDE)

While it is technically possible to write code using a bare plain text editor,
it is very likely that you will be more productive if you use an IDE.

Thus, even though it is not technically an IDE, PackSquash developers use
[Visual Studio Code](https://code.visualstudio.com/) with extensions that
provide IDE features like advanced syntax highlighting, autocompletion, and a
debugger. A `.vscode` folder is checked out to the repository to make VS Code
easier to set up.

[VS Code is the IDE of choice for many developers according to StackOverflow
surveys](https://insights.stackoverflow.com/survey/2021##integrated-development-environment),
and the chances are that you like it or are using it already. It is also used
regularly by other contributors, so it is in the best interests of everyone that
it works well with the PackSquash development workflow. However, you can also
use other IDEs, like CLion and Eclipse, if you think the benefits of doing so
are worth it. If you do not mind it being commercial and closed source, CLion is
also a solid choice if you want more features, like a graphical profiler and a
snappy experience overall. In the end, what matters for accepting or rejecting
contributions is what ends up being submitted.

## 📦 Cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the standard package and dependency
manager for Rust. It also takes care of compiling software artifacts, including
PackSquash itself, and executing build scripts, which in the case of C libraries
PackSquash depends on invoke the platform C compiler to build them.

For the most part, building PackSquash with Cargo is easy. After installing Rust
and Cargo, running `cargo build` on the repository root folder should be enough
to produce a working debug executable. In reality, however, that will not work
out of the box due to C libraries that depend on a properly configured platform
C toolchain and development files. The best way to get these set up is to look
at the [CI build
workflow](https://github.com/ComunidadAylas/PackSquash/blob/master/.github/workflows/ci.yml)
and replicate its steps on your machine.

A corollary of the previously stated complications with C libraries is that your
contributions should not add C code dependencies for no good reason. If
possible, choose native Rust libraries over libraries written in other
languages, unless there is a noticeable advantage in not doing so, like
demonstrably better performance.

## ✔️ Cargo test runner

In contrast with other development environments that rely on separate tools to
run automated tests, Cargo also bundles a test runner. To run all the tests for
a Rust project, just execute `cargo test`.

If applicable, any code contribution you submit must add or modify unit tests to
show that the changes work as intended. Ideally, unit tests should be
independent of each other and test the smallest program section that implements
some valuable behavior, which usually is a function or method.

## 📎 Clippy

The Rust Project itself maintains
[Clippy](https://github.com/rust-lang/rust-clippy##readme), the standard static
analysis tool for Rust that helps improve Rust code by suggesting changes that
improve its correctness, how idiomatic it is, simplicity, and, in some cases,
performance. If you are familiar with static analysis tools used in other
languages, like SonarQube, ESLint, or PHPStan, you will find that Clippy
fulfills a similar purpose.

The rationale to use Clippy is that, even though Rust has a strict compiler that
requires the code you write to be mostly correct in order for it to build, it
probably can be improved nevertheless. As Clippy is an automated tool that
proposes useful suggestions, running it before submitting a code contribution
allows both reviewers and contributors to cut to the chase and discuss less
trivial matters. Your code contributions should not cause any new Clippy
warning, but if they do, and you think you have good reasons to ignore them,
please suppress the warnings via an attribute and state your reasoning.

To get started with Clippy, make sure that your Rust toolchain includes the
Clippy component, by running `rustup component add clippy`. After that is done,
you can run Clippy with `cargo clippy`, just like any other Cargo command.
Remember to use the Clippy component bundled with the nightly toolchain, not the
stable one!

## 📝 rustfmt

[rustfmt](https://github.com/rust-lang/rustfmt) is the standard code formatter
for Rust, also maintained by the Rust Project. As its name suggests, it is a
tool that formats source code according to a configured style.

As with Clippy, running rustfmt helps make your code contribution look as others
expect, enforcing a consistent style over the codebase and saving trivial
reviews proposing whitespace changes. All the code you contribute must be
formatted as if rustfmt was run on it. If you think that the style it enforces
is inadequate for some reason, please state why and try to suggest how to
improve it.

To get started with rustfmt, make sure that your Rust toolchain includes the
rustfmt component by running `rustup component add rustfmt`. After that is done,
you can run rustfmt with `cargo fmt`, just like any other Cargo command.
Remember to use the rustfmt component bundled with the nightly toolchain, not
the stable one!

## ✨ Other Cargo subcommands

Depending on the scope of your changes to PackSquash code, you might also find
these subcommands to be handy:

- `cargo update` (install with `cargo install cargo-update`): updates all the
  dependencies specified in `Cargo.toml`. Useful for keeping up to date.
- `cargo upgrade`: updates the dependencies locked in `Cargo.lock`, but does not
  change the `Cargo.toml` file. Useful for keeping up to date, usually in
  combination with `cargo update`.
- `cargo bloat` (install with `cargo install cargo-bloat`): shows what functions
  and crates take the most space in the generated executable. Useful for
  optimizing code size.
- `cargo cache` (install with `cargo install cargo-cache`): shows the size of
  the local Cargo cache on your computer, what things take what disk space on
  it, and allows cleaning it easily with `cargo cache -a`. Useful to save quite
  a bit of disk space when there are stale source code downloads in that cache.
- `cargo deny` (install with `cargo install cargo-deny`): audits the dependency
  chain for known security vulnerabilities, license incompatibilities,
  dependencies on several versions of the same crate, and so on. Useful to make
  a better dependency selection.
- `cargo tree`: shows the dependency tree of the project, including transitive
  dependencies. Useful for managing dependencies and optimizing code size.

## 👍 Useful scripts

The `scripts` directory of the repository contains some useful scripts that you
can run with any POSIX-compliant shell (on Windows, try Git Bash or MinGW; on
Linux or macOS, use your favorite shell).

Among those scripts, the most useful one available to everyone and unrelated to
distribution is `manual_packsquash_test.sh`. By default, this script will build
and run PackSquash in release mode to optimize the pack at the `test_pack`
directory, creating a `pack.mcmeta` file for a resource pack if it does not
exist. The pack format version and directory can be set with the `-v` and
`-d` options, respectively. This script is meant to make it more convenient to
do manual end-to-end PackSquash tests. Feel free to adapt your local copy as you
see fit for your activities.

# ✏️ Submitting a pull request (PR)

If your contribution involves editing files in the repository, you will need to
submit a PR via GitHub, so the changes can be reviewed and hopefully
incorporated into the project. You can do this by following [the usual procedure
explained in the GitHub
documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request).

Pull requests should follow etiquette, describe what they change, and show some
effort to make them as easy to review as possible. [The pull request guide at
the Atlassian
blog](https://www.atlassian.com/blog/git/written-unwritten-guide-pull-requests)
does a good job explaining how to make a good pull request, and you should read
it. Commits should follow the style described in the [Conventional Commits
v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) specification.

If your PR contains code modifications, you should also update the relevant
documentation and make sure that every continuous integration check passes,
unless you are explicitly told otherwise, in addition to following the relevant
guidelines highlighted in the technologies section of this document. In case of
doubt or incomplete guidelines, follow the style and design choices of the
already existing code.

## ✒️ Contributor License Agreement (CLA)

A CLA is a legal document that explicitly states the terms under which we accept
contributions. The purpose of a CLA is to reiterate that you have the
intellectual property of every change you submit to the project while
facilitating the resolution of disputes and stipulating how the project can use
and distribute your work.

Every contributor that submits a PR must explicitly agree to the CLA for their
contributions to be taken into account. When a contributor that did not yet
agree to the CLA submits a PR, a bot will comment on it with instructions on how
to accept the CLA and its acceptance status. The current version of the CLA is
available
[here](https://gist.github.com/AlexTMjugador/cac877c806a4b2d98df9ee48f78a1521).

We reckon that a CLA might seem a bit overkill for a project like PackSquash and
that some contributors might feel uneasy with the possibility of their work
being distributed under a different license. However, we think that letting
contributors be aware of the rules under which their work will be used provides
safety to them and is a good transparency exercise. Prominent free and
open-source software projects like Eclipse, Ubuntu, and KDE, are already using a
CLA for similar reasons.

About the relicensing possibility that our CLA opens, we think of it as a
_reserve power_ that will only be used responsibly, to avoid the legal
complications that would otherwise arise. With "responsibly", we mean that we
will not take such relicensing decisions lightly, we will only do them after
publicly asking and letting our community know about them, and we will always
make PackSquash available under an open-source license.
