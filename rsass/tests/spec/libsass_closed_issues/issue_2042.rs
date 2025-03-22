//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2042.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2042")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".wizard-editor {\r\
             \n    transform: scale(50/1);\r\
             \n    transform: scale((50/1));\r\
             \n    transform: scale( (50/1) );\r\
             \n}"),
        ".wizard-editor {\
         \n  transform: scale(50/1);\
         \n  transform: scale(50);\
         \n  transform: scale(50);\
         \n}\n"
    );
}
