//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/010_test_multiple_extends_with_multiple_extenders_and_single_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "010_test_multiple_extends_with_multiple_extenders_and_single_target",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo.bar {a: b}\
             \n.baz {@extend .foo}\
             \n.bang {@extend .bar}\n"),
        ".foo.bar, .foo.bang, .bar.baz, .baz.bang {\
         \n  a: b;\
         \n}\n"
    );
}
