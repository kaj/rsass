//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/important.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("important")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  color: red ! important;\
             \n  width: 5px ! important;\
             \n}"),
        "div {\
         \n  color: red !important;\
         \n  width: 5px !important;\
         \n}\n"
    );
}
