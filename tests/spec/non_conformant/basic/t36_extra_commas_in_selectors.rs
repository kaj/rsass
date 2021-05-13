//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/36_extra_commas_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div,, , span, ,, {\
             \n  color: red;\
             \n}"),
        "div, span {\
         \n  color: red;\
         \n}\n"
    );
}
