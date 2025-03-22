//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while_in_functions.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while_in_functions")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function test-while() {\
             \n  $x : true;\
             \n  @while $x {\
             \n    @return $x\
             \n  }\
             \n}\n\
             \ndiv {\
             \n  y: test-while();\
             \n}"),
        "div {\
         \n  y: true;\
         \n}\n"
    );
}
