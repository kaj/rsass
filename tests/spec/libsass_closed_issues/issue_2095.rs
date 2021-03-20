//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2095.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all {\
            \n  @mixin foo() {}\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
