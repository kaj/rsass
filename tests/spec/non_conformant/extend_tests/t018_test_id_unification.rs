//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/018_test_id_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a .foo.bar {a: b}\
            \n#baz {@extend .foo} -a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a .foo.bar, -a .bar#baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
