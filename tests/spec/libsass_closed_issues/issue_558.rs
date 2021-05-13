//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_558.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function is_gold($c) {\r\
             \n    @if ($c == gold) {\r\
             \n        @return \'yes\';\r\
             \n    }\r\
             \n    @return \'no\';\r\
             \n}\r\
             \n\r\
             \ndiv {\r\
             \n    foo: is_gold(gold);\r\
             \n    bar: is_gold(white);\r\
             \n}"),
        "div {\
         \n  foo: \"yes\";\
         \n  bar: \"no\";\
         \n}\n"
    );
}
