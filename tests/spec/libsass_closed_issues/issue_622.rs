//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_622.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n    a {\
            \n        color: red;\
            \n    }\
            \n}\
            \n\
            \n.link {\
            \n    @media (foo: bar) {\
            \n        display: flex;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media (foo: bar) {\
        \n  .link {\
        \n    display: flex;\
        \n  }\
        \n}\
        \n"
    );
}
