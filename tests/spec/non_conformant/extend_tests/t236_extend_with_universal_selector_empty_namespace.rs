//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/236_extend_with_universal_selector_empty_namespace.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a |*.foo {a: b}\
            \na {@extend .foo}\
            \n-a {@extend %-a}\
            \n"
        )
        .unwrap(),
        "-a |*.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
