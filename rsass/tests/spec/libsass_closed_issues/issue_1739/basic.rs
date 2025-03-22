//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739/basic.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\r\
             \n  baz: 2/3;\r\
             \n  baz: 2/  3;\r\
             \n  baz: 2  /3;\r\
             \n  baz: 2  /  3;\r\
             \n}\r\
             \n\r\
             \nadd {\r\
             \n  baz: 2+3;\r\
             \n  baz: 2+  3;\r\
             \n  baz: 2  +3;\r\
             \n  baz: 2  +  3;\r\
             \n}\r\
             \n\r\
             \nsub {\r\
             \n  baz: 2-3;\r\
             \n  baz: 2-  3;\r\
             \n  baz: 2  -3;\r\
             \n  baz: 2  -  3;\r\
             \n}\r\
             \n\r\
             \nmul {\r\
             \n  baz: 2*3;\r\
             \n  baz: 2*  3;\r\
             \n  baz: 2  *3;\r\
             \n  baz: 2  *  3;\r\
             \n}\r\
             \n\r\
             \nmod {\r\
             \n  baz: 2%3;\r\
             \n  baz: 2%  3;\r\
             \n  baz: 2  %3;\r\
             \n  baz: 2  %  3;\r\
             \n}"),
        "div {\
         \n  baz: 2/3;\
         \n  baz: 2/3;\
         \n  baz: 2/3;\
         \n  baz: 2/3;\
         \n}\
         \nadd {\
         \n  baz: 5;\
         \n  baz: 5;\
         \n  baz: 5;\
         \n  baz: 5;\
         \n}\
         \nsub {\
         \n  baz: -1;\
         \n  baz: -1;\
         \n  baz: 2 -3;\
         \n  baz: -1;\
         \n}\
         \nmul {\
         \n  baz: 6;\
         \n  baz: 6;\
         \n  baz: 6;\
         \n  baz: 6;\
         \n}\
         \nmod {\
         \n  baz: 2% 3;\
         \n  baz: 2% 3;\
         \n  baz: 2;\
         \n  baz: 2;\
         \n}\n"
    );
}
