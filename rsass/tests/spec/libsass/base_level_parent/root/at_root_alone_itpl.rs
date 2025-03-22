//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-alone-itpl")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@at-root {\r\
             \n  #{&} {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n2 |   #{&} {\
         \n  |        ^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
