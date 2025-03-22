//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/for_in_functions.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for_in_functions")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n\t$limit: 10;\
             \n\t$y: 0;\
             \n\t@for $x from 1 through $limit {\
             \n\t  $limit: 4;\
             \n\t  $y: $y + $x;\
             \n\t}\
             \n\t@return $y;\
             \n}\n\
             \ndiv {\
             \n\twidth: foo();\
             \n}"),
        "div {\
         \n  width: 55;\
         \n}\n"
    );
}
