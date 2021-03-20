//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1297.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".test .testa {\
            \n  @at-root #{\"%foo\"} {\
            \n    color: red;\
            \n  }\
            \n  @extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test .testa {\
        \n  color: red;\
        \n}\
        \n"
    );
}
