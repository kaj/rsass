//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2116.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return if(& != null, green, red);\
            \n}\
            \n\
            \ntest {\
            \n  color: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  color: green;\
        \n}\
        \n"
    );
}
