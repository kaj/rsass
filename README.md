# rsass

Sass reimplemented in rust with nom (very early stage).  The "r" in
the name might stand for the Rust programming language, or for my name
Rasmus.

[![Build Status](https://travis-ci.org/kaj/rsass.svg?branch=master)](https://travis-ci.org/kaj/rsass)

The build contains a small but growing number of tests from
https://github.com/sass/sass-spec .
Attempting to run the full suite of 6025 tests found 301 passing, so
this project is really not done yet.

If you want a working rust library for sass, you will probably be
better of with [sass-rs](https://crates.io/crates/sass-rs) which is a
rust wrapper around libsass, or possibly
[sassers](https://crates.io/crates/sassers) which is another early
stage pure rust implementation.
That said, this implementation has reached a version where I find it
usable for my personal projects, and the number of working tests are
improving.
