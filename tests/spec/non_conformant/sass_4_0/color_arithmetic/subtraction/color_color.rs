//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/color_arithmetic/subtraction/color_color.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("color_color")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("$v: #abc - #123;\n"),
        "Error: Undefined operation \"#abc - #123\".\
         \n  ,\
         \n1 | $v: #abc - #123;\
         \n  |     ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
