//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_549.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$value: 10;\
            \n\
            \nfoo {\
            \n  filter: foo(opacity=$value*100);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: foo(opacity=1000);\
        \n}\
        \n"
    );
}
