//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1133/vararg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($this, $my) {\
             \n  @return (this: $this, my: $my);\
             \n}\n\
             \na {\
             \n  $map: foo((this: is, my: map)...);\
             \n  @each $k, $v in $map {\
             \n    #{$k}: $v;\
             \n  }\
             \n}\n\
             \nb {\
             \n  $map: call(\"foo\", (this: is, my: map)...);\
             \n  @each $k, $v in $map {\
             \n    #{$k}: $v;\
             \n  }\
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
