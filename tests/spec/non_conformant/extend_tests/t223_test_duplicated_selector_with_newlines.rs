//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/223_test_duplicated_selector_with_newlines.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".example-1-1,\
            \n.example-1-2,\
            \n.example-1-3 {\
            \na: b;\
            \n}\
            \n\
            \n.my-page-1 .my-module-1-1 {@extend .example-1-2}\
            \n"
        )
        .unwrap(),
        ".example-1-1,\
        \n.example-1-2,\
        \n.my-page-1 .my-module-1-1,\
        \n.example-1-3 {\
        \n  a: b;\
        \n}\
        \n"
    );
}
