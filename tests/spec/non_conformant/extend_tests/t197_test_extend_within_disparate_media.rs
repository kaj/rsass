//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/197_test_extend_within_disparate_media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media screen {.foo {a: b}}\
             \n@media screen {.bar {@extend .foo}}\n"),
        "@media screen {\
         \n  .foo, .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
