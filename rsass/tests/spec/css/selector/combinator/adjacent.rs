//! Tests auto-converted from "sass-spec/spec/css/selector/combinator/adjacent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("adjacent")
}

#[test]
#[ignore] // unexepected error
fn function() {
    assert_eq!(
        runner().ok("@use \'sass:selector\';\n\
             \na {\
             \n  b: selector.parse(\"[c]d\");\
             \n}\n"),
        "a {\
         \n  b: [c] d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn stylesheet() {
    assert_eq!(
        runner().ok("[a]b {c: d}\n"),
        "[a] b {\
         \n  c: d;\
         \n}\n"
    );
}
