//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1488.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($arg2) {\
            \n  @return type-of($arg2);\
            \n}\
            \n\
            \n@function foo_($arg2...) {\
            \n  @return type-of($arg2);\
            \n}\
            \n\
            \n@function bar($arg1, $arg2) {\
            \n  @return type-of($arg1) + \"::\" + type-of($arg2);\
            \n}\
            \n\
            \n@function bar_($arg1, $arg2...) {\
            \n  @return type-of($arg1) + \"::\" + type-of($arg2);\
            \n}\
            \n\
            \nfoo {\
            \n  foo: foo(one);\
            \n  foo: foo(one...);\
            \n  bar: bar(one, two);\
            \n  bar: bar(one, two...);\
            \n  foo: call(\'foo\', one);\
            \n  foo: call(\'foo\', one...);\
            \n  bar: call(\'bar\', one, two);\
            \n  bar: call(\'bar\', one, two...);\
            \n}\
            \n\
            \nbar {\
            \n  foo: foo_(one);\
            \n  foo: foo_(one...);\
            \n  bar: bar_(one, two);\
            \n  bar: bar_(one, two...);\
            \n  foo: call(\'foo_\', one);\
            \n  foo: call(\'foo_\', one...);\
            \n  bar: call(\'bar_\', one, two);\
            \n  bar: call(\'bar_\', one, two...);\
            \n}"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
