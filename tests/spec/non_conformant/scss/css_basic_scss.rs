//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_basic_scss.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("sel {\
             \n  p: v; }\n"),
        "sel {\
         \n  p: v;\
         \n}\n"
    );
}
