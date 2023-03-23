//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-alone-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone-itpl")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "#{&} {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}\r\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n1 | #{&} {\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
