//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/sibling.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".sibling {\
            \n    & ~ & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".sibling ~ .sibling {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
