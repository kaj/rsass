# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking changes

* `SourceName` and `sass::Item::Import` was changed by #62.

### Improvements

* Provide `From<bool>` for `css::Value` (and deprecate `Value::bool(v)`).
* PR #62: Improve `ParseError`, `SourcePos`, and `SourceName` by using
  located spans by [nom_locate](https://lib.rs/crates/nom_locate) in
  the parser.  A `sass::Item::Import` now handles where each file is
  imported from, to improve error reporting.
* Improve parsing of `@else` clauses.
* Update spec to 2020-10-29.


## Release 0.15.0 - 2010-10-25

Progress: 2320 of 5577 tests passed in dart-sass compatiblilty mode.

### Breaking changes

* The `Item::Comment` enum value now contains a `SassString` rather
  than a `String`.
* The `Error::ParseError` enum value changed, and `ErrPos` is replaced
  with `SourcePos` and `SourceName`.
* The error type of `parse_scss_data` changed to `ParseError`.
* `Unit::dimension()` now returns a `Dimension` rather than a `&str`.

### Improvements

* PR #79: Refactor some error handling.  Relates to #46.
* Improve map parsing (a map is parsed as a plain vec of key/value
  pairs, filterig duplicates happens only when it is evaluated).
* Allow multiple `&` items in the same selector.
* `&` evaluates to null when used as a value without enclosing selector.
* Implement the `@debug` directive.
* Fix `a % b` evaluation for negative values.
* Allow / ignore comments in properties and values.
* Allow `!default` and `!global` in any order on variable declarations.
* Improve `inspect(...)` formatting of list values.
* Refactor and improve arithmetic on different units.
* `ListSeparator` is now `Copy`.
* Allow interpolation in comments.
* Some refactoring

Tested with rustc 1.47.0 (18bf6b4f0 2020-10-07),
1.44.1 (c7087fe00 2020-06-17), 1.42.0 (b8cedc004 2020-03-09),
1.40.0 (73528e339 2019-12-16), 1.38.0 (625451e37 2019-09-23),
1.48.0-beta.5 (4c78178b1 2020-10-21), and
1.49.0-nightly (ffa2e7ae8 2020-10-24).

## Release 0.14.2 - 2010-10-13

Progress: 2294 of 5577 tests passed in dart-sass compatiblilty mode.

* Some improvements in list handling, formatting and introspection.
* Change default precision to 10, matching dart-sass.
* Semicolon is optional after variable, if last in block or file.
* Fix addition (and subtraction) of values with comparable units.
* Fix css value equality for map keys and other "special" places.
* Recognize known `@`-rules with escaped characters.
* Don't copy extra `@charset` rules to output (one is included if
  needed).
* Some internal refactor and cleanup.
* Update spec to 2020-10-07.

Tested with rustc 1.47.0 (18bf6b4f0 2020-10-07),
1.44.1 (c7087fe00 2020-06-17), 1.42.0 (b8cedc004 2020-03-09),
1.40.0 (73528e339 2019-12-16), 1.38.0 (625451e37 2019-09-23),
1.48.0-beta.2 (212e76c31 2020-10-08), and
1.49.0-nightly (8dae8cdcc 2020-10-12)


## Release 0.14.0 - 2020-10-03

Progress: 2234 of 5510 tests passed in dart-sass compatiblilty mode.

* PR #76 and #78: Target dart-sass rather than libsass compatibilty.
  This sets the target when testing and changes some different
  behaviour, including how strings are parsed and handled.
* PR #75 from @divergentdave: Parse and add `Value` variants for
  BigInt numbers
* PR #77: Support `rgba(r g b / a)` and `hsla(h s l / a)` functions,
  i.e. the `channels` parameter with div-separated alpha channel.
* Fix `@import` indention.
* Improve function default argument parsing and dont panic in
  `parse_value_data`.
* Handle dictionary-ellipsis style call-by-varargs for functions.
* PR #73 from @divergentdave: Remove three unwraps from escaped_char
* PR #71 from @connorskees: Further optimize number printing
* Update travis url in README.
* PR #70 from @svenstaro: Mention grass rather than sassers, as
  sassers appears to be dead while grass is in rather active
  development.
* Add a CHANGELOG.md
* Update spec to 2020-09-17.
* Update num-rational to 0.3.0.

Tested with rustc 1.46.0 (04488afe3 2020-08-24),
1.44.1 (c7087fe00 2020-06-17), 1.42.0 (b8cedc004 2020-03-09),
1.40.0 (73528e339 2019-12-16), 1.38.0 (625451e37 2019-09-23),
1.47.0-beta.7 (e28d2bd09 2020-10-01), and
1.48.0-nightly (8876ffc92 2020-10-02)


## Release 0.13.0 - 2020-04-19

Progress: 1634 of 3502 tests passed in libsass compatiblilty mode.

* output::Format wraps an output::Style and a precision.  The global
  precision setting is removed.
* Value and other structs no longer implents Display, instead the
  provid a `format` method, that takes an output format and provides a
  Display implementation.
* The `compile_value` rust function now takes a format argument.
* Issue #42 (partial): The `min(..)` and `max(...)` functions are
  handled as plain css functions when valid, while still handling them
  as sass functions when nessecary.
* PR #69, fixing #68: Avoid exponential complexity on parenthesis.
* Allow comments in `@function` bodies.
* PR #67 from @connorskees: Refactor and fix printing of numbers
* When defining multiple variables, having more names than values is
  allowed.  The extra names get null values.
* Convert som panics / unwraps to properly handled errors.
* There was massive restructuring in the test suite, so almost all
  tests moved around.  I moved remaining tests to build a single test
  program for the suite rather than several for different directories
  in the suite (this also means more tests are included in the
  conversion).
* Fix check_spec stats calculation.

Thanks to @connorskees for code contributions and bug reports.

Tested with rustc 1.42.0 (b8cedc004 2020-03-09),
1.40.0 (73528e339 2019-12-16), 1.38.0 (625451e37 2019-09-23),
1.36.0 (a53f9df32 2019-07-03), 1.43.0-beta.6 (062dea094 2020-04-18),
and 1.44.0-nightly (52fa23add 2020-04-18).


## Release 0.12.2 - 2020-02-23

* Fix #63 and #64: Handle DOS-style newlines
* Use structopt rather than raw clap for the CLI.
* Sass function `str_slice` require integer indexes.
* Some cleanup, some of it suggested by clippy.
* Removed link to no-longer working graphical progress bar in docs.

Thanks to @connorskees for bug reports.

Tested with rustc 1.41.0 (5e1a79984 2020-01-27),
1.40.0 (73528e339 2019-12-16), 1.38.0 (625451e37 2019-09-23),
1.36.0 (a53f9df32 2019-07-03), 1.42.0-beta.3 (86f329b41 2020-02-07),
and 1.43.0-nightly (436494b8f 2020-02-22).


## Release 0.12.0 - 2019-12-25

* Issue #54: Add a LICENSE file.
* PR #57: Allow channel string in rgba()
* Issue #58: Let ructe::Error implement std::error::Error.
* Issue #59: Improve `@import` and media rules.
* Implement `@error` and `@warn` directives.
* Improve string unquoting, including multi-position numerical
  escapes.
* Improve at-rule parsing.
* Fix strange operator handling.
* Implement modulo operator.
* Fix line breaks in comments.
* Handle bad argument to `random()` function.
* Update test suite to e9e219bdf (2019-12-19).
* Some refactorig and clean-up.

Thanks to @emielbeinema, @maxbrunsfeld, and @Boiethios for contributed
code and suggestios.

Tested with rustc 1.40.0 (73528e339 2019-12-16),
1.38.0 (625451e37 2019-09-23), 1.36.0 (a53f9df32 2019-07-03),
1.34.2 (6c2484dc3 2019-05-13), 1.41.0-beta.1 (eb3f7c2d3 2019-12-17),
and 1.42.0-nightly (a9c1c04e9 2019-12-24).


## Release 0.11.2 - 2019-12-25

* PR #56:  The url function is special.
  Handle `url(foo)` as simply a string if possible.  Sometimes, such
  as `url("foo" + $bar)`, it must be a function returning a string
  instead.  Partially adresses #42.  Fixes #41.
* PR #55: Implement raw css functions.
  Handles the `calc`, `element`, `env`, `expression`, `var`, and
  `progid:...` special functions.  Partially adresses #42.  Fixes #53.
* Issue #50: Make tests more readable.
* Improve error handling in selector fns.
  Avoid a panic in some error conditions.
* Update test suite (to fcd6bd6c0, 2019-09-24).
* Update bytecount to 0.6.0.
* Some internal refactoring.

Thanks to @glebm and @nic-harley for reports and suggestions.


## Release 0.11.0 - 2019-07-16

Sass-spec reports 1507 of 2865 tests passed in libsass compatibility
mode.

* Update `nom` to version 5.0.
* A string can start with a `#` without being a color.
* Correct parameter name for `unit`, `unitless`, `quote`, and
  `unquote` functions.
* The `random` function ignores units.
* An empty unquoted string behaves like null.
* Fix some issues with string offsets in `str_slice` and `str_insert`.
* The `quote` function can use single quotes when needed.
* Return an error rather than panicing on some invalid scss.
* Some test updates and fixes in spectest.

Tested with rustc 1.36.0 (a53f9df32 2019-07-03),
1.34.2 (6c2484dc3 2019-05-13), 1.32.0 (9fda7c223 2019-01-16),
1.37.0-beta.3 (2ba6de7e2 2019-07-12), and
1.38.0-nightly (4b65a86eb 2019-07-15).



## Release 0.10.2 - 2019-07-07

Sass-spec reports 1472 of 2802 tests passed in libsass compatibility mode.

* Update progress number in readme and documentation, since I forgot
  in 0.10.0.


## Release 0.10.0 - 2019-07-07

* PR #51: Impove string parsing/escaping.
* Issue #46, PR #47: Report file position of parse errors, and improve
  the parser to have errors reported in a more specific position.
* Improve error handling in functions, calling a function _can_ yield
  an error.  Partially solves #41: The unsupported call will now yield
  an error rather than hang `rsass`.
* Commandline changes:
  - Support `-I` commandline argument for include path.
  - Use lowercase `-v` for version (expected by the test runner).
* Some color function improvements:
  - Handle alpha percentatges in rgba.
  - The rgb function can be used as rgba with four args.
  - Use alpha propely when mixing colors.
  - Handle both alpha and opacity css function fallback.
  - Implement the grayscale function.
  - Support three arguments to `rgba` function.
* Some list function improvements:
  - `inspect` shows parens for empty list, and spells out `"null"` for
    a null value.
  - Join gets better at selecting wether result should be comma or
    space separated.
  - The length function treats a null as an empty list.
  - Handle singleton in `index` and `nth` functions.
* More function improvements:
  - Implement the `unique_id` function.
  - Check that indexes are sane in `str_slice` function.
  - Skip null args even if not last when evaluating a function to a
    function call.
  - Implement equality check for builtin functions.
* Error, not panic, on unknown mixin.
* Allow modifiers on attribute selectors.
* Color names are caseless.
* Improve handling of url imports.
* Update `deunicode` to 1.0 and `rand` to 0.7.0.
* Allow interpolation in namespacerule.
* Allow ";" after body block.
* Lots of test suite updates, including support for .hrx tests in
  spectest.  Target libsass compatibility in testing.

I don't really know if I should target libsass or dart-sass
compatibility, but libsass seems to be closer to my current
implementation, so target that at least for now.

Thanks to @maxbrunsfeld and @glebm for code and suggestions.

Tested with rustc 1.36.0 (a53f9df32 2019-07-03), 1.34.2 (6c2484dc3 2019-05-13), 1.32.0 (9fda7c223 2019-01-16), 1.37.0-beta.1 (178aa6611 2019-07-04), and 1.38.0-nightly (481068a70 2019-07-05).


## Release 0.9.8 - 2019-02-10

Sass-spec reports 1645 of 3440 tests for sass 3.6 passed.

* Improve support for `transparent`.
* PR #45 from @glebm: Automatically update passing / failing tests
  Hardkoded skiplists in spectest.rs is cleared apart for tests where
  rsass panics.
* Issue 43: Normalize newlines in tests.
* PR #44 from @glebm: Update to Rust 2018 Edition, Rust v1.31.0+
* Issue #39, PR #40 from @glebm: Parser CallArgs: Allow trailing
  spacelike sans ","
* Issue #32: Make the selectors module pub.
* Issue #34, PR #35 from @glebm: Accessing an undefined variable is an error.
* Allow input starting with UTF-8 U+FEFF BOM.
* Target sass spec version 3.6.

Thanks to contributors @glebm, @maxbrunsfeld and @sunjay for code and
suggestions.

Tested with rustc 1.32.0 (9fda7c223 2019-01-16),
1.31.0 (abe02cefd 2018-12-04), 1.33.0-beta.6 (b203178b6 2019-02-05),
and 1.34.0-nightly (3315728c0 2019-02-09).


## Release 0.9.6 - 2019-01-09

A default run of sass-spec found 1680 of 3635 tests passing.

* Issues #26, #30, PR #29 from @tw1t611; Add `fr` unit.
* Refactor and document the Number type used inside value types.
* Allow trailing comma in sass function definitions.
* The sass `length` function now correctly returns one for single
  (non-list) values.
* Fix a bug in color function parameter handling.
* Add libsass and parser to auto-testing setup.
* Update rand to 0.6.1.

Tested with 1.31.1 (b6c32da9b 2018-12-18),
1.30.1 (1433507eb 2018-11-07), 1.29.0 (aa3ca1994 2018-09-11),
1.28.0 (9634041f0 2018-07-30), 1.27.2 (58cc626de 2018-07-18),
1.26.2 (594fb253c 2018-06-01), 1.32.0-beta.12 (2bde39b8e 2019-01-05),
and 1.33.0-nightly (d22fa2d87 2019-01-08)


## Release 0.9.4 - 2018-09-23

A default run of sass-spec found 1672 of 3483 tests passing.

* Allow unicode variable names.
* Implement the precision argument.
* Implement hex colors with alpha.
* More flexible call_args; allow whitespace before separating commas,
  and allow a comma trailing the last argument.
* Test infrastructure: a program spectest now converts selected tests from
  sass-spec to integration tests, to facilitate keeping tests up-to-date.

Tested with rust 1.29.0 (aa3ca1994 2018-09-11),
1.28.0 (9634041f0 2018-07-30), 1.27.2 (58cc626de 2018-07-18),
1.26.2 (594fb253c 2018-06-01), 1.25.0 (84203cac6 2018-03-25),
1.30.0-beta.7 (0ebb25088 2018-09-22), and
1.30.0-nightly (4591a245c 2018-09-22).


## Release 0.9.2 - 2018-09-02

* Fix #25; Avoid crashing on some non-utf8 input.

Thanks again to @killercup and rust-fuzz/targets#119 for identifying
more parser crashes on non-utf8 input.

Tested with rustc 1.28.0 (9634041f0 2018-07-30),
1.27.2 (58cc626de 2018-07-18), 1.26.2 (594fb253c 2018-06-01),
1.25.0 (84203cac6 2018-03-25), 1.24.1 (d3ae9a9e0 2018-02-27),
1.29.0-beta.9 (b4ec8d46e 2018-09-01), and
1.30.0-nightly (28bcffead 2018-09-01).


## Release 0.9.0 - 2018-08-26

A default run of sass-spec found 1661 of 3482 tests passing.

Changes since version 0.8.0 includes:

* PR #24: Implement a Rgba type and use in css::Value and sass::Value.
* PR #22: Partial support for `@at-root`.  Supports `&` back-refs but
  not `with` and `without`.
* Add "foo/index.scss" and "foo/_index.scss" when attempting to find
  "foo" for import.
* The parser should fail rather than crash on some non-utf8 input.
* PR #21: Introduce a Number type, mainly for sharing some abstraction
  between css and sass values.
* Make the `rgba` function handle non-numeric inputs (e.g. calc(...)).
* Implement the `feature-exists` function.
* Make `!important` part of value rather than property, so it can be
  used in fuction / mixin arguments etc.
* A css3 pseudo-element selector may have arguments.
* Add support for unicode ranges.
* The `@each` construct can bind multipe values.
* PR #20: Division is now a BinOp rather than a special case.
* Minor improvement in parsing and formatting at-rules.
* Fix some clippy complaints.
* Minor documentation updates.
* Some updates to travis build script and test structure.
* Add appveyour (windows) continous integration testing.
* Depedency updates: num-rational 0.2.1, nom 4.0 (PR #23) and rand 0.5.

Thanks to @killercup and rust-fuzz/targets#119 for identifying some
parser crashes on non-utf8 input.

Tested with rustc 1.28.0 (9634041f0 2018-07-30),
1.27.2 (58cc626de 2018-07-18), 1.26.2 (594fb253c 2018-06-01),
1.25.0 (84203cac6 2018-03-25), 1.24.1 (d3ae9a9e0 2018-02-27),
1.29.0-beta.6 (5c5be098e 2018-08-25), and
1.30.0-nightly (39e6ba821 2018-08-25).


## Release 0.8.0 - 2018-04-07

A default run of sass-spec found 1583 of 3364 tests passing.

Changes since version 0.7.0 includes:

* PR #17, #18, and #19 from @ErichDonGubler: Improvements to `check_spec`.
* Allow trailing comma in dictionary definition.
* Implement the `get-function` function, making a (named) function a
  css::Value, and modify the call function to take such a Value.  The
  warning when the Value is a Literal is more motivated now.
* Add weight arg for invert function.
* If a function call fails, output it as-is.
* Allow `@include` w/o ";" to be last input.
* Use correct names for output styles.  The supported styles are
  `expanded` and `compressed`.
* Add maintenence status to manifest.
* Impement the `&` value, expanding to the selector containing the
  value.
* Implement the `selector-parse`, `selector-append`, and
  `selector-nest` functions.
* Fix AsciiExt warning on rust 1.23 and up.
* Fix #16, Do not build a no-op command line, by specifying commandline as
  required-features of the bin target.
  Thanks to @biluohc for the suggestion!
* Miscellaneous code cleanup, dependency updates and reformatting.

Tested with rustc 1.23.0 (766bd11c8 2018-01-01),
1.24.1 (d3ae9a9e0 2018-02-27), 1.25.0 (84203cac6 2018-03-25),
1.26.0-beta.2 (0e350672e 2018-04-05), and
1.27.0-nightly (eeea94c11 2018-04-06).


## Release 0.7.0 - 2017-12-28

A default run of sass-spec found 1466 of 3339 tests passing (or 1520
of 6094 when claiming to be libsass). Changes since version 0.6.0
includes:

* Fix #15 by making building the commandline utility optional.
* Fix some issues with string quoting and list whitespace.
* Improve support for `@`-rules.
* Use num-rational without default features, for faster compilation.

Tested with rustc 1.19.0 (0ade33941 2017-07-17),
1.20.0 (f3d6973f4 2017-08-27), 1.21.0 (3b72af97e 2017-10-09),
1.22.1 (05e2e1c41 2017-11-22), 1.23.0-beta.2 (c9107ee93 2017-12-08),
and 1.24.0-nightly (1abeb436d 2017-12-27).


## Release 0.6.0 - 2017-12-06

A default run of sass-spec found 1466 of 3339 tests passing (or 1520
of 6094 when claiming to be libsass). Changes since version 0.5.0
includes:

* Handle quoting and escaping in strings closer to correct.
* Updated lazy_static to version 1.0
* Some internal cleanup.

Tested with rustc 1.19.0 (0ade33941 2017-07-17),
1.20.0 (f3d6973f4 2017-08-27), 1.21.0 (3b72af97e 2017-10-09),
1.22.1 (05e2e1c41 2017-11-22), 1.23.0-beta.1 (082b0ff02 2017-11-21),
and 1.24.0-nightly (cfba0d446 2017-12-05).


## Release 0.5.0 - 2017-11-05

A default run of sass-spec found 1359 of 3331 tests passing (or 1416
of 6086 when claiming to be libsass). Changes since version 0.4.0
includes:

* Improve handling of quoted / unquoted strings and interpolation.
  Still not correct, but exhibits correct behavior in more tests than
  earlier.
* Support interpolation in selectors.
* Support interpolation in property names.
* PR #14: Implement map syntax and map functions.
* Make error handling not break if nom is compiled with verbose-errors.
* Miscellaneous internal cleanups, some suggested by clippy.

Tested with rustc 1.18.0 (03fc9d622 2017-06-06), 1.19.0 (0ade33941
2017-07-17), 1.20.0 (f3d6973f4 2017-08-27), 1.21.0 (3b72af97e
2017-10-09), 1.22.0-beta.2 (a0a837c45 2017-10-20), and 1.23.0-nightly
(d762b1d6c 2017-11-04).


## Release 0.4.0 - 2017-10-15

A default run of sass-spec found 1163 of 3331 tests passing (or 1227
of 6086 when claiming to be libsass). Changes since version 0.3.4
includes:

* Implemented bracketed lists.
* Support CSS3 pseudo elements
* Added functions: `unitless`, `content-exists`, `zip`,
  `is-bracketed`, `join`.
* Update the nom parser library to 3.2.
* PR #13 Split of css values from sass values.
* PR #10 Preparatory work for splitting Value
* PR #7 Extract parser into separate module
* PR #8 Extract som modules from lib.rs
* Pin rustfmt to 0.8.4, to avoid surprising formatting changes.
* PR #6 Don't require a git plugin for the check-spec.

Thank You to [Jonas Nicklas](https://github.com/jnicklas) for many
contributions.

Tested with rustc 1.18.0 (03fc9d622 2017-06-06), 1.19.0 (0ade33941
2017-07-17), 1.20.0 (f3d6973f4 2017-08-27), rustc 1.21.0 (3b72af97e
2017-10-09), 1.22.0-beta.1 (e694dd37b 2017-10-12), and 1.22.0-nightly
(7778906be 2017-10-14).


## Release 0.3.4 - 2017-06-05

A default run of sass-spec found 1146 of 3310 tests passing (or 1215
of 6065 when claiming to be libsass). Changes since version 0.3.2
includes:

* Improve handling of at-rules.
* Allow block comments in selectors.
* Make FileContext::file(..) public.
* Test and fix type-of for booleans.
* Test and fix parsing of some strange lists.
* Test and fix unary operations.
* Test and fix some strange string values.
* Some internal cleanup and code restructuring.

Tested in rust 1.15.1 (021bd294c 2017-02-08), 1.16.0 (30cf806ef
2017-03-10), 1.17.0 (56124baa9 2017-04-24), 1.18.0-beta.4 (0308c9865
2017-05-27), and 1.19.0-nightly (0418fa9d3 2017-06-04).


## Release 0.3.2 - 2017-05-06

A default run of sass-spec found 1053 of 3299 tests passing (or 1111
of 6054 when claiming to be libsass). Changes since version 0.3.0
includes:

* Clean up nom error creation, to be compatible with both default and
  verbose-errors nom.
* Implement some more quote strangenes.
* Minor internal cleanup

Tested in rust 1.15.1, 1.16.0, 1.17.0 (stable), 1.18.0-beta.1
(4dce67253 2017-04-25), and 1.19.0-nightly (f4209651e 2017-05-05).


## Release 0.3.0 - 2017-05-06

A default run of sass-spec found 1039 of 3299 tests passing (or 1097
of 6054 when claiming to be libsass). Changes since version 0.2.0
includes:

* PR #4: Support application-defined functions in rust.
* Equal strings are equal even if the quoting differs
  (e.g. foo == "foo").
* Improved value parsing in many ways.
* Implemened numeric rounding rules to match libsass default.
* Improved handling of null values.
* Fix some minor output formatting issues.
* Handle arguments to pseudo-selectors (which are themselfs selectors)
  properly.
* Support for rust 1.14 is dropped.

Tested in rust 1.15.1, 1.16.0, 1.17.0 (stable), 1.18.0-beta.1
(4dce67253 2017-04-25), and 1.19.0-nightly (f4209651e 2017-05-05).


## Release 0.2.0 - 2017-04-28

A default run of sass-spec found 841 of 3294 tests passing (or 899 of
6049 when claiming to be libsass). Changes since version 0.1.10
includes:

* Err results from public functions is now `Error`, not `String`.
  This is a breaking change in public api.
* PR #2: value interpolation (as separate values and in strings).
* Implement basic `@content` for mixins (some scoping issues remains).
* Support namespace properties.
* Improve parsing of unquoted strings.
* Implement the `call()` function.
* Implement the `ie-hex-str()` function.
* Allow alternative parameter names to `rgba()` function.
* Improve defered operators handling.
* Improve scopeing and value evaluating (but not enough).
* Allow function declarations in blocks.
* Implement `for`, `while`, and `each` in user-defined functions.
* Fix scoping error for while loops.
* Support css unary + and -.
* Misc internal or smaller improvements (some suggested by clippy).

Tested in rust 1.14.0, 1.15.1, 1.16.0, 1.17.0 (stable), 1.18.0-beta.1
(4dce67253 2017-04-25) and 1.18.0-nightly (94e884b63 2017-04-27).


## Release 0.1.10 - 2017-04-14

* Improve string concatenation with plus operator.
* Support `*` in selectors
* Support `\` in selectors
* Support varargs functions.
* Allow trailing commas in lists.
* Implement list functions `append`, `length`, and `index`.
* Implement numeric functions `min` and `max`.
* Improve `@for` and `@each` loop handling.
* Internal cleanup and refactorisation.

Tested in rust 1.14.0, 1.15.1, 1.16.0 (stable), 1.17.0-beta.3 (beta),
and 1.18.0-nightly (28a742997 2017-04-13).


## Release 0.1.8 - 2017-04-08

A default run of sass-spec found 576 of 3294 tests passing (or 619 of
6049 when claiming to be libsass). Changes since version 0.1.6
includes:

* Support user-defined functions.
* Support functions `global_variable_exists`, `function_exists`,
  `mixin_exists`.
* Support `@each`, `@for` and `@while` loops.
* Make dash and underscore equivalent in names.
* Fix all tests from `spec/scope`.
* Improve documentation.
* Miscellaneous refactorisations and cleanups.

Tested in rust 1.14.0, 1.15.1, 1.16.0 (stable), 1.17.0-beta.3 (beta),
and 1.18.0-nightly (53f4bc311 2017-04-07).


## Release 0.1.6 - 2017-03-05

A default run of sass-spec found 502 of 3289 tests passing (or 532 of
6044 when claiming to be libsass). Changes since version 0.1.4
includes:

* Many more functions implemented.
* Better support för `at` directieves.
* Support the `!default` tag on variable declaration.
* Support all known css units.
* Support `!important` tag on properties.
* Allow the `*` character inside comments.

Tested in rust 1.14.0, 1.15.1, 1.16.0-beta.3 (stable), and
1.17.0-nightly (b1e31766d 2017-03-03).


## Release 0.1.4 - 2017-02-08

A default run of sass-spec found 420 of 3286 tests running (or 449 of
6041 when claiming to be libsass).

* Very basic support for media queries.
* Implement more color functions (the internal representation of a
  color now uses Rational for the channels, to avoid rounding errors
  when using the result of one color function as input to another).
* More complete parsing of simple numbers.
* Support ambiguous imports, as examplified by basic test 33.
* Use the `?` operator rather than the `try!` macro.
* Proper handling of escaped quotes in quoted strings (basic test 53).
* Miscellaneous internal cleanups and refactorings

Tested in rust 1.14, 1.15, and stable and nightly as of 2017-02-08.


## Release 0.1.2 - 2017-01-29

Second release. Passes 380 tests of 6026 from sass-spec. Changes since 0.1.0 include:

* Basic pseudo-classes.
* Functions return a Result (so they can fail without panic). Note
  that all other error handling is still simply panics.
* Implemented functions `str-slice`, `str-insert`, `str-length`.
* Documentation


## Release 0.1.0 - 2017-01-22

First release to crates.io.  Passes 346 tests of the sass-spec suite.


## Initial commit - 2016-12-20

The first three tests (of several thousand) works.  :-)
