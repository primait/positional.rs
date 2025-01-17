# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

---

## [0.4.4] - 2025-01-06

# Changed

- remove ![doc(hidden)] from `ToPositionalField`

# Updated

- Updated `thiserror` to 2.0
- Updated `itertools` to 0.14 

---

## [0.4.3] - 2024-10-02

- replace the proc-macro-error dependency with proc-macro-error2

## [0.4.2]

### Changed

- update criterion to 0.5
- update itertools to 0.13
- update syn to 2.0

## [0.4.1]

- update Criterion dependency from 0.3.5 to 0.4.0
- expose PositionalResult<T> instead of Box<dyn Error>
- improved macro hygiene and added regression tests

## [0.3.0]

- limited support for unicode chars while parsing files. We are now leveraging a
  `Chars` iterator instead of an index slice.
  https://doc.rust-lang.org/std/primitive.str.html#method.chars
- fixed a bug where the library would panic (instead of returning an Err) while
  trying to parse a malformed row.

## [0.2.3]

- fix build pipeline

## [0.2.0]

- `from_positional_row` now accepts `&str` instead of `impl ToString` and
  eliminates all internal allocations during parsing

## [0.1.3]

- `parse` function from the FromPositionalRow trait has been renamed to
  `from_positional_row` for coherence with the counterpart

## [0.1.2]

- FromPositionalRow and ToPositionalRow macros are now applicable also to enums

[Next]: https://github.com/primait/positional.rs/compare/0.4.2...HEAD
[0.4.2]: https://github.com/primait/positional.rs/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/primait/positional.rs/compare/0.3.0...0.4.1
[0.3.0]: https://github.com/primait/positional.rs/compare/0.2.3...0.3.0
[0.2.3]: https://github.com/primait/positional.rs/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/primait/positional.rs/compare/0.2.0...0.2.2
[0.2.0]: https://github.com/primait/positional.rs/compare/0.1.3...0.2.0
[0.1.3]: https://github.com/primait/positional.rs/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/primait/positional.rs/compare/0.1.1...0.1.2


[Unreleased]: https://github.com/primait/positional.rs/compare/0.4.4...HEAD
[0.4.4]: https://github.com/primait/positional.rs/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/primait/positional.rs/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/primait/positional.rs/releases/tag/0.4.2
