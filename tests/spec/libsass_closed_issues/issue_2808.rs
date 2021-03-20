//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2808.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n  content: str-slice(abcdef, -10, 2)\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: ab;\
        \n}\
        \n"
    );
}
