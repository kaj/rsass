//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/hey2.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div { color: red; }\n"),
        "div {\
         \n  color: red;\
         \n}\n"
    );
}
