//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/interpolate/right.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
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
        )
        .unwrap_err(),
        "Error: Undefined operation \"3 * 2\".\
         \n   ,\
         \n23 |   baz: 3*#{1*2};\
         \n   |        ^^^^^^^^\
         \n   \'\
         \n  input.scss 23:8  root stylesheet\
         \n",
    );
}
