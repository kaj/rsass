//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/02_simple_nesting.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  img {\
             \n    border: 0px;\
             \n  }\
             \n}"),
        "div img {\
         \n  border: 0px;\
         \n}\n"
    );
}
