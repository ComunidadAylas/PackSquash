If you're reading this document, chances are you're thinking about contributing to PackSquash, or at least are curious about how the contribution process is. In any case, thank you for your interest on the project!

The next sections outline contributing guidelines and related tidbits of information that are deemed important to help you make better contributions: contributions which align with the vision, goals and ways of doing things of the project, and that are useful to its whole community of users and developers.

# Table of contents

- [❓ What is a contribution? What things can I contribute to?](#-what-is-a-contribution-what-things-can-i-contribute-to)
- [📌 Work organization](#-work-organization)
  - [🗺️ Roadmap](#️-roadmap)
  - [❗ Issues](#-issues)
  - [🗪 Discussions](#-discussions)
  - [🗩 Discord](#-discord)
- [👨‍💻 Technologies and development environment](#-technologies-and-development-environment)
  - [🦀 Rust](#-rust)
  - [⚙️ Integrated Development Environment (IDE)](#️-integrated-development-environment-ide)
  - [📦 Cargo](#-cargo)
  - [✔️ Cargo test runner](#️-cargo-test-runner)
  - [📎 Clippy](#-clippy)
  - [📝 rustfmt](#-rustfmt)
- [✏️ Submitting a pull request (PR)](#️-submitting-a-pull-request-pr)
  - [✒️ Contributor License Agreement (CLA)](#️-contributor-license-agreement-cla)

# ❓ What is a contribution? What things can I contribute to?

Due to the fact that GitHub is a Git hosting provider with features that cater mostly to software development, non-technical people usually see GitHub as a platform for programmers. In fact, there are programmers that agree with that perception, thinking that GitHub is not meant for non-technical people. However, software development is a broad activity that involves much more than writing computer code, and there is room for people of strikingly different backgrounds to make contributions to a software project in very different areas, without leaving GitHub.

Thus, contributions are a broad concept. The [Cambridge Dictionary](https://dictionary.cambridge.org/dictionary/english/contribution) defines a contribution as _"something that you contribute or do to help produce or achieve something together with other people, or to help make something successful"_. In the case of PackSquash, the following acts are examples of contributions:

- Opening an issue to constructively suggest an improvement, or write a good bug report.
- Engaging with the community in a friendly manner, sharing experiences, ideas or showing willingness to help the project in some meaningful way.
- Using your knowledge about PackSquash to improve the already existing documentation, creating new useful documentation, or author educational resources (videos, slides, etc.) that divulge that knowledge among fellow users.
- Submitting answers to user questions about PackSquash based on correct and updated facts.
- Recommending PackSquash to others that may find it relevant for their endeavours (interestingly, at the time these guidelines were written, recommendations are one of the main sources of traffic for the project!).
- Researching Minecraft or file format internals and sharing the relevant results of that research, in the form of ideas, documents or proofs of concept.
- Sharing packs that are useful for developers to analyze to improve PackSquash. For example, packs that were protected with techniques different than those of PackSquash, or that have a distribution of assets that highlights a problem or exhibits interesting performance characteristics.
- Writing or reviewing code that fixes a defect, adds a feature, or implements an improvement.
- Submitting a patch that improves [software quality](https://en.wikipedia.org/wiki/Software_quality), structurally or functionally.
- Donating some amount of money to the project.

As you can grasp from the previous non-exhaustive list, writing code, while it is an appreciated task, is just a fraction of everything that can be called a contribution. In general, we love hearing about PackSquash users that have something nice to share. We think that such interactions really humanize the development and make it more likely to succeed.

After all, free and open source software is about collaborative efforts, giving users the liberty to run, study, redistribute and improve the software, offering a public good that everyone can use, and taking some pride in all of that, not just making the source code available for download.

# 📌 Work organization

Predictably, any project needs some sort of organization and coordination. PackSquash is not an exception to this, and being familiar with how things are organized will come in handy for a contributor.

## 🗺️ Roadmap

We've opted to keep a simple [Kanban-inspired board](https://en.wikipedia.org/wiki/Kanban_(development)##Kanban_boards) to visualize the progress on the [project roadmap](https://github.com/ComunidadAylas/PackSquash/projects/1): ideas that would be nice for someone to tackle at some point, work items that will be completed for the next release, work items that are being worked on by someone, and work items that were completed and commited to the mainline.

This roadmap is the single most relevant means of planification for the whole project. If you are about to contribute in some way, the roadmap helps answering the following questions you might have:

- _Did someone think on this contribution before? Would it be a good contribution?_ ⇒ If an idea was accepted on the roadmap, it surely is a welcome improvement. If it is not, chances are that you've thought of something novel and suitable for the project, or something that's not suitable for the project. In the last case, if your contribution would take substantial effort and you think that there might be conceivable reasons for it to be rejected, the best course of action is to ask the community how they would feel about it first, to assess the likelihood of it being a welcome contribution that makes it to the mainline.
- _Who's working on what?_ ⇒ If someone is working on something, submitting an alternative, independently developed version of their work usually is a bad idea, as one of them will necessarily be rejected fully or in part. Some work items in the roadmap might have someone assigned to them; in this case, it's best to reach out and cooperate with the assigned person. If an item is not assigned to anyone, that usually means that nobody is working on it, but it's still a good idea to ask around, as maybe you find someone else interested on it that just didn't formally commit to that task yet.
- _What's the plan for the next release?_ ⇒ The scope for any given release at least contains the work items that were scheduled for that release in the roadmap. Moreover, it usually is wise to align your contributions with the scope of the next release. For example, if the next release is focused on fixing bugs, a code contribution that fixes more bugs will have higher chances of being accepted sooner than a code contribution that adds a complex, whole new feature. This doesn't imply that you _must_ restrain yourself from working on whatever you'd like, though. In any case, be aware that it could happen that your contribution just didn't come at the right time.

## ❗ Issues

[Issues](https://github.com/ComunidadAylas/PackSquash/issues) are filled out to formally report defects in PackSquash, suggest improvements or new features, and track work items with more detail than the cards in the roadmap Kanban board allow.

In order to maximize the usefulness of the issues you open, please heed the following points:

- _Each issue should be about only one thing_. Avoid expanding the scope of a single issue to talk about different bugs, features or work items.
- _If you are reporting a bug, make sure it happens on the latest builds and that you can reproduce it_. Reporting something that has already been fixed is a waste of time (although it is useful to let others know that you were affected by the bug, so the importance of the fix can be assessed better). Conversely, bugs that can't be reproduced are, at least, very difficult to identify and fix.
- _The title of the issue should be a good summary_. It should describe its topic using approximately 10 words, allowing to identify it quickly and uniquely. "Something doesn't work" is a terrible title for a bug report, because it gives almost no information about the problem and is not an unique title at all.
- _Make sure that the topic you're going to open an issue about does not already have an issue_. Having two separate issues to talk about the same thing serves no purpose. Please check if there are issues about your topic before creating a new one.
- _Follow the issue topic templates, if available_. The templates are meant to guide you in making a good issue, and you should not remove sections from them or leave them blank unless you have a good reason to do so.

An issue can be assigned to someone. When an issue is assigned to someone, it means that person is responsible for it. A person responsible for an issue works to put it in a resolved state, which means completing its associated work item, or making someone else complete that work item. Therefore, those assigned for an issue should be contacted for status updates and offering help to drive it to completion.

Issues can also have one or several labels. The following labels are especially relevant for contributors:

- _Good first issue_. This label is added at the discretion of the project maintainers to highlight issues that would be nice for first-time contributors to tackle, given their perceived approachability for beginners, usefulness for the project, and learning potential. In other words, they are fairly low hanging fruit that nevertheless can be a fulfilling endeavour for everyone.
- _Help wanted_. This is a generic label that means that some contribution to it would be appreciated, useful and accepted. The issue body should detail what kind of contributions would be welcome.

## 🗪 Discussions

The [GitHub Discussions](https://github.com/ComunidadAylas/PackSquash/discussions) widget is available for the PackSquash repository. It is meant to be used as a forum of sorts, to formally and publicly discuss or announce topics that are not suitable for an issue.

## 🗩 Discord

Discord is the main place where the PackSquash community gathers to discuss topics informally, including those related to work organization. Even the best contributing guidelines can't be a substitute for actual quick communication between contributors and users, so you probably want to join the Discord server mentioned in the `README.md` file to ask questions, meet fellow contributors, help others and be helped by others.

# 👨‍💻 Technologies and development environment

Like any other software project, PackSquash uses a number of technologies, and has a particular development environment. These details usually are only interesting for those that want to dive into the code that makes PackSquash tick.

## 🦀 Rust

[Rust](https://www.rust-lang.org/) is the programming language PackSquash source code was written in. Therefore, to develop code for PackSquash, you should have some working knowledge of Rust and be familiar with its ecosystem first. If you're new to Rust, the free [Rust Book](https://doc.rust-lang.org/book/) is the most comprehensive and easy to read introduction that will teach you the basics of the programming language and its development tools.

While it is an unusual choice within the Minecraft community, its performance (which is roughly on par with C++), portability, design choices, community and availability of high quality, well documented and well maintained open source libraries that nicely abstract file optimization tasks justified its usage. Due to the fact that Rust compiles to machine code, operating systems can execute it without needing any external runtime, which simplifies deployment and avoids a class of problems related to the management of that runtime.

PackSquash always targets the latest [nightly Rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html##rustup-and-the-role-of-rust-nightly) release at the time a commit is made. This is because it depends on some unstable language features to achieve more elegant code. So your contributions can also depend on such unstable features, although try to use good judgement when determining whether an unstable feature is really necessary: such features might be removed or changed at any time. Also, in the future, this will also make life more difficult for those that use PackSquash as a Rust library, because they might desire to limit themselves to a stable, time-invariant Rust version in their projects. If your changes depend on some unstable feature, it'd be nice to at least mention how they could be refactored to work in stable Rust. To ensure that you are using the nightly Rust toolchain for everything you do, run a command like `rustup default nightly`.

Finally, on Windows platforms, PackSquash targets the MinGW-w64 (GNU GCC) Rust toolchain, instead of the Microsoft Visual C (MSVC) toolchain bundled with Visual Studio. This is because MinGW makes Windows be more Unix-like for building C dependencies, which helps with portability and streamlines development accross platforms.

## ⚙️ Integrated Development Environment (IDE)

While it is technically possible to write code using a bare plain text editor, it's very likely that you will be more productive if you use an IDE.

Thus, even though it's not technically an IDE, PackSquash developers use [Visual Studio Code](https://code.visualstudio.com/) with extensions that provide IDE features like advanced syntax highlighting, autocompletion and a debugger. To make it easier to set VS Code up, a `.vscode` folder is checked out to the repository, that will configure VS Code and make it prompt you to install the extensions you will need to get started when you open the repository folder.

[VS Code is the IDE of choice for many developers according to StackOverflow surveys](https://insights.stackoverflow.com/survey/2021##integrated-development-environment), and chances are that you like it or are using it already. It's also used regularly by other contributors, so it's in the best interests of everyone that it works well with the PackSquash development workflow. However, you can also use other IDEs, like IntelliJ and Eclipse, if you think the benefits of doing so are worth it. At the end of the day, what really matters for accepting or rejecting contributions is what ends up being submitted.

## 📦 Cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the standard package and dependency manager for Rust. It also takes care of compiling software artifacts, including PackSquash itself, and executing build scripts, which in the case of C libraries PackSquash depends on invoke the platform C compiler to build them.

For the most part, building PackSquash with Cargo is easy. After installing the latest nightly Rust toolchain, running `cargo build` on the repository root folder should be enough to produce a working debug executable. In reality, however, that will not work out of the box, due to C libraries that depend on a properly configured platform C toolchain and development files. The best way to get these set up is to look at the [CI build workflow](https://github.com/ComunidadAylas/PackSquash/blob/master/.github/workflows/build.yml) and replicate its steps on your machine.

A corollary of the previously stated complications with C libraries is that your contributions should not add C code dependencies for no good reason. If possible, choose native Rust libraries over libraries written in other languages, unless there is a noticeable advantage in not doing so, like demonstrably better performance.

## ✔️ Cargo test runner

In contrast with other development environments, that rely on separate tools to run automated tests, Cargo also bundles a test runner. To run all the tests for a Rust project, just execute `cargo test`.

If applicable, any code contribution you submit must add or modify unit tests to show that the changes work as intended. Ideally, unit tests should be independent from each other and test the smallest program section that implements some valuable behavior, which usually is a function or method.

## 📎 Clippy

The Rust Project itself maintains [Clippy](https://github.com/rust-lang/rust-clippy##readme), the standard static analysis tool for Rust, that helps improving Rust code by suggesting changes that improve its correctness, how idiomatic it is, simplicity and, in some cases, performance. If you're familiar with static analysis tools used in other languages, like SonarQube, ESLint or PHPStan, you'll find that Clippy fulfills a similar purpose.

The rationale to use Clippy is that, even though Rust has a strict compiler that requires the code you write to be mostly correct in order for it to build, it probably can be improved nevertheless. As Clippy is an automated tool that proposes useful suggestions, running it before submitting a code contribution allows both reviewers and contributors to cut to the chase and discuss less trivial matters. Your code contributions should not cause any new Clippy warning, but if they do and you think you have good reasons to ignore them, please supress the warnings via an attribute and state your reasoning.

To get started with Clippy, make sure that your Rust toolchain includes the Clippy component, by running `rustup component add clippy`. After that's done, you can run Clippy with `cargo clippy`, just like any other Cargo command. Remember to use the Clippy component bundled with the nightly toolchain, not the stable one!

## 📝 rustfmt

[rustfmt](https://github.com/rust-lang/rustfmt) is the standard code formatter for Rust, and is also maintained by the Rust Project. As its name suggests, it's a tool that formats the source code, according to a configured style.

As with Clippy, running rustfmt helps making your code contribution look as others expect, enforcing a consistent style over the codebase and saving trivial reviews proposing whitespace changes. All the code you contribute must be formatted as if rustfmt was run on it. If you think that the style it enforces is bad for some reason, please state why and try to suggest how to improve it.

To get started with rustfmt, make sure that your Rust toolchain includes the rustfmt component, by running `rustup component add rustfmt`. After that's done, you can run rustfmt with `cargo fmt`, just like any other Cargo command. Remember to use the rustfmt component bundled with the nightly toolchain, not the stable one!

# ✏️ Submitting a pull request (PR)

If your contribution involves editing files in the repository, you'll need to submit a PR via GitHub, so the changes can be reviewed and hopefully incorporated to the project. You can do this by following [the usual procedure explained in the GitHub documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request).

Pull requests should follow an etiquette, describe what they change, and show some effort to make them as easy to review as possible. [The pull request guide at the Atlassian blog](https://www.atlassian.com/blog/git/written-unwritten-guide-pull-requests) does a good job explaining how to make a good pull request, and you should read it.

If your PR contains code modifications, you should also update the relevant documentation and make sure that every continuous integration check passes, unless you're explicitly told otherwise, in addition to following the relevant guidelines highlighted in the technologies section of this document.

## ✒️ Contributor License Agreement (CLA)

A CLA is a legal document that explicitly states the terms under which we accept contributions. The purpose of a CLA is to reiterate that you have the intellectual property of every change you submit to the project, while facilitating the resolution of disputes and stipulating how the project can use and distribute your work.

Every contributor that submits a PR must explicitly agree to the CLA for their contributions to be taken into account. When a contributor that did not yet agree to the CLA submits a PR, a bot will comment on it with instructions on how to accept the CLA, and will add another comment when it is accepted. The current version of the CLA is available [here](https://gist.github.com/AlexTMjugador/cac877c806a4b2d98df9ee48f78a1521).

We reckon that a CLA might seem a bit overkill for a project like PackSquash, and that some contributors might feel uneasy with the possibility of their work being distributed under a different license. However, we think that letting contributors be aware of the rules under which their work will be used provides safety to them and is a good transparency exercise. In fact, prominent free and open source software projects like Eclipse, Ubuntu and KDE, are already using a CLA for similar reasons.

About the relicensing possibility that our CLA opens, we think of it as a _reserve power_ that will only be used responsibly, to avoid the legal complications that would otherwise arise. With "responsibly", we mean that we will not do take such relicensing decisions lightly, we will only do them after publicly asking and letting our community know about them, and we will always make PackSquash available under an open source license.
