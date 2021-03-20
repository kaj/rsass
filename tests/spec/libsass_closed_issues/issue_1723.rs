//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1723.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test-1 test-2 test-3 test-4 test-5,\r\
            \ntest-6 test-7 test-8 test-9 test-10 {\r\
            \n    @each $set in & {\r\
            \n        set: inspect($set);\r\
            \n\r\
            \n        @each $selector in $set {\r\
            \n            selector: inspect($selector);\r\
            \n        }\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \ntest-1 test-2 test-3 test-4 test-5,\r\
            \ntest-6 test-7 test-8 test-9 test-10 {\r\
            \n    @for $i from 1 through length(&) {\r\
            \n        $set: nth(&, $i);\r\
            \n        set: inspect($set);\r\
            \n\r\
            \n        @each $selector in $set {\r\
            \n            selector: inspect($selector);\r\
            \n        }\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "test-1 test-2 test-3 test-4 test-5,\
        \ntest-6 test-7 test-8 test-9 test-10 {\
        \n  set: test-1 test-2 test-3 test-4 test-5;\
        \n  selector: test-1;\
        \n  selector: test-2;\
        \n  selector: test-3;\
        \n  selector: test-4;\
        \n  selector: test-5;\
        \n  set: test-6 test-7 test-8 test-9 test-10;\
        \n  selector: test-6;\
        \n  selector: test-7;\
        \n  selector: test-8;\
        \n  selector: test-9;\
        \n  selector: test-10;\
        \n}\
        \ntest-1 test-2 test-3 test-4 test-5,\
        \ntest-6 test-7 test-8 test-9 test-10 {\
        \n  set: test-1 test-2 test-3 test-4 test-5;\
        \n  selector: test-1;\
        \n  selector: test-2;\
        \n  selector: test-3;\
        \n  selector: test-4;\
        \n  selector: test-5;\
        \n  set: test-6 test-7 test-8 test-9 test-10;\
        \n  selector: test-6;\
        \n  selector: test-7;\
        \n  selector: test-8;\
        \n  selector: test-9;\
        \n  selector: test-10;\
        \n}\
        \n"
    );
}
