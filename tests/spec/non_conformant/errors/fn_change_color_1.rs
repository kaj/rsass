//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-change-color-1.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\r\
             \n  test: change-color(red, $red: 0.5, $hue: 0.2);\r\
             \n}"
        )
        .unwrap_err(),
        "Error: RGB parameters may not be passed along with HSL parameters.\
         \n  ,\
         \n2 |   test: change-color(red, $red: 0.5, $hue: 0.2);\
         \n  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:9  root stylesheet\
         \n",
    );
}
