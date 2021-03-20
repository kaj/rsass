//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1907.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: \'test\' + \'1 #{2} 3\';\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"test1 2 3\";\
        \n}\
        \n"
    );
}
