//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/block_comment_in_script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {a: 1 + /* flang */ bar}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1bar;\
        \n}\
        \n"
    );
}
