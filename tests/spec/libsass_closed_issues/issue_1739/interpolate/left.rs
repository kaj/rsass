//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/interpolate/left.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("left")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
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
        ),
        "Error: Undefined operation \"2 * 3\".\
         \n   ,\
         \n23 |   baz: #{1*2}*3;\
         \n   |        ^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet",
    );
}
