//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-alone-itpl.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "#{&} {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n1 | {\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
