//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/backslash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div, span {\
             \n\tcolor: red;\
             \n\t\\ foo {\
             \n\t\tcolor: blue;\
             \n\t}\
             \n}"),
        "div, span {\
         \n  color: red;\
         \n}\
         \ndiv \\ foo, span \\ foo {\
         \n  color: blue;\
         \n}\n"
    );
}
