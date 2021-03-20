//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/one_line_comments.hrx"

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
