//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2333.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("test { test: inspect((a:1,b:(foo,bar),c:3)); }")
            .unwrap(),
        "test {\
        \n  test: (a: 1, b: (foo, bar), c: 3);\
        \n}\
        \n"
    );
}
