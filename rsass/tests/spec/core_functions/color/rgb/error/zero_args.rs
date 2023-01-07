//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/zero_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("zero_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: rgb();\
             \n}\n"
        ),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: rgb();\
         \n  |      ^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgb($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
