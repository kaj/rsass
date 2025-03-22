//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/two_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("two_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: hsl(#123, 0.5);\
             \n}\n"
        ),
        "Error: Missing argument $lightness.\
         \n  ,\
         \n2 |   b: hsl(#123, 0.5);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
