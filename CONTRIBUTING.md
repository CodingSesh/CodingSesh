# Contributing to CodeingSesh

Thank you for your interest in contributing to CodeingSesh! There are many ways to contribute, and we appreciate all of them. This document is a bit long, so here's links to the major sections:

- [Bug Reports](#bug-reports)
- [Pull Request](#pull-request)
  - [Pull-Request Title Prefix](#pull-request-title-prefix)
- [Things to Know Before Contributing](#things-to-know-before-contributing)

## Bug Reports

While bugs are unfortunate, they're a reality in software. We can't fix what we don't know about, so please report liberally. If you're not sure if something is a bug or not, feel free to file a bug anyway.

If you have the chance, before reporting a bug, please [search existing issues](https://github.com/CodeingSesh/CodeingSesh/search?q=&type=Issues&utf8=%E2%9C%93), as it's possible that someone else has already reported your error. This doesn't always work, and sometimes it's hard to know what to search for, so consider this extra credit. We won't mind if you accidentally file a duplicate report.

Similarly, to help others who encountered the bug find your issue, consider filing an issue with a descriptive title, which contains information that might be unique to it.

Opening an issue is as easy as following [this link](https://github.com/CodeingSesh/CodeingSesh/issues/new) and filling out the fields.

## Pull Request

We generally prefer you to PR your work earlier rather than later. This ensures everyone else has a better idea of what's being worked on, and can help reduce wasted effort. If work on your PR has just begun, please feel free to create the PR with [WIP] (work in progress) in the PR title, and let us know when it's ready for review in the comments.

Since mainnet has been released, the bar for having PRs accepted has been raised. Before submitting your PR for approval, please be ensure it:

- Includes a proper description of what problems the PR addresses, as well as a detailed explanation as to what it changes
- Explains whether/how the change is consensus breaking or breaks existing client functionality
- Contains unit tests exercising new/changed functionality
- Fully considers the potential impact of the change on other parts of the system
- Describes how you've tested the change
- Updates any documentation that's affected by the PR

If submitting a PR consisting of documentation changes only, please try to ensure that the change is significantly more substantial than one or two lines. For example, working through an install document and making changes and updates throughout as you find issues is worth a PR. For typos and other small changes, either contact one of the developers, or if you think it's a significant enough error to cause problems for other users, please feel free to open an issue.

The development team will be happy to help and guide you with any of these points and work with you getting your PR submitted for approval. Create a PR with [WIP] in the title and ask for specific assistance within the issue, or contact the dev team on any of the channels below.

### Pull-Request Title Prefix

Please consider putting one of the following prefixes in the title of your pull-request:

- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Formatting, missing semi-colons, white-space, etc
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests
- chore: Maintain. Changes to the build process or auxiliary tools/libraries/documentation

## Things to Know Before Contributing

**Note**: To work with CodeingSesh you must use rustup. Linux package managers typically carry a too old rust version. See build docs for more info.

CodeingSesh uses rusty hooks(git hooks) to write better code that just works. They require rustfmt and clippy to be installed to work properly. To install run

```sh
rustup update
rustup component add rustfmt clippy
```

CodeingSesh also uses [git journal](https://github.com/saschagrunert/git-journal)(The Git Commit Message and Changelog Generation Framework) which needs to be installed before commiting.

Via Cargo
```sh
cargo install git-journal
```

Or if you use Arch Linux
```sh
sudo pacman -S rust cargo
```
