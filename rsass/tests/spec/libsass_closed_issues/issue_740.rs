//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_740.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_740")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: null;\
             \n$foo: #fff !default;\
             \n$bar: #000;\
             \n$bar: #f00 !default;\n\
             \nfoo {\
             \n  foo: $foo;\
             \n  bar: $bar;\
             \n}\n"),
        "foo {\
         \n  foo: #fff;\
         \n  bar: #000;\
         \n}\n"
    );
}
