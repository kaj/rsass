//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1723.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1723")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\n\
             \ntest-1 test-2 test-3 test-4 test-5,\
             \ntest-6 test-7 test-8 test-9 test-10 {\
             \n    @each $set in & {\
             \n        set: meta.inspect($set);\n\
             \n        @each $selector in $set {\
             \n            selector: meta.inspect($selector);\
             \n        }\
             \n    }\
             \n}\n\
             \ntest-1 test-2 test-3 test-4 test-5,\
             \ntest-6 test-7 test-8 test-9 test-10 {\
             \n    @for $i from 1 through list.length(&) {\
             \n        $set: list.nth(&, $i);\
             \n        set: meta.inspect($set);\n\
             \n        @each $selector in $set {\
             \n            selector: meta.inspect($selector);\
             \n        }\
             \n    }\
             \n}"),
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
         \n}\n"
    );
}
