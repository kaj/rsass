//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/040_test_newlines_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("040_test_newlines_in_selectors")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo\
             \nbar {a: b}\n"),
        "foo\
         \nbar {\
         \n  a: b;\
         \n}\n"
    );
}
