//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/199_test_extend_within_disparate_nested_directives.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("199_test_extend_within_disparate_nested_directives")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media screen {@flooblehoof {.foo {a: b}}}\
             \n@media screen {@flooblehoof {.bar {@extend .foo}}}\n"),
        "@media screen {\
         \n  @flooblehoof {\
         \n    .foo, .bar {\
         \n      a: b;\
         \n    }\
         \n  }\
         \n}\
         \n@media screen {\
         \n  @flooblehoof {}\
         \n}\n"
    );
}
