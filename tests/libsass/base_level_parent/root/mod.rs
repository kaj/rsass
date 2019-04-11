//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-alone-itpl.hrx"

// Ignoring "at_root_alone_itpl", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-alone.hrx"

// Ignoring "at_root_alone", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix-itpl.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix.hrx"

// Ignoring "at_root_postfix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix-itpl.hrx"
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

// From "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix.hrx"

// Ignoring "at_root_prefix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/basic-alone-itpl.hrx"

// Ignoring "basic_alone_itpl", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/basic-alone.hrx"

// Ignoring "basic_alone", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl.hrx"
#[test]
fn basic_postfix_itpl() {
    assert_eq!(
        rsass("#{&}post {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "post foo {\n  bar: baz;\n}\n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/root/basic-postfix.hrx"

// Ignoring "basic_postfix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl.hrx"
#[test]
fn basic_prefix_itpl() {
    assert_eq!(
        rsass("pre#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n")
            .unwrap(),
        "pre foo {\n  bar: baz;\n}\n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/root/basic-prefix.hrx"

// Ignoring "basic_prefix", error tests are not supported yet.
