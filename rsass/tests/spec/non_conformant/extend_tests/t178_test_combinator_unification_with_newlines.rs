//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/178_test_combinator_unification_with_newlines.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("178_test_combinator_unification_with_newlines")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a >\
             \n.b\
             \n+ x {a: b}\
             \n.c\
             \n> .d +\
             \ny {@extend x}\n"),
        ".a > .b + x, .a.c > .b.d + y {\
         \n  a: b;\
         \n}\n"
    );
}
