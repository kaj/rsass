//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/190_test_options_passed_to_script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("190_test_options_passed_to_script")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {color: color.adjust(black, $lightness: -10%)}\n"),
        "foo {\
         \n  color: hsl(0, 0%, -10%);\
         \n}\n"
    );
}
