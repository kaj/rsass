//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/237_extend_with_universal_selector_different_namespace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a ns|*.foo {a: b}\
             \na {@extend .foo}\
             \n-a {@extend %-a}\n"),
        "-a ns|*.foo {\
         \n  a: b;\
         \n}\n"
    );
}
