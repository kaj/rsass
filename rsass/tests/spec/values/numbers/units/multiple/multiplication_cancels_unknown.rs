//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/multiplication_cancels_unknown.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiplication_cancels_unknown")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n// Units cancel even if they\'re totally unknown to Sass.\
             \n$number: 1foo * 1bar / 1baz / 1qux;\
             \na {\
             \n  b: meta.inspect($number * 1baz);\
             \n}\n"),
        "a {\
         \n  b: calc(1foo * 1bar / 1qux);\
         \n}\n"
    );
}
