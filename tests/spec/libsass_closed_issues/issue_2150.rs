//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2150.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (min-width: 100px) {\
            \n  .parent > %child {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @extend %child;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 100px) {\
        \n  .parent > .foo {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}
