//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1482.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".mango {\
            \n  color: red;\
            \n}\
            \n\
            \ntype {\
            \n  &__else {\
            \n    @extend .mango;\
            \n  }\
            \n}\
            \n\
            \n.qualified {\
            \n  &__else {\
            \n    @extend .mango;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".mango, .qualified__else, type__else {\
        \n  color: red;\
        \n}\
        \n"
    );
}
