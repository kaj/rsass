//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_basic_scss.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_basic_scss")
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
