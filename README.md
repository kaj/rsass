# rsass

Sass reimplemented in rust with nom (very early stage).  The "r" in
the name might stand for the Rust programming language, or for my name
Rasmus.

[![Build Status](https://travis-ci.org/kaj/rsass.svg?branch=master)](https://travis-ci.org/kaj/rsass)
[![Crate](https://meritbadge.herokuapp.com/rsass)](https://crates.io/crates/rsass)
[![docs](https://docs.rs/rsass/badge.svg)](https://docs.rs/rsass)

## Commandline

To make compiling faster when rsass is used as a library crate (which
is probably the dominant use-case), I have made building a usefull
command-line utility optional.
To build the commandline, define the `commandline` feature when
building.

    cargo build --release --features=commandline

## Sass language and implemetation status

The sass language [is defined in its reference
doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
This implementation is incomplete but getting there, if slowly.

Progress: ![1466](http://progressed.io/bar/146?scale=333&suffix=6)
of 3339 tests passed
(or 1520 of 6094 when claiming to be libsass).

If you want a working rust library for sass right now, you will
probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
which is a rust wrapper around libsass.
Another alternative is [sassers](https://crates.io/crates/sassers)
which is another early stage pure rust implementation.
That said, this implementation has reached a version where I find it
usable for my personal projects, and the number of working tests are
improving.
