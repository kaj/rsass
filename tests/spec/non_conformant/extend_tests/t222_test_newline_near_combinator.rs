//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/222_test_newline_near_combinator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a +\
             \n.b x {a: b}\
             \n.c y {@extend x}\n"),
        ".a +\
         \n.b x, .a +\
         \n.b .c y, .c .a +\
         \n.b y {\
         \n  a: b;\
         \n}\n"
    );
}
