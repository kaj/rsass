//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/interpolate/right.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("right")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "div {\r\
             \n  baz: 3/#{1/2};\r\
             \n  baz: 3/  #{1/  2};\r\
             \n  baz: 3  /#{1  /2};\r\
             \n  baz: 3 /  #{1  /  2};\r\
             \n}\r\
             \n\r\
             \nadd {\r\
             \n  baz: 3+#{1+2};\r\
             \n  baz: 3+  #{1+  2};\r\
             \n  baz: 3  +#{1  +2};\r\
             \n  baz: 3 +  #{1  +  2};\r\
             \n}\r\
             \n\r\
             \nsub {\r\
             \n  baz: 3-#{1-2};\r\
             \n  baz: 3-  #{1-  2};\r\
             \n  baz: 3  -#{1  -2};\r\
             \n  baz: 3 -  #{1  -  2};\r\
             \n}\r\
             \n\r\
             \nmul {\r\
             \n  baz: 3*#{1*2};\r\
             \n  baz: 3*  #{1*  2};\r\
             \n  baz: 3  *#{1  *2};\r\
             \n  baz: 3 *  #{1  *  2};\r\
             \n}\r\
             \n\r\
             \nmod {\r\
             \n  baz: 3%#{1%2};\r\
             \n  baz: 3%  #{1%  2};\r\
             \n  baz: 3  %#{1  %2};\r\
             \n  baz: 3 %  #{1  %  2};\r\
             \n}"
        ),
        "Error: Undefined operation \"3 * 2\".\
         \n   ,\
         \n23 |   baz: 3*#{1*2};\
         \n   |        ^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet",
    );
}
