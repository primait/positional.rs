# positional.rs

[![CI](https://github.com/primait/positional.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/primait/positional.rs/actions/workflows/ci.yml)

This is a library for authoring and parsing positional files

[RustDoc](https://docs.rs/positional)

## Release

For releasing we have a manual approach that involves editing the `Cargo.toml` files.
Both crates matches (`positional`, `positional_derive`) the same version.

1. Bump the version in both `Cargo.toml`s.
  a. `positional/Cargo.toml`
  b. `positional_derive/Cargo.toml`
2. Commit and push the changes
3. Open a PR
4. Create a GitHub release matching same tag
