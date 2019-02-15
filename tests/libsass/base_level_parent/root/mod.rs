//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "at-root-alone-itpl.hrx", error tests are not supported yet.

// Ignoring "at-root-alone.hrx", error tests are not supported yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix-itpl.hrx"
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

// Ignoring "at-root-postfix.hrx", error tests are not supported yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix-itpl.hrx"
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

// Ignoring "at-root-prefix.hrx", error tests are not supported yet.

// Ignoring "basic-alone-itpl.hrx", error tests are not supported yet.

// Ignoring "basic-alone.hrx", error tests are not supported yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl.hrx"
#[test]
fn basic_postfix_itpl() {
    assert_eq!(
        rsass("#{&}post {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "post foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-postfix.hrx", error tests are not supported yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl.hrx"
#[test]
fn basic_prefix_itpl() {
    assert_eq!(
        rsass("pre#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "pre foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-prefix.hrx", error tests are not supported yet.
