//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inline")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: #{(2px*5in)};\
             \n}\n"),
        "foo {\
         \n  bar: calc(10px * 1in);\
         \n}\n"
    );
}
