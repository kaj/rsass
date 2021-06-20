//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-change-color-1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "foo {\r\
             \n  test: change-color(red, $red: 0.5, $hue: 0.2);\r\
             \n}"
        ),
        "Error: RGB parameters may not be passed along with HSL parameters.\
         \n  ,\
         \n2 |   test: change-color(red, $red: 0.5, $hue: 0.2);\
         \n  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:9  root stylesheet",
    );
}
