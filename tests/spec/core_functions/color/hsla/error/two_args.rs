//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/two_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: hsla(#123, 0.5);\
             \n}\n"
        ),
        "Error: Missing argument $lightness.\
         \n  ,\
         \n2 |   b: hsla(#123, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
