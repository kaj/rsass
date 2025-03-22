//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/color_arithmetic/modulo/color_number.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("color_number")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("$v: #abc % 1;\n"),
        "Error: Undefined operation \"#abc % 1\".\
         \n  ,\
         \n1 | $v: #abc % 1;\
         \n  |     ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
