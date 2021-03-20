//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1304.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    a:&;\
            \n    > bar {\
            \n        b:&;\
            \n        > baz {\
            \n            c:&;\
            \n        }\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: foo;\
        \n}\
        \nfoo > bar {\
        \n  b: foo > bar;\
        \n}\
        \nfoo > bar > baz {\
        \n  c: foo > bar > baz;\
        \n}\
        \n"
    );
}
