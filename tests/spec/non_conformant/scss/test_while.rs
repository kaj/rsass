//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  $x : true;\
             \n  @while $x {\
             \n    stuff: 1;\
             \n    more-stuff: 2;\
             \n    even-more-stuff: 3;\
             \n    lets-stop-now: 4;\
             \n    $x: false;\
             \n  }\
             \n}"),
        "div {\
         \n  stuff: 1;\
         \n  more-stuff: 2;\
         \n  even-more-stuff: 3;\
         \n  lets-stop-now: 4;\
         \n}\n"
    );
}
