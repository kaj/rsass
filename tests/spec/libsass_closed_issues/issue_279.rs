//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_279.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".theme {\
            \n  @import \"foo.scss\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".theme .test-hello, .theme .test-world {\
        \n  color: red;\
        \n}\
        \n"
    );
}
