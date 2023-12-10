//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/division_cancels_unknown.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("division_cancels_unknown")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// Units cancel even if they\'re totally unknown to Sass.\
             \n$number: 1foo * 1bar / 1baz / 1qux;\
             \na {\
             \n  b: inspect($number / 1foo);\
             \n}\n"
        ),
        "a {\
         \n  b: calc(1bar / 1baz / 1qux);\
         \n}\n"
    );
}
