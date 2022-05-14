//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/interpolate/both.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("both")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "div {\r\
             \n  baz: #{1/2}/#{1/2};\r\
             \n  baz: #{1/  2}/  #{1/  2};\r\
             \n  baz: #{1  /2}  /#{1  /2};\r\
             \n  baz: #{1  /  2}  /  #{1  /  2};\r\
             \n}\r\
             \n\r\
             \nadd {\r\
             \n  baz: #{1+2}+#{1+2};\r\
             \n  baz: #{1+  2}+  #{1+  2};\r\
             \n  baz: #{1  +2}  +#{1  +2};\r\
             \n  baz: #{1  +  2}  +  #{1  +  2};\r\
             \n}\r\
             \n\r\
             \nsub {\r\
             \n  baz: #{1-2}-#{1-2};\r\
             \n  baz: #{1-  2}-  #{1-  2};\r\
             \n  baz: #{1  -2}  -#{1  -2};\r\
             \n  baz: #{1  -  2}  -  #{1  -  2};\r\
             \n}\r\
             \n\r\
             \nmul {\r\
             \n  baz: #{1*2}*#{1*2};\r\
             \n  baz: #{1*  2}*  #{1*  2};\r\
             \n  baz: #{1  *2}  *#{1  *2};\r\
             \n  baz: #{1  *  2}  *  #{1  *  2};\r\
             \n}\r\
             \n\r\
             \nmod {\r\
             \n  baz: #{1%2}%#{1%2};\r\
             \n  baz: #{1%  2}%  #{1%  2};\r\
             \n  baz: #{1  %2}  %#{1  %2};\r\
             \n  baz: #{1  %  2}  %  #{1  %  2};\r\
             \n}"
        ),
        "Error: Undefined operation \"2 * 2\".\
         \n   ,\
         \n23 |   baz: #{1*2}*#{1*2};\
         \n   |        ^^^^^^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet",
    );
}
