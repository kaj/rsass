//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/color_arithmetic/subtraction/color_number.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$v: #abc - 1;\
             \n"
        )
        .unwrap_err(),
        "Error: Undefined operation \"#abc - 1\".\
         \n  ,\
         \n1 | $v: #abc - 1;\
         \n  |     ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet\
         \n",
    );
}
