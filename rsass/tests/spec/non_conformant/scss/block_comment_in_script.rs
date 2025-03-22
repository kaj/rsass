//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/block_comment_in_script.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("block_comment_in_script")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {a: 1 + /* flang */ bar}\n"),
        "foo {\
         \n  a: 1bar;\
         \n}\n"
    );
}
