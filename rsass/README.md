# rsass

Sass reimplemented in rust with nom.
The "r" in the name might stand for the Rust programming language, for
"re-implemented", or possibly for my name Rasmus.

This is the `rsass` library crate.
To use it, add `rsass` to your dependencies in a rust project.

    cargo add rsass

[![Crate](https://img.shields.io/crates/v/rsass.svg)](https://crates.io/crates/rsass)
[![docs](https://docs.rs/rsass/badge.svg)](https://docs.rs/rsass)
[![Github build](https://github.com/kaj/rsass/workflows/CI/badge.svg)](https://github.com/kaj/rsass/actions)

## Sass language and implementation status

The sass language [is defined in its reference
doc](http://sass-lang.com/documentation/).
This implementation is incomplete but getting there, if slowly.

Progress: 4500 of 6843 tests passed.

If you need _complete_ sass support, you'll need to call
[dart sass](https://sass-lang.com/dart-sass).
Another alternative is [grass](https://crates.io/crates/grass)
which is another incomplete pure rust implementation.
That said, this implementation has reached a version where I find it
usable for my personal projects, and the number of working tests are
improving.

## Contributing

Welcome!
The first step in any contribution is probably to either try to use
the crate or to read some of the documentation.
When you do, you might find something broken, not yet implemented, or
just plain incomprehensible.
If so, please
[see if there is an issue](https://github.com/kaj/rsass/issues)
matching the problem or file a new one.

If you contribute code through a pull request, github will
automatically check that the code compiles and passes its tests with
all required versions of rust, and that the code is properly formatted
according to rustfmt.
Hopefully, I will then review the code, and either ask you for changes
or merge it.
This is a hobby project, so please excuse if the review is delayed.

I, the rsass maintainer, will do my best to follow both the
[Sass Community Guidelines](https://sass-lang.com/community-guidelines)
and the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct),
and I ask you to do the same.
