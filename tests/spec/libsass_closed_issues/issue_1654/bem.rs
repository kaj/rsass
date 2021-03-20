//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1654/bem.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
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
