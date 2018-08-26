# rsass

Sass reimplemented in rust with nom (early stage).
The "r" in the name might stand for the Rust programming language, for
"re-implemented", or possibly for my name Rasmus.

[![Crate](https://meritbadge.herokuapp.com/rsass)](https://crates.io/crates/rsass)
[![docs](https://docs.rs/rsass/badge.svg)](https://docs.rs/rsass)
[![Build Status](https://travis-ci.org/kaj/rsass.svg?branch=master)](https://travis-ci.org/kaj/rsass)
[![Build status](https://ci.appveyor.com/api/projects/status/w0hfnjwx31f7eoxj/branch/master?svg=true)](https://ci.appveyor.com/project/kaj/rsass/branch/master)

## Commandline

To make compiling faster when rsass is used as a library crate (which
is probably the dominant use-case), I have made building the
command-line utility optional.
To build the commandline, define the `commandline` feature when
building.

    cargo build --release --features=commandline

## Sass language and implemetation status

The sass language [is defined in its reference
doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
This implementation is incomplete but getting there, if slowly.

Progress: ![1661](http://progressed.io/bar/166?suffix=1&scale=348)
of 3482 tests passed

If you want a working rust library for sass right now, you will
probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
which is a rust wrapper around libsass.
Another alternative is [sassers](https://crates.io/crates/sassers)
which is another early stage pure rust implementation.
That said, this implementation has reached a version where I find it
usable for my personal projects, and the number of working tests are
improving.
