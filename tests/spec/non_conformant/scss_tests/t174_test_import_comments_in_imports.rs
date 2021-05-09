//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/174_test_import_comments_in_imports.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"foo.css\", // this is a comment\
             \n        \"bar.css\", /* this is another comment */\
             \n        \"baz.css\"; // this is a third comment\n"),
        "@import \"foo.css\";\
         \n@import \"bar.css\";\
         \n@import \"baz.css\";\n"
    );
}
