//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "at-root-alone", tests with expected error not implemented yet.

// Ignoring "at-root-alone-itpl", tests with expected error not implemented yet.

// Ignoring "at-root-postfix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix-itpl"
#[test]
#[ignore] // failing
fn at_root_postfix_itpl() {
    assert_eq!(
        rsass(
            "@at-root {\r\n  #{&}post {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "post foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "at-root-prefix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix-itpl"
#[test]
#[ignore] // failing
fn at_root_prefix_itpl() {
    assert_eq!(
        rsass(
            "@at-root {\r\n  pre#{&} {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "pre foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-alone", tests with expected error not implemented yet.

// Ignoring "basic-alone-itpl", tests with expected error not implemented yet.

// Ignoring "basic-postfix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl"
#[test]
fn basic_postfix_itpl() {
    assert_eq!(
        rsass("#{&}post {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "post foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-prefix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl"
#[test]
fn basic_prefix_itpl() {
    assert_eq!(
        rsass("pre#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "pre foo {\n  bar: baz;\n}\n"
    );
}
