//! These are from the `scope` directory in the sass specification.
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

#[test]
fn clash() {
    check(
        "$foo: 42;\n$bar: 43;\n$baz: 45;\n\n\
         foo {\n  foo: $foo;\n  bar: $bar;\n  baz: $baz;\n\n\
         $bar: 4; // this is a different $bar than `$bar !global`\n\n  \
         foo: $foo;\n  bar: $bar;\n  baz: $baz;\n\n  \
         @if true {\n    \
         $foo: 3; // this is a different $foo than `$foo !global`\n    \
         $bar: 5; // this is a different $bar than `$bar !global`\n\n    \
         foo: $foo;\n    bar: $bar;\n    baz: $baz;\n  }\n}\n",
        "foo {\n  foo: 42;\n  bar: 43;\n  baz: 45;\n  foo: 42;\n  \
         bar: 4;\n  baz: 45;\n  foo: 3;\n  bar: 5;\n  baz: 45;\n}\n",
    )
}

#[test]
fn each() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         @each $item in 1337 {\n  $foo: $item !global;\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn test_for() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         @for $i from 1337 to 1338 {\n  $foo: $i !global;\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn function() {
    check(
        "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n\
         @function foo() {\n  $foo: 1337 !global;\n  @return $foo;\n}\n\n\
         @if foo() {}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn test_if() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         @if true {\n  $foo: 1337 !global;\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn mixin() {
    check(
        "$foo: 42;\n\n\
         @mixin foo {\n  $foo: 1337 !global;\n}\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         @include foo;\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn nested() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n\n  \
         > .bar {\n    $foo: 1337 !global;\n  }\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn root() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         $foo: 1337 !global;\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn ruleset() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n  $foo: 1337 !global;\n  \
         content: $foo;\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n  content: 1337;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

#[test]
fn test_while() {
    check(
        "$foo: 42;\n\n\
         .foo {\n  content: $foo;\n}\n\n\
         $condition: 1337;\n\
         @while $condition > 0 {\n  $foo: $condition !global;\n  \
         $condition: 0;\n}\n\n\
         .bar {\n  content: $foo;\n}\n",
        ".foo {\n  content: 42;\n}\n\n\
         .bar {\n  content: 1337;\n}\n",
    )
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_bytes(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
