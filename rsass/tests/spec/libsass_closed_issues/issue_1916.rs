//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1916.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1916")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".z-depth-1 {\
             \n  box-shadow: 0 2px 5px 0 rgba(0, 0, 0, 0.16), 0 2px 10px 0 rgba(0, 0, 0, 0.12);\
             \n}\n\
             \n.z-depth-1-half {\
             \n  box-shadow: 0 5px 11px 0 rgba(0, 0, 0, 0.18), 0 4px 15px 0 rgba(0, 0, 0, 0.15);\
             \n}\n\
             \n.btn {\
             \n  @extend .z-depth-1;\n\
             \n  &:hover {\
             \n    @extend .z-depth-half-1;\
             \n  }\
             \n}\n"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend .z-depth-half-1 !optional\" to avoid this error.\
         \n   ,\
         \n13 |     @extend .z-depth-half-1;\
         \n   |     ^^^^^^^^^^^^^^^^^^^^^^^\
         \n   \'\
         \n  input.scss 13:5  root stylesheet",
    );
}
