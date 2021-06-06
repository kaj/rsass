//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/zero_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: rgba();\
             \n}\n"
        ),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: rgba();\
         \n  |      ^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgba($channels) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
