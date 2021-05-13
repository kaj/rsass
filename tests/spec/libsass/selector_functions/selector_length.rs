//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/selector-length.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo.bar.baz asd.qwe xyz, second {\r\
             \n  length: length(&);\r\
             \n  length: length(nth(&, 1));\r\
             \n  length: length(nth(nth(&, 1), 1));\r\
             \n}"),
        "foo.bar.baz asd.qwe xyz, second {\
         \n  length: 2;\
         \n  length: 3;\
         \n  length: 1;\
         \n}\n"
    );
}
