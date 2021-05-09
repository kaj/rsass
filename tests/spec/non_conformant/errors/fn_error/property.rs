//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/property.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a {\r\
             \n  b: {\r\
             \n    @error \"error\";\r\
             \n    foo: bar;\r\
             \n  }\r\
             \n}"
        ),
        "Error: \"error\"\
         \n  ,\
         \n3 |     @error \"error\";\
         \n  |     ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
    );
}
