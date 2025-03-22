//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/color_arithmetic/addition/number_color.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("number_color")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("$v: 1 + #123;\n"),
        "Error: Undefined operation \"1 + #123\".\
         \n  ,\
         \n1 | $v: 1 + #123;\
         \n  |     ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
