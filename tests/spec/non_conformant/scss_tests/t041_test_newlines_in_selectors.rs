//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/041_test_newlines_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo,\
             \nbar {\
             \n  baz,\
             \n  bang {a: b}}\n"),
        "foo baz,\
         \nfoo bang,\
         \nbar baz,\
         \nbar bang {\
         \n  a: b;\
         \n}\n"
    );
}
