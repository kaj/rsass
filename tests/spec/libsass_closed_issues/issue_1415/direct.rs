//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415/direct.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if & {\
            \n  foo {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
