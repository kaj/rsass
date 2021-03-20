//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2153.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: \"\\\"foo\\\"\" + \"\";\
            \n  b: \"\" + \"\\\"foo\\\"\";\
            \n  c: \"\" + \"\\\"foo\";\
            \n  d: \"\\\"foo\\\"\" + \"bar\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: \'\"foo\"\';\
        \n  b: \'\"foo\"\';\
        \n  c: \'\"foo\';\
        \n  d: \'\"foo\"bar\';\
        \n}\
        \n"
    );
}
