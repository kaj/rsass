//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "blead-global", not expected to work yet.

// Ignoring "defaults", not expected to work yet.

/// From "sass-spec/spec/libsass/variable-scoping/defaults-global"
#[test]
fn defaults_global() {
    assert_eq!(
        rsass(
            "div {\n  $foo: inner !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n$foo: outer !default !global;\nouter { foo: $foo; }\n\ndiv {\n  $foo: footer !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        ).unwrap(),
        "div inner {\n  foo: lexical;\n}\n\nouter {\n  foo: inner;\n}\n\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/defaults-global-null"
#[test]
fn defaults_global_null() {
    assert_eq!(
        rsass(
            "div {\n  $foo: null !default !global;\n  $foo: inner !default !global;\n  $foo: null !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n$foo: null !default !global;\n$foo: outer !default !global;\n$foo: null !default !global;\nouter { foo: $foo; }\n\ndiv {\n  $foo: null !default !global;\n  $foo: footer !default !global;\n  $foo: null !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        ).unwrap(),
        "div inner {\n  foo: lexical;\n}\n\nouter {\n  foo: inner;\n}\n\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/defaults-null"
#[test]
fn defaults_null() {
    assert_eq!(
        rsass(
            "div {\n  $foo: null !default;\n  $foo: inner !default;\n  $foo: null !default;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n// this should error\n// empty { foo: $foo; }\n\n$foo: null !default;\n$foo: outer !default;\n$foo: null !default;\nouter { foo: $foo; }\n\ndiv {\n  $foo: null !default;\n  $foo: footer !default;\n  $foo: null !default;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        ).unwrap(),
        "div inner {\n  foo: lexical;\n}\n\nouter {\n  foo: outer;\n}\n\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/feature-test"
#[test]
fn feature_test() {
    assert_eq!(
        rsass(
            "@if feature-exists(global-variable-shadowing) {\n  div {\n    feature: true;\n  }\n}"
        ).unwrap(),
        "div {\n  feature: true;\n}\n"
    );
}

// Ignoring "lexical-scope", not expected to work yet.

// Ignoring "root-scope", not expected to work yet.
