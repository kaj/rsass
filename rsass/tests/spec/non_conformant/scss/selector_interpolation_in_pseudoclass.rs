//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_in_pseudoclass.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_in_pseudoclass")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo:nth-child(#{5 + \"n\"}) {a: b}\n"),
        "foo:nth-child(5n) {\
         \n  a: b;\
         \n}\n"
    );
}
