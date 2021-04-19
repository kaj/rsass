//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/functioncall.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function fncall($void) {\r\
             \n  @return \"key\";\r\
             \n}\r\
             \n\r\
             \n$map: (\r\
             \n  fncall(1+2): \'foo\',\r\
             \n  fncall(1+2): \'bar\',\r\
             \n);\r\
             \n\r\
             \n.foo {\r\
             \n  content: inspect($map);\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Duplicate key.\
         \n  ,\
         \n6 |   fncall(1+2): \'foo\',\
         \n  |   =========== first key\
         \n7 |   fncall(1+2): \'bar\',\
         \n  |   ^^^^^^^^^^^ second key\
         \n  \'\
         \n  input.scss 7:3  root stylesheet\
         \n",
    );
}
