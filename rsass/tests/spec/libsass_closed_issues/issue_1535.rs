//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1535.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1535")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \nfoo {\
             \n    test: meta.type-of(1--em);\
             \n    test: (1--em-2--em);\
             \n    test: (1--em- 2--em);\
             \n    test: (1--em -2--em);\
             \n}\n"),
        "foo {\
         \n  test: list;\
         \n  test: 1 --em-2--em;\
         \n  test: 1 --em- 2 --em;\
         \n  test: 1 --em -2 --em;\
         \n}\n"
    );
}
