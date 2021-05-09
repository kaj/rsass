//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_478.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: \"x\";\
             \n$y: \"y\";\
             \n#{$x}--#{$y} {\
             \n  a: 1\
             \n}\n"),
        "x--y {\
         \n  a: 1;\
         \n}\n"
    );
}
