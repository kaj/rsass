//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone-itpl.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@at-root {\r\
             \n  #{&} {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n2 |   {\
         \n  |   ^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
