//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/grayscale.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("grayscale")
}

#[test]
fn with_calc() {
    assert_eq!(
        runner().ok("a {b: grayscale(calc(1 + 2))}\n"),
        "a {\
         \n  b: grayscale(3);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn with_color() {
    assert_eq!(
        runner().ok("a {b: grayscale(red)}\n"),
        "a {\
         \n  b: rgb(127.5, 127.5, 127.5);\
         \n}\n"
    );
}
#[test]
fn with_css_var() {
    assert_eq!(
        runner().ok("a {b: grayscale(var(--c))}\n"),
        "a {\
         \n  b: grayscale(var(--c));\
         \n}\n"
    );
}
#[test]
fn with_number() {
    assert_eq!(
        runner().ok(
            "// A number should produce a plain function string, for CSS filter functions.\
             \na {b: grayscale(15%)}\n"
        ),
        "a {\
         \n  b: grayscale(15%);\
         \n}\n"
    );
}
#[test]
fn with_unquoted_calc() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: grayscale(string.unquote(\'calc(1)\'))}\n"),
        "a {\
         \n  b: grayscale(calc(1));\
         \n}\n"
    );
}
