//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/002_test_one_line_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo bar[val=\"//\"] {\
            \n  baz: bang; //}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo bar[val=\"//\"] {\
        \n  baz: bang;\
        \n}\
        \n"
    );
}
