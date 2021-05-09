//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1526.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: (1--em-2--em);\
             \n  baz: (1--em - 2--em);\
             \n}\n"),
        "foo {\
         \n  bar: 1 --em-2--em;\
         \n  baz: 1 --em-2 --em;\
         \n}\n"
    );
}
