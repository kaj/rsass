//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/selector_interpolation_before_element_name.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_before_element_name")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{\"foo\" + \" bar\"}baz {a: b}\n"),
        "foo barbaz {\
         \n  a: b;\
         \n}\n"
    );
}
