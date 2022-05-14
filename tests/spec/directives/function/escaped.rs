//! Tests auto-converted from "sass-spec/spec/directives/function/escaped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaped")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// Function names can be defined and referred to using escapes, which are\
             \n// normalized.\
             \n@function f\\6Fo-bar() {@return 1}\n\
             \na {b: foo-b\\61r()}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
