//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1629.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  background: url(...) 2rem 3rem / auto 2rem;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  background: url(...) 2rem 3rem/auto 2rem;\
        \n}\
        \n"
    );
}
