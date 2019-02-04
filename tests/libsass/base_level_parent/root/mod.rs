//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "at-root-alone", tests with expected error not implemented yet.

// Ignoring "at-root-alone-itpl", tests with expected error not implemented yet.

// Ignoring "at-root-postfix", tests with expected error not implemented yet.

// Ignoring "at-root-postfix-itpl", not expected to work yet.

// Ignoring "at-root-prefix", tests with expected error not implemented yet.

// Ignoring "at-root-prefix-itpl", not expected to work yet.

// Ignoring "basic-alone", tests with expected error not implemented yet.

// Ignoring "basic-alone-itpl", tests with expected error not implemented yet.

// Ignoring "basic-postfix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-postfix-itpl"
#[test]
fn basic_postfix_itpl() {
    assert_eq!(
        rsass("#{&}post {\n  foo {\n    bar: baz;\n  }\n}\n").unwrap(),
        "post foo {\n  bar: baz;\n}\n"
    );
}

// Ignoring "basic-prefix", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/base-level-parent/root/basic-prefix-itpl"
#[test]
fn basic_prefix_itpl() {
    assert_eq!(
        rsass("pre#{&} {\n  foo {\n    bar: baz;\n  }\n}\n").unwrap(),
        "pre foo {\n  bar: baz;\n}\n"
    );
}
