//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-change-color-1.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("fn-change-color-1")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \nfoo {\r\
             \n  test: color.change(red, $red: 0.5, $hue: 0.2);\r\
             \n}"
        ),
        "Error: $hue: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n3 |   test: color.change(red, $red: 0.5, $hue: 0.2);\
         \n  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
