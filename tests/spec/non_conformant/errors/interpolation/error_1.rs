//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/interpolation/error-1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("\'#{)\'{\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | \'#{)\'{\
         \n  |  ^^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
