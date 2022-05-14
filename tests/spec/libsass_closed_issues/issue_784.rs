//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_784.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_784")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @each $item in (a: 1, b: 2, c: 3) {\
             \n    each: $item;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  each: a 1;\
         \n  each: b 2;\
         \n  each: c 3;\
         \n}\n"
    );
}
