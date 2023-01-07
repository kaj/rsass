//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/011_test_chained_extends.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("011_test_chained_extends")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {@extend .foo}\
             \n.baz {@extend .bar}\
             \n.bip {@extend .bar}\n"),
        ".foo, .bar, .bip, .baz {\
         \n  a: b;\
         \n}\n"
    );
}
