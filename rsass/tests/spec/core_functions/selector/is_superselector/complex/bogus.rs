//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/complex/bogus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bogus")
}

#[test]
#[ignore] // wrong result
fn sub() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"c\", \"d + ~ c\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn test_super() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"> c\", \"c\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
