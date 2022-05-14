//! Tests auto-converted from "sass-spec/spec/libsass/wrapped-selector-whitespace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("wrapped-selector-whitespace")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("div {\r\
             \n  :-moz-any(a , b) {\r\
             \n    foo: foo;\r\
             \n  }\r\
             \n  :foo(a , b) {\r\
             \n    bar: bar;\r\
             \n  }\r\
             \n}\r\n"),
        "div :-moz-any(a, b) {\
         \n  foo: foo;\
         \n}\
         \ndiv :foo(a , b) {\
         \n  bar: bar;\
         \n}\n"
    );
}
