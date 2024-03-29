//! Tests auto-converted from "sass-spec/spec/non_conformant/sass/var-args/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin bar($x, $y, $z) {\
             \n  x: $x;\
             \n  y: $y;\
             \n  z: $z;\
             \n}\n\
             \ndiv {\
             \n  @include bar(a, c d e...);\
             \n}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n    ,\
         \n1   | @mixin bar($x, $y, $z) {\
         \n    |        =============== declaration\
         \n... |\
         \n8   |   @include bar(a, c d e...);\
         \n    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 8:3  bar()\
         \n  input.scss 8:3  root stylesheet",
    );
}
