//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2321.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: if(true, b, c...);\
            \n  c: if(false, b, c...);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b;\
        \n  c: c;\
        \n}\
        \n"
    );
}
