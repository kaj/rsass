//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod blead_global;

/// From "sass-spec/spec/libsass/variable-scoping/defaults-global-null.hrx"
#[test]
fn defaults_global_null() {
    assert_eq!(
        rsass(
            "div {\n  $foo: null !default !global;\n  $foo: inner !default !global;\n  $foo: null !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n$foo: null !default !global;\n$foo: outer !default !global;\n$foo: null !default !global;\nouter { foo: $foo; }\n\ndiv {\n  $foo: null !default !global;\n  $foo: footer !default !global;\n  $foo: null !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        )
        .unwrap(),
        "div inner {\n  foo: lexical;\n}\nouter {\n  foo: inner;\n}\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/defaults-global.hrx"
#[test]
fn defaults_global() {
    assert_eq!(
        rsass(
            "div {\n  $foo: inner !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n$foo: outer !default !global;\nouter { foo: $foo; }\n\ndiv {\n  $foo: footer !default !global;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        )
        .unwrap(),
        "div inner {\n  foo: lexical;\n}\nouter {\n  foo: inner;\n}\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/defaults-null.hrx"
#[test]
fn defaults_null() {
    assert_eq!(
        rsass(
            "div {\n  $foo: null !default;\n  $foo: inner !default;\n  $foo: null !default;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n\n// this should error\n// empty { foo: $foo; }\n\n$foo: null !default;\n$foo: outer !default;\n$foo: null !default;\nouter { foo: $foo; }\n\ndiv {\n  $foo: null !default;\n  $foo: footer !default;\n  $foo: null !default;\n  $foo: lexical;\n  inner { foo: $foo; }\n}\n"
        )
        .unwrap(),
        "div inner {\n  foo: lexical;\n}\nouter {\n  foo: outer;\n}\ndiv inner {\n  foo: lexical;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/defaults.hrx"
#[test]
#[ignore] // failing
fn defaults() {
    assert_eq!(
        rsass(
            "$i: 9;\n$z: 3 !default;\ndiv {\n  asd: $i;\n  $i: 99 !global;\n  $n: 42 !global;\n  qwe: $i;\n  zapf: $z;\n  $z: 84;\n  ding: $z;\n}\ndiv {\n  foo: $n;\n  foo: $i;\n  $i: 999;\n  $n: 999;\n  foo: $n;\n  foo: $i;\n  div {\n    $i: 9999;\n    $n: 9999 !default;\n    bar: $i;\n    bar: $n;\n  }\n  baz: $i;\n}\ndiv {\n  asd: $i;\n  qwe: $n;\n  zap: $z;\n}"
        )
        .unwrap(),
        "div {\n  asd: 9;\n  qwe: 99;\n  zapf: 3;\n  ding: 84;\n}\ndiv {\n  foo: 42;\n  foo: 99;\n  foo: 999;\n  foo: 999;\n  baz: 9999;\n}\ndiv div {\n  bar: 9999;\n  bar: 999;\n}\ndiv {\n  asd: 99;\n  qwe: 42;\n  zap: 3;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/feature-test.hrx"
#[test]
fn feature_test() {
    assert_eq!(
        rsass(
            "@if feature-exists(global-variable-shadowing) {\n  div {\n    feature: true;\n  }\n}"
        )
        .unwrap(),
        "div {\n  feature: true;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/lexical-scope.hrx"
#[test]
#[ignore] // failing
fn lexical_scope() {
    assert_eq!(
        rsass(
            "$x: -42;\n$y: -84;\ndiv {\n  $x: 42;\n  $y: 84;\n  for {\n    @for $x from 1 through 5 {\n      $y: $y + 5;\n      y: $y;\n      x: $x;\n      $x: 999;\n      x: $x;\n      $y: -9 !global;\n      $x: -9 !global;\n    }\n  }\n  x: $x;\n  y: $y;\n}"
        )
        .unwrap(),
        "div {\n  x: 42;\n  y: 109;\n}\ndiv for {\n  y: 89;\n  x: 1;\n  x: 999;\n  y: 94;\n  x: 2;\n  x: 999;\n  y: 99;\n  x: 3;\n  x: 999;\n  y: 104;\n  x: 4;\n  x: 999;\n  y: 109;\n  x: 5;\n  x: 999;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/root-scope.hrx"
#[test]
#[ignore] // failing
fn root_scope() {
    assert_eq!(
        rsass(
            "$x: -42;\n$y: -84;\ndiv {\n  x: $x;\n  y: $y;\n  for {\n    x: $x;\n    y: $y;\n    @for $x from 1 through 5 {\n      $y: $y + 5;\n      x: $x;\n      y: $y;\n      $x: 999;\n      $y: -9 !global;\n      $x: -9 !global;\n    }\n    x: $x;\n    y: $y;\n  }\n  x: $x;\n  y: $y;\n}"
        )
        .unwrap(),
        "div {\n  x: -42;\n  y: -84;\n  x: -9;\n  y: -9;\n}\ndiv for {\n  x: -42;\n  y: -84;\n  x: 1;\n  y: -79;\n  x: 2;\n  y: -74;\n  x: 3;\n  y: -69;\n  x: 4;\n  y: -64;\n  x: 5;\n  y: -59;\n  x: -9;\n  y: -9;\n}\n"
    );
}
