//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/five_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: hsla(0, 100%, 50%, 0.5, 0);\
             \n}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 |   b: hsla(0, 100%, 50%, 0.5, 0);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hsla($hue, $saturation, $lightness, $alpha) {\
         \n  |           =========================================== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
