//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/interpolate/left.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
             \n  baz: #{1/2}/3;\r\
             \n  baz: #{1/  2}/  3;\r\
             \n  baz: #{1  /2}  /3;\r\
             \n  baz: #{1  /  2}  /  3;\r\
             \n}\r\
             \n\r\
             \nadd {\r\
             \n  baz: #{1+2}+3;\r\
             \n  baz: #{1+  2}+  3;\r\
             \n  baz: #{1  +2}  +3;\r\
             \n  baz: #{1  +  2}  +  3;\r\
             \n}\r\
             \n\r\
             \nsub {\r\
             \n  baz: #{1-2}-3;\r\
             \n  baz: #{1-  2}-  3;\r\
             \n  baz: #{1  -2}  -3;\r\
             \n  baz: #{1  -  2}  -  3;\r\
             \n}\r\
             \n\r\
             \nmul {\r\
             \n  baz: #{1*2}*3;\r\
             \n  baz: #{1*  2}*  3;\r\
             \n  baz: #{1  *2}  *3;\r\
             \n  baz: #{1  *  2}  *  3;\r\
             \n}\r\
             \n\r\
             \nmod {\r\
             \n  baz: #{1%2}%3;\r\
             \n  baz: #{1%  2}%  3;\r\
             \n  baz: #{1  %2}  %3;\r\
             \n  baz: #{1  %  2}  %  3;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Undefined operation \"2 * 3\".\
         \n   ,\
         \n23 |   baz: #{1*2}*3;\
         \n   |        ^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet",
    );
}
