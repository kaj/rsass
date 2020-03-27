//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone-itpl.hrx"

// Ignoring "at_root_alone_itpl", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone.hrx"

// Ignoring "at_root_alone", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-postfix-itpl.hrx"
#[test]
#[ignore] // wrong result
fn at_root_postfix_itpl() {
    assert_eq!(
        rsass("@import \"include.scss\";").unwrap(),
        "post foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-postfix.hrx"

// Ignoring "at_root_postfix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-prefix-itpl.hrx"
#[test]
#[ignore] // wrong result
fn at_root_prefix_itpl() {
    assert_eq!(
        rsass("@import \"include.scss\";").unwrap(),
        "pre foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/imported/at-root-prefix.hrx"

// Ignoring "at_root_prefix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-alone-itpl.hrx"

// Ignoring "basic_alone_itpl", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-alone.hrx"

// Ignoring "basic_alone", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix-itpl.hrx"
#[test]
#[ignore] // wrong result
fn basic_postfix_itpl() {
    assert_eq!(
        rsass("@import \"include.scss\";").unwrap(),
        "post foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix.hrx"

// Ignoring "basic_postfix", error tests are not supported yet.

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-prefix-itpl.hrx"
#[test]
#[ignore] // wrong result
fn basic_prefix_itpl() {
    assert_eq!(
        rsass("@import \"include.scss\";").unwrap(),
        "pre foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/base-level-parent/imported/basic-prefix.hrx"

// Ignoring "basic_prefix", error tests are not supported yet.
