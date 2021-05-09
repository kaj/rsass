//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/040_test_newlines_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
