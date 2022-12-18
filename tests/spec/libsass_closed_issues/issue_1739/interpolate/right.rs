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
        "DEPRECATION WARNING on line 11, column 14 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    1 + 2\n\
         \nbut you may have intended it to mean:\n\
         \n    1 (+2)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n11 |   baz: 3  +#{1  +2};\
         \n   |              ^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 11, column 8 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    3 + #{1 + 2}\n\
         \nbut you may have intended it to mean:\n\
         \n    3 (+#{1 + 2})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n11 |   baz: 3  +#{1  +2};\
         \n   |        ^^^^^^^^^^^^\
         \n   \'\n\
         \nError: Undefined operation \"3 * 2\".\
         \n   ,\
         \n23 |   baz: 3*#{1*2};\
         \n   |        ^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet",
    );
}
