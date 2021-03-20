//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/031_test_line_comment_in_script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {a: 1 + // flang }\
            \n  blang }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1blang;\
        \n}\
        \n"
    );
}
