//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/171_test_loud_comment_in_compressed_mode.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("171_test_loud_comment_in_compressed_mode")
}

#[test]
fn test() {
    assert_eq!(runner().ok("/*! foo */\n"), "/*! foo */\n");
}
