//! Tests auto-converted from "sass-spec/spec/libsass/css_nth_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_nth_selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(":nth-child(2n + 3) {\
             \n  outer-whitespace: false;\
             \n}\n\
             \n// Regression test for sass/dart-sass#465.\
             \n:nth-child( 2n + 3 ) {\
             \n  outer-whitespace: true;\
             \n}\n"),
        ":nth-child(2n+3) {\
         \n  outer-whitespace: false;\
         \n}\
         \n:nth-child(2n+3) {\
         \n  outer-whitespace: true;\
         \n}\n"
    );
}
