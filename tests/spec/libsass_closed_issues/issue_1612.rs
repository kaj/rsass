//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1612.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "c {\
            \n  b: \"foo\", bar;\
            \n  b: \"foo\", bar\
            \n}\
            \n"
        )
        .unwrap(),
        "c {\
        \n  b: \"foo\", bar;\
        \n  b: \"foo\", bar;\
        \n}\
        \n"
    );
}
