//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1526.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1526")
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
