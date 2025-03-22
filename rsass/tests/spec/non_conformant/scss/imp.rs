//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/imp.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("imp")
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
