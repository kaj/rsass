//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$bar : \"#foo\";\n\n\n\
             \nul li#{$bar} a span.label { foo: bar; }\n"),
        "ul li#foo a span.label {\
         \n  foo: bar;\
         \n}\n"
    );
}
