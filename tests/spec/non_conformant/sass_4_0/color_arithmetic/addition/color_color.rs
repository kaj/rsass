//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/color_arithmetic/addition/color_color.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$v: #abc + #123;\
             \n"
        )
        .unwrap_err(),
        "Error: Undefined operation \"#abc + #123\".\
         \n  ,\
         \n1 | $v: #abc + #123;\
         \n  |     ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
