//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/ie-backslash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ie-backslash")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  background-color: darken(red, 10%) \\9;\
             \n}"),
        "div {\
         \n  background-color: #cc0000 \\9 ;\
         \n}\n"
    );
}
