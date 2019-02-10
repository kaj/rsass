//! Tests auto-converted from "sass-spec/spec/scope"
//! version dd3a5edf, 2019-02-04 13:14:26 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/scope/clash"
#[test]
fn clash() {
    assert_eq!(
        rsass(
            "$foo: 42;\n$bar: 43;\n$baz: 45;\n\nfoo {\n  foo: $foo;\n  bar: $bar;\n  baz: $baz;\n\n  $bar: 4; // this is a different $bar than `$bar !global`\n\n  foo: $foo;\n  bar: $bar;\n  baz: $baz;\n\n  @if true {\n    $foo: 3; // this is a different $foo than `$foo !global`\n    $bar: 5; // this is a different $bar than `$bar !global`\n\n    foo: $foo;\n    bar: $bar;\n    baz: $baz;\n  }\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: 42;\n  bar: 43;\n  baz: 45;\n  foo: 42;\n  bar: 4;\n  baz: 45;\n  foo: 3;\n  bar: 5;\n  baz: 45;\n}\n"
    );
}

/// From "sass-spec/spec/scope/each"
#[test]
fn each() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n@each $item in 1337 {\n  $foo: $item !global;\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/for"
#[test]
fn test_for() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n@for $i from 1337 to 1338 {\n  $foo: $i !global;\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/function"
#[test]
fn function() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n@function foo() {\n  $foo: 1337 !global;\n  @return $foo;\n}\n\n@if foo() {}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/if"
#[test]
fn test_if() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n@if true {\n  $foo: 1337 !global;\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/mixin"
#[test]
fn mixin() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n@mixin foo {\n  $foo: 1337 !global;\n}\n\n.foo {\n  content: $foo;\n}\n\n@include foo;\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/nested"
#[test]
fn nested() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n\n  > .bar {\n    $foo: 1337 !global;\n  }\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/root"
#[test]
fn root() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n$foo: 1337 !global;\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/ruleset"
#[test]
fn ruleset() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n  $foo: 1337 !global;\n  content: $foo;\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n  content: 1337;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

/// From "sass-spec/spec/scope/while"
#[test]
fn test_while() {
    assert_eq!(
        rsass(
            "$foo: 42;\n\n.foo {\n  content: $foo;\n}\n\n$condition: 1337;\n@while $condition > 0 {\n  $foo: $condition !global;\n  $condition: 0;\n}\n\n.bar {\n  content: $foo;\n}\n"
        )
        .unwrap(),
        ".foo {\n  content: 42;\n}\n.bar {\n  content: 1337;\n}\n"
    );
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
