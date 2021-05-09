//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/sass_script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: 1 + 2;\
             \n  b: 1 - 2;\
             \n  c: foo + bar;\
             \n  d: floor(12.3px); }\n"),
        "foo {\
         \n  a: 3;\
         \n  b: -1;\
         \n  c: foobar;\
         \n  d: 12px;\
         \n}\n"
    );
}
