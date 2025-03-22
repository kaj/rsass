//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/01_simple_css.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_simple_css")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  color: blue;\
             \n}"),
        "a {\
         \n  color: blue;\
         \n}\n"
    );
}
