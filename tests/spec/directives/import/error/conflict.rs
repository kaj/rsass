//! Tests auto-converted from "sass-spec/spec/directives/import/error/conflict.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("conflict")
        .mock_file("all/_other.sass", "a\n  syntax: sass\n  partial: true\n")
        .mock_file("all/_other.scss", "a {syntax: scss; partial: true}\n")
        .mock_file("all/other.sass", "a\n  syntax: sass\n  partial: false\n")
        .mock_file("all/other.scss", "a {syntax: scss; partial: false}\n")
        .mock_file("extension/other.sass", "a\n  syntax: sass\n")
        .mock_file("extension/other.scss", "a {syntax: scss}\n")
        .mock_file(
            "import_only/no_extension/other.import.sass",
            "a\n  syntax: sass\n",
        )
        .mock_file(
            "import_only/no_extension/other.import.scss",
            "a {syntax: scss}\n",
        )
        .mock_file(
            "import_only/with_extension/_other.import.scss",
            "a {partial: true}\n",
        )
        .mock_file(
            "import_only/with_extension/other.import.scss",
            "a {partial: false}\n",
        )
        .mock_file("index/other/_index.scss", "a {partial: true}\n")
        .mock_file("index/other/index.scss", "a {partial: false}\n")
        .mock_file("partial/_other.scss", "a {partial: true}\n")
        .mock_file("partial/other.scss", "a {partial: false}\n")
}

#[test]
#[ignore] // missing error
fn all() {
    let runner = runner().with_cwd("all");
    assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it has conflicting partials *and*\
             \n// conflicting extensions.\
             \n@import \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.sass\
         \n  other.sass\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn extension() {
    let runner = runner().with_cwd("extension");
    assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the \".sass\" or\
             \n// \".scss\" file.\
             \n@import \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.sass\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
mod import_only {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import_only")
    }

    #[test]
    #[ignore] // missing error
    fn no_extension() {
        let runner = runner().with_cwd("no_extension");
        assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the Sass or\
             \n// the SCSS import-only file.\
             \n@import \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.import.sass\
         \n  other.import.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn with_extension() {
        let runner = runner().with_cwd("with_extension");
        assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial.\
             \n@import \"other.scss\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.import.scss\
         \n  other.import.scss\
         \n  ,\
         \n3 | @import \"other.scss\";\
         \n  |         ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn index() {
    let runner = runner().with_cwd("index");
    assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial index file.\
             \n@import \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  other/_index.scss\
         \n  other/index.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn partial() {
    let runner = runner().with_cwd("partial");
    assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the partial or\
             \n// the non-partial file.\
             \n@import \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
