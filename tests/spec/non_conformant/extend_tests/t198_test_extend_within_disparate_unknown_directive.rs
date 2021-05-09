//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/198_test_extend_within_disparate_unknown_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@flooblehoof {.foo {a: b}}\
             \n@flooblehoof {.bar {@extend .foo}}\n"),
        "@flooblehoof {\
         \n  .foo, .bar {\
         \n    a: b;\
         \n  }\
         \n}\
         \n@flooblehoof {}\n"
    );
}
