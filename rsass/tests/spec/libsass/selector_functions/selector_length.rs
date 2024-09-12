//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/selector-length.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector-length")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \nfoo.bar.baz asd.qwe xyz, second {\r\
             \n  length: list.length(&);\r\
             \n  length: list.length(list.nth(&, 1));\r\
             \n  length: list.length(list.nth(list.nth(&, 1), 1));\r\
             \n}"),
        "foo.bar.baz asd.qwe xyz, second {\
         \n  length: 2;\
         \n  length: 3;\
         \n  length: 1;\
         \n}\n"
    );
}
