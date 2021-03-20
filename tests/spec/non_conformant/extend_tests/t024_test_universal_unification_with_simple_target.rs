//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/024_test_universal_unification_with_simple_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .foo.bar {a: b}\
            \nns|* {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a ns|*.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
