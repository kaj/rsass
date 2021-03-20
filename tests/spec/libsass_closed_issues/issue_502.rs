//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_502.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: 1;;\
            \n;;\
            \n"
        )
        .unwrap(),
        ""
    );
}
