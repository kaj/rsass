//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/newlines_in_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("newlines_in_selectors")
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
