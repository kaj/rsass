//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_815.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_815")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \nfoo {\
             \n  foo: string.slice(\"bar\", 1, 2);\
             \n  bar: string.slice(\"bar\", 3);\
             \n}\n"),
        "foo {\
         \n  foo: \"ba\";\
         \n  bar: \"r\";\
         \n}\n"
    );
}
