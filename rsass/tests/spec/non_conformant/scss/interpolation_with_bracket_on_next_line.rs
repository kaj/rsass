//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolation_with_bracket_on_next_line.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolation_with_bracket_on_next_line")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a.#{\"foo\"} b\
             \n{color: red}\n"),
        "a.foo b {\
         \n  color: red;\
         \n}\n"
    );
}
