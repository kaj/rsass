//! Tests auto-converted from "sass-spec/spec/values/identifiers/if.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
