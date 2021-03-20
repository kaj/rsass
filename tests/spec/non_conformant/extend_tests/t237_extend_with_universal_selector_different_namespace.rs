//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/237_extend_with_universal_selector_different_namespace.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a ns|*.foo {a: b}\
            \na {@extend .foo}\
            \n-a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a ns|*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
