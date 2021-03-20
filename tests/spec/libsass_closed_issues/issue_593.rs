//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_593.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "h1:nth-of-type(#{2 + \'n + 1\'}) {\
            \n    color: red;\
            \n}\
            \n\
            \nh1:nth-of-type(#{2 + \'n   +  1\'}) {\
            \n    color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "h1:nth-of-type(2n + 1) {\
        \n  color: red;\
        \n}\
        \nh1:nth-of-type(2n + 1) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
