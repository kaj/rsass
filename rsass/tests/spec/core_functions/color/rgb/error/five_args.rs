//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/five_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("five_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb(1, 2, 3, 0.4, 5);\
             \n}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 |   b: rgb(1, 2, 3, 0.4, 5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgb($red, $green, $blue, $alpha) {\
         \n  |           ================================ declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
