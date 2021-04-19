//! Tests auto-converted from "sass-spec/spec/directives/import/error/not_found.hrx"

#[test]
#[ignore] // missing error
fn directory_dot_import() {
    assert_eq!(
        crate::rsass(
            "// Import-only file extensions only apply to individual files, not to\
             \n// directories.\
             \n@import \"other\";\
             \n"
        ).unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn no_extension() {
    assert_eq!(
        crate::rsass(
            "@import \"other\";\
             \n"
        )
        .unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn parent_relative() {
    assert_eq!(
        crate::rsass(
            "// A file in a subdirectory shouldn\'t be able to load a URL relative\
             \n// to the importing file.\
             \n// Regression test for scssphp/scssphp#242\
             \n@import \"dir/child\"\
             \n"
        ).unwrap_err(),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"sibling\"\
         \n  |         ^^^^^^^^^\
         \n  \'\
         \n  dir/child.scss 1:9  @import\
         \n  input.scss 4:9      root stylesheet\
         \n",
    );
}
