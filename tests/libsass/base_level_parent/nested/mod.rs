//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::precision;

/// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-alone"
#[test]
fn at_root_alone() {
    assert_eq!(
        rsass(
            "test {\r\n  @at-root {\r\n    & {\r\n      foo {\r\n        bar: baz;\r\n      }\r\n    }\r\n  }\r\n}"
        )
        .unwrap(),
        "test foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "at-root-alone-itpl", not expected to work yet.

/// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-postfix"
#[test]
fn at_root_postfix() {
    assert_eq!(
        rsass(
            "test {\r\n  @at-root {\r\n    &post {\r\n      foo {\r\n        bar: baz;\r\n      }\r\n    }\r\n  }\r\n}"
        )
        .unwrap(),
        "testpost foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "at-root-postfix-itpl", not expected to work yet.

// Ignoring "at-root-prefix", tests with expected error not implemented yet.

// Ignoring "at-root-prefix-itpl", not expected to work yet.

/// From "sass-spec/spec/libsass/base-level-parent/nested/basic-alone"
#[test]
fn basic_alone() {
    assert_eq!(
        rsass(
            "test {\r\n  & {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}"
        )
        .unwrap(),
        "test foo {\n  bar: baz;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/base-level-parent/nested/basic-alone-itpl"
#[test]
fn basic_alone_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  #{&} {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "test test foo {\n  bar: baz;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix"
#[test]
fn basic_postfix() {
    assert_eq!(
        rsass(
            "test {\r\n  &post {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}"
        )
        .unwrap(),
        "testpost foo {\n  bar: baz;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix-itpl"
#[test]
fn basic_postfix_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  #{&}post {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "test testpost foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-prefix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/nested/basic-prefix-itpl"
#[test]
fn basic_prefix_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  pre#{&} {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "test pretest foo {\n  bar: baz;\n}\n"
    );
}
