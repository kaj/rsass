//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/functions-and-mixins.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("functions-and-mixins")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  @return \"hello\";\
             \n}\n\
             \n@mixin foo() {\
             \n  content: \"hello\";\
             \n}\n\
             \ndiv {\
             \n  span {\
             \n    @function length($a, $b, $c, $d) {\
             \n      @return $a + $b + $c + $d;\
             \n    }\n\
             \n    div {\
             \n      content: foo();\
             \n      @include foo();\
             \n      width: length(1,2,2,3);\
             \n    }\
             \n  }\n\
             \n  height: length(a b c d e);\n\
             \n}"),
        "div span div {\
         \n  content: \"hello\";\
         \n  content: \"hello\";\
         \n  width: 8;\
         \n}\
         \ndiv {\
         \n  height: 5;\
         \n}\n"
    );
}
