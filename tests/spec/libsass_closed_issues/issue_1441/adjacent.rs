//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/adjacent.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".adjacent {\
            \n    & + & {\
            \n        foo: bar;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".adjacent + .adjacent {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
