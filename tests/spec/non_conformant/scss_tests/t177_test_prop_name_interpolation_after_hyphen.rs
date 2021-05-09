//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/177_test_prop_name_interpolation_after_hyphen.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a { -#{\"foo\"}-bar: b; }\n"),
        "a {\
         \n  -foo-bar: b;\
         \n}\n"
    );
}
