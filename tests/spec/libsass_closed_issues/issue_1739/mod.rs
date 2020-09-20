//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1739"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1739/basic.hrx"
#[test]
#[ignore] // wrong result
fn basic() {
    assert_eq!(
        rsass(
            "div {\r\
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
            \n}"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}

mod interpolate;
