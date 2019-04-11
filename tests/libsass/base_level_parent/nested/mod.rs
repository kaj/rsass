//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-alone-itpl.hrx"
#[test]
#[ignore] // failing
fn at_root_alone_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  @at-root {\r\n    #{&} {\r\n      foo {\r\n        bar: baz;\r\n      }\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "test foo {\n  bar: baz;\n}\n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-alone.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-postfix-itpl.hrx"
#[test]
#[ignore] // failing
fn at_root_postfix_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  @at-root {\r\n    #{&}post {\r\n      foo {\r\n        bar: baz;\r\n      }\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "testpost foo {\n  bar: baz;\n}\n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-postfix.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-prefix-itpl.hrx"
#[test]
#[ignore] // failing
fn at_root_prefix_itpl() {
    assert_eq!(
        rsass(
            "test {\r\n  @at-root {\r\n    pre#{&} {\r\n      foo {\r\n        bar: baz;\r\n      }\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "pretest foo {\n  bar: baz;\n}\n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/nested/at-root-prefix.hrx"

// Ignoring "at_root_prefix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-alone-itpl.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-alone.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix-itpl.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-postfix.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-prefix-itpl.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/nested/basic-prefix.hrx"

// Ignoring "basic_prefix", error tests are not supported yet.
