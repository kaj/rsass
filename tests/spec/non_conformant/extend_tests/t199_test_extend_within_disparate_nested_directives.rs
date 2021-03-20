//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/199_test_extend_within_disparate_nested_directives.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {@flooblehoof {.foo {a: b}}}\
            \n@media screen {@flooblehoof {.bar {@extend .foo}}}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  @flooblehoof {\
        \n    .foo, .bar {\
        \n      a: b;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen {\
        \n  @flooblehoof {}\
        \n}\
        \n"
    );
}
