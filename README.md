# rsass

Sass reimplemented in rust with nom (very early stage).  The "r" in
the name might stand for the Rust programming language, or for my name
Rasmus.

[![Build Status](https://travis-ci.org/kaj/rsass.svg?branch=master)](https://travis-ci.org/kaj/rsass)
[![Crate](https://meritbadge.herokuapp.com/rsass)](https://crates.io/crates/rsass)
[![docs](https://docs.rs/rsass/badge.svg)](https://docs.rs/rsass)

## Sass language and implemetation status

The sass language [is defined in its reference
doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
This implementation is incomplete but getting there, if slowly.

Progress: ![1359](http://progressed.io/bar/135?scale=333&suffix=9)
of 3331 tests passed
(or 1416 of 6086 when claiming to be libsass).

If you want a working rust library for sass right now, you will
probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
which is a rust wrapper around libsass.
Another alternative is [sassers](https://crates.io/crates/sassers)
which is another early stage pure rust implementation.
That said, this implementation has reached a version where I find it
usable for my personal projects, and the number of working tests are
improving.
