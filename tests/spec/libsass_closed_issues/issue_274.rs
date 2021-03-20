//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_274.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "input[type=submit],\r\
            \ninput[type=reset],\r\
            \ninput[type=button]\r\
            \n{\r\
            \n       filter:chroma(color=#000000);\r\
            \n}"
        )
        .unwrap(),
        "input[type=submit],\
        \ninput[type=reset],\
        \ninput[type=button] {\
        \n  filter: chroma(color=#000000);\
        \n}\
        \n"
    );
}
