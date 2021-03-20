//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1224.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all and (max-width: 768px) {\
            \n  @media only screen {\
            \n    a { b: c; }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen and (max-width: 768px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
