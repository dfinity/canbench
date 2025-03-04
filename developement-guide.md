# Development Guide

## Release Preparation

There are 3 packages that are published to crates.io, `canbench`, `canbench-rs` and `canbench-rs-macros`. Before publishing
them you should create a PR to bump the versions of the packages and then cut a new release on Github after the PR with
the new versions is merged. Let's say it's version `vX.X.X`.

Here's an example PR bumping the versions: https://github.com/dfinity/canbench/pull/71.

## Steps to Cut a Release

1. Identify the commit for the release, eg. `aff3eef`
2. Draft a new pre-release
    - Click on `Draft a new release` at the [releases page](https://github.com/dfinity/canbench/releases), make sure the right commit is selected
    - Create a new tag with the name `vX.X.X`.
    - Set the title to be `vX.X.X`.
    - Create a new tag which matches `vX.X.X`.
    - Choose the previous tag as the last release.
    - Add release notes. Github can generate the release notes by clicking on `Generated Release Notes`, modify as needed.
3. Click on publish release when ready.

## Steps to publish the new packages to crates.io

1. Generate an API token to use with `crates.io`. Login to crates.io with your Github account and then navigate to account settings. Go to the API tokens section and generate a new one.
2. Run `cargo login` on the command line to authenticate with `crates.io` before publishing the packages. Follow the prompts and enter your API key when requested.
3. Checkout the repo at the tag that was created for the release above `vX.X.X`. E.g. `git checkout vX.X.X`.
4. Publish the crates. There is a specific order in which they need to be published.
  - `cargo publish -p canbench-rs-macros`
  - `cargo publish -p canbench-rs`
  - `cargo publish -p canbench`
