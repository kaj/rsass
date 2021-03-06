//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1683"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1683/function.hrx"
#[test]
#[ignore] // unexepected error
fn function() {
    assert_eq!(
        rsass(
            "@function foo($x, $y) { @return null }\
            \n\
            \na {\
            \n  b: foo(1 2 3...);\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1683/mixin.hrx"
#[test]
#[ignore] // unexepected error
fn mixin() {
    assert_eq!(
        rsass(
            "@mixin foo($x, $y) { }\
            \n\
            \na {\
            \n  @include foo(1 2 3...);\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
