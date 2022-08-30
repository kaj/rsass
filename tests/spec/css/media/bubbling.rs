//! Tests auto-converted from "sass-spec/spec/css/media/bubbling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bubbling")
}

#[test]
#[ignore] // wrong result
fn unmergeable_and_merged() {
    assert_eq!(
        runner().ok(
            "// Regression test for sass/sass#3384. In Dart Sass, this caused a bug because\
             \n// we were asking \"has this media query been merged?\" to determine whether to\
             \n// bubble it up through parent queries, disregarding whether it had been merged\
             \n// with *that particular query*.\
             \n@media not a {\
             \n  @media (b) {\
             \n    @media (c) {\
             \n      d {e: f}\
             \n    }\
             \n  }\
             \n}\n"
        ),
        "@media not a {\
         \n  @media (b) and (c) {\
         \n    d {\
         \n      e: f;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
