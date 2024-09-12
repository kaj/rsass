//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1133/normal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("normal")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function foo($map) {\
             \n    @return $map;\
             \n}\n\
             \na {\
             \n    $map: foo((this: is, my: map));\
             \n    @each $k, $v in $map {\
             \n        #{$k}: $v;\
             \n    }\
             \n}\n\
             \nb {\
             \n    $map: meta.call(\"foo\", (this: is, my: map));\
             \n    @each $k, $v in $map {\
             \n        #{$k}: $v;\
             \n    }\
             \n}\n"),
        "a {\
         \n  this: is;\
         \n  my: map;\
         \n}\
         \nb {\
         \n  this: is;\
         \n  my: map;\
         \n}\n"
    );
}
