# Welcome to Polyanya contributing guide

Thank you for investing your time in contributing to this project!

## New contributor guide

To get an overview of the project, read the [README](README.md). Here are some resources to help you get started with open source contributions:

- [Set up Git](https://docs.github.com/en/get-started/quickstart/set-up-git)
- [Collaborating with pull requests](https://docs.github.com/en/github/collaborating-with-pull-requests)

## Getting started

### Issues

#### Create a new issue

If you spot a problem with the library, [search if an issue already exists](https://docs.github.com/en/github/searching-for-information-on-github/searching-on-github/searching-issues-and-pull-requests#search-by-the-title-body-or-comments). If a related issue doesn't exist, you can [open a new issue](https://github.com/vleue/polyanya/issues/new/choose).

#### Solve an issue

Scan through our [existing issues](https://github.com/vleue/polyanya/issues) to find one that interests you. If you find an issue to work on, you are welcome to open a PR with a fix.

### Pull Request

GitHub Actions will run a few checks on each PR:
* Formatting, with `cargo fmt --all`
* Clippy lints, with `cargo clippy -- -D warnings`
* Tests, with `cargo tests` and `cargo test --release --examples`

Tests on examples can be quite expensive to compile and run, don't hesitate to not run those locally before creating a PR. 

Additionally, benchmarks are available to ensure your PR doesn't introduce regressions. They are not run in actions, you are encouraged to run them locally on the `main` branch then on your branch to check for any improvement.
