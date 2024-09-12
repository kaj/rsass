//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/id.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("id")
}

#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"#c\", \"#c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn unequal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"#c\", \"#d\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
