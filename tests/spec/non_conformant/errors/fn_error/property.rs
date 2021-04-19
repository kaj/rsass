//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/property.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\r\
             \n  b: {\r\
             \n    @error \"error\";\r\
             \n    foo: bar;\r\
             \n  }\r\
             \n}"
        )
        .unwrap_err(),
        "Error: \"error\"\
         \n  ,\
         \n3 |     @error \"error\";\
         \n  |     ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
    );
}
