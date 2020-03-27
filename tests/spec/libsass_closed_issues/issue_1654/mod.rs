//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1654"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1654/basic.hrx"
#[test]
#[ignore] // wrong result
fn basic() {
    assert_eq!(
        rsass(
            "%foo {\
            \n  &bar {\
            \n    display: block;\
            \n  }\
            \n  &.bar {\
            \n    display: block;\
            \n  }\
            \n}\
            \nzoo {\
            \n  @extend %foo;\
            \n}"
        )
        .unwrap(),
        "zoo.bar {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1654/bem.hrx"
#[test]
#[ignore] // wrong result
fn bem() {
    assert_eq!(
        rsass(
            "%foo,\
            \n.foo {\
            \n display:block;\
            \n\
            \n  &--up {\
            \n   border: none;\
            \n }\
            \n}\
            \n\
            \n.zoo {\
            \n  @extend %foo;\
            \n\
            \n  &--up {\
            \n    @extend %foo--up;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".zoo,\
        \n.foo {\
        \n  display: block;\
        \n}\
        \n.zoo--up,\
        \n.foo--up {\
        \n  border: none;\
        \n}\
        \n"
    );
}
