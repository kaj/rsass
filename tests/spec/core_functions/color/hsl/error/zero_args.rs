//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/zero_args.hrx"

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
             \n  b: hsl();\
             \n}\n"
        ),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: hsl();\
         \n  |      ^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hsl($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
