//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/hey1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hey1")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div { width: 1px; }\n"),
        "div {\
         \n  width: 1px;\
         \n}\n"
    );
}
