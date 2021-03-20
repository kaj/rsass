//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/174_test_import_comments_in_imports.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"foo.css\", // this is a comment\
            \n        \"bar.css\", /* this is another comment */\
            \n        \"baz.css\"; // this is a third comment\
            \n"
        )
        .unwrap(),
        "@import \"foo.css\";\
        \n@import \"bar.css\";\
        \n@import \"baz.css\";\
        \n"
    );
}
