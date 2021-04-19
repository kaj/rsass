//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2352.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$theme: (red: #D700EE);\r\
             \n\r\
             \n@function test($args...) {\r\
             \n\t@return #000;\r\
             \n}\r\
             \n\r\
             \n.test {\r\
             \n\tcolor: test($theme...);\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Variable keyword argument map must have string keys.\
         \nred is not a string in (red: #D700EE).\
         \n  ,\
         \n8 |     color: test($theme...);\
         \n  |                 ^^^^^^\
         \n  \'\
         \n  input.scss 8:14  root stylesheet",
    );
}
