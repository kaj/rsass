//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/color.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\r\
             \n  red: \'foo\',\r\
             \n  red: \'bar\',\r\
             \n);\r\
             \n\r\
             \n.foo {\r\
             \n  content: inspect($map);\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Duplicate key.\
         \n  ,\
         \n2 |   red: \'foo\',\
         \n  |   === first key\
         \n3 |   red: \'bar\',\
         \n  |   ^^^ second key\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
