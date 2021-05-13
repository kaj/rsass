//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
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
         \n2 |   {\
         \n  |   ^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
