//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/220_test_extend_in_double_nested_media_query.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("220_test_extend_in_double_nested_media_query")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media all {\
             \n@media (orientation: landscape) {\
             \n  %foo {color: blue}\
             \n  .bar {@extend %foo}\
             \n}\
             \n}\n"),
        "@media (orientation: landscape) {\
         \n  .bar {\
         \n    color: blue;\
         \n  }\
         \n}\n"
    );
}
