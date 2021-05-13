//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/five_args.hrx"

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
             \n  b: rgba(1, 2, 3, 0.4, 5);\
             \n}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 |   b: rgba(1, 2, 3, 0.4, 5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgba($red, $green, $blue, $alpha) {\
         \n  |           ================================= declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
