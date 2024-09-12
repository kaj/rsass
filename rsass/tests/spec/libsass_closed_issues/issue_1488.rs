//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1488.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1488")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function foo($arg2) {\
             \n  @return meta.type-of($arg2);\
             \n}\n\
             \n@function foo_($arg2...) {\
             \n  @return meta.type-of($arg2);\
             \n}\n\
             \n@function bar($arg1, $arg2) {\
             \n  @return meta.type-of($arg1) + \"::\" + meta.type-of($arg2);\
             \n}\n\
             \n@function bar_($arg1, $arg2...) {\
             \n  @return meta.type-of($arg1) + \"::\" + meta.type-of($arg2);\
             \n}\n\
             \nfoo {\
             \n  foo: foo(one);\
             \n  foo: foo(one...);\
             \n  bar: bar(one, two);\
             \n  bar: bar(one, two...);\
             \n  foo: meta.call(\'foo\', one);\
             \n  foo: meta.call(\'foo\', one...);\
             \n  bar: meta.call(\'bar\', one, two);\
             \n  bar: meta.call(\'bar\', one, two...);\
             \n}\n\
             \nbar {\
             \n  foo: foo_(one);\
             \n  foo: foo_(one...);\
             \n  bar: bar_(one, two);\
             \n  bar: bar_(one, two...);\
             \n  foo: meta.call(\'foo_\', one);\
             \n  foo: meta.call(\'foo_\', one...);\
             \n  bar: meta.call(\'bar_\', one, two);\
             \n  bar: meta.call(\'bar_\', one, two...);\
             \n}"),
        "foo {\
         \n  foo: string;\
         \n  foo: string;\
         \n  bar: string::string;\
         \n  bar: string::string;\
         \n  foo: string;\
         \n  foo: string;\
         \n  bar: string::string;\
         \n  bar: string::string;\
         \n}\
         \nbar {\
         \n  foo: arglist;\
         \n  foo: arglist;\
         \n  bar: string::arglist;\
         \n  bar: string::arglist;\
         \n  foo: arglist;\
         \n  foo: arglist;\
         \n  bar: string::arglist;\
         \n  bar: string::arglist;\
         \n}\n"
    );
}
