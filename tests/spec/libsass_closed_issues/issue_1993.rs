//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1993.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\
            \n  @extend #{\'%test\'} !optional\
            \n}"
        )
        .unwrap(),
        ""
    );
}
