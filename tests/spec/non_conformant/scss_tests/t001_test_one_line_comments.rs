//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/001_test_one_line_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {// bar: baz;}\
            \n  baz: bang; //}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  baz: bang;\
        \n}\
        \n"
    );
}
