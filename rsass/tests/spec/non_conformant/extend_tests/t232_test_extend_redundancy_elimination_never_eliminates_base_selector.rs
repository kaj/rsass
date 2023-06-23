//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/232_test_extend_redundancy_elimination_never_eliminates_base_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("232_test_extend_redundancy_elimination_never_eliminates_base_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.foo {a: b}\
             \n.foo {@extend a}\n"),
        "a.foo, .foo {\
         \n  a: b;\
         \n}\n"
    );
}
