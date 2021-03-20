//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1710.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "ul, ol {\
            \n    & & {\
            \n      display: block;\
            \n    }\
            \n  }\
            \n"
        )
        .unwrap(),
        "ul ul, ul ol, ol ul, ol ol {\
        \n  display: block;\
        \n}\
        \n"
    );
}
