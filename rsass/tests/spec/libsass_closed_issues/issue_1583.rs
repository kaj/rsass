//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1583.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1583")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"sass:string\";\n\
             \n$ls: ((foo,));\n\
             \nfoo {\
             \n  baz: list.length($ls);\
             \n  baz: meta.type-of($ls);\
             \n  baz: meta.inspect($ls);\
             \n}\n\
             \nbar {\
             \n  baz: list.length(&);\
             \n  baz: meta.type-of(&);\
             \n  baz: meta.inspect(&);\
             \n}\n\
             \nfoo {\
             \n  string: meta.inspect(&);\
             \n  str-length: string.length(meta.inspect(&));\
             \n  list-length: list.length(&);\
             \n}\n\
             \nfoo, bar {\
             \n  string: meta.inspect(&);\
             \n  str-length: string.length(meta.inspect(&));\
             \n  list-length: list.length(&);\
             \n}\n"),
        "foo {\
         \n  baz: 1;\
         \n  baz: list;\
         \n  baz: (foo,);\
         \n}\
         \nbar {\
         \n  baz: 1;\
         \n  baz: list;\
         \n  baz: (bar,);\
         \n}\
         \nfoo {\
         \n  string: (foo,);\
         \n  str-length: 6;\
         \n  list-length: 1;\
         \n}\
         \nfoo, bar {\
         \n  string: foo, bar;\
         \n  str-length: 8;\
         \n  list-length: 2;\
         \n}\n"
    );
}
