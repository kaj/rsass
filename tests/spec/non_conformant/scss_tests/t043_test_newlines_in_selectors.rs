//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/043_test_newlines_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo, bar\
             \nbaz {\
             \n  bang, bip\
             \n  bop {a: b}}\n"),
        "foo bang, foo bip\
         \nbop, bar\
         \nbaz bang, bar\
         \nbaz bip\
         \nbop {\
         \n  a: b;\
         \n}\n"
    );
}
