//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/functions-and-mixins.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return \"hello\";\
            \n}\
            \n\
            \n@mixin foo() {\
            \n  content: \"hello\";\
            \n}\
            \n\
            \ndiv {\
            \n  span {\
            \n    @function length($a, $b, $c, $d) {\
            \n      @return $a + $b + $c + $d;\
            \n    }\
            \n\
            \n    div {\
            \n      content: foo();\
            \n      @include foo();\
            \n      width: length(1,2,2,3);\
            \n    }\
            \n  }\
            \n\
            \n  height: length(a b c d e);\
            \n\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  height: 5;\
        \n}\
        \ndiv span div {\
        \n  content: \"hello\";\
        \n  content: \"hello\";\
        \n  width: 8;\
        \n}\
        \n"
    );
}
