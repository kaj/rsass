//! Tests auto-converted from "sass-spec/spec/directives/import/error/not_found.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("not_found")
        .mock_file(
            "directory_dot_import/other.import/index.scss",
            "a {b: c}\n",
        )
        .mock_file("no_extension/other", "a {b: c}\n")
        .mock_file("parent_relative/dir/child.scss", "@import \"sibling\"\n")
        .mock_file("parent_relative/sibling.scss", "a {b: \"\"}\n")
}

#[test]
#[ignore] // wrong error
fn directory_dot_import() {
    let runner = runner().with_cwd("directory_dot_import");
    assert_eq!(
        runner.err(
            "// Import-only file extensions only apply to individual files, not to\
             \n// directories.\
             \n@import \"other\";\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    input.scss 3:9  root stylesheet\n\
         \nError: Can\'t find stylesheet to import.\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn no_extension() {
    let runner = runner().with_cwd("no_extension");
    assert_eq!(
        runner.err(
            "@import \"other\";\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn parent_relative() {
    let runner = runner().with_cwd("parent_relative");
    assert_eq!(
        runner.err(
            "// A file in a subdirectory shouldn\'t be able to load a URL relative\
             \n// to the importing file.\
             \n// Regression test for scssphp/scssphp#242\
             \n@import \"dir/child\"\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n4 | @import \"dir/child\"\
         \n  |         ^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 4:9  root stylesheet\n\
         \nDEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"sibling\"\
         \n  |         ^^^^^^^^^\
         \n  \'\
         \n    dir/child.scss 1:9  @import\
         \n    input.scss 4:9      root stylesheet\n\
         \nError: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"sibling\"\
         \n  |         ^^^^^^^^^\
         \n  \'\
         \n  dir/child.scss 1:9  @import\
         \n  input.scss 4:9      root stylesheet",
    );
}
