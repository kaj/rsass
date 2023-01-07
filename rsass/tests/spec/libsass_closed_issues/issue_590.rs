//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_590.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_590")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  foo: 1/2;\
             \n  foo: 0.5;\
             \n  foo: (1/2);\
             \n  foo: 1/2 == 0.5;\
             \n  foo: (1/2) == 0.5;\
             \n}\n"),
        "foo {\
         \n  foo: 1/2;\
         \n  foo: 0.5;\
         \n  foo: 0.5;\
         \n  foo: true;\
         \n  foo: true;\
         \n}\n"
    );
}
