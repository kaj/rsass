//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/171_test_loud_comment_in_compressed_mode.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("/*! foo */\n"), "/*! foo */\n");
}
