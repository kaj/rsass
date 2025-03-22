//! Tests auto-converted from "sass-spec/spec/values/identifiers/if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass#1405.\
             \na {b: if}\n"),
        "a {\
         \n  b: if;\
         \n}\n"
    );
}
