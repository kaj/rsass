//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_815.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  foo: str-slice(\"bar\", 1, 2);\
            \n  bar: str-slice(\"bar\", 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: \"ba\";\
        \n  bar: \"r\";\
        \n}\
        \n"
    );
}
