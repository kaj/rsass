//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/181_test_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("181_test_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$bar : \"#foo\";\
             \nul li#{$bar} a span.label { foo: bar; }\n"),
        "ul li#foo a span.label {\
         \n  foo: bar;\
         \n}\n"
    );
}
