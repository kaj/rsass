//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/223_test_duplicated_selector_with_newlines.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("223_test_duplicated_selector_with_newlines")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".example-1-1,\
             \n.example-1-2,\
             \n.example-1-3 {\
             \na: b;\
             \n}\n\
             \n.my-page-1 .my-module-1-1 {@extend .example-1-2}\n"),
        ".example-1-1,\
         \n.example-1-2,\
         \n.my-page-1 .my-module-1-1,\
         \n.example-1-3 {\
         \n  a: b;\
         \n}\n"
    );
}
