//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/017_test_class_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .foo.baz {a: b}\
            \n.baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
