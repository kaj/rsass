//! Tests auto-converted from "sass-spec/spec/directives/use/error/load.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file(
            "conflict/all/_other.css",
            "a {syntax: css; partial: true}\n",
        )
        .mock_file(
            "conflict/all/_other.sass",
            "a\n  syntax: sass\n  partial: true\n",
        )
        .mock_file(
            "conflict/all/_other.scss",
            "a {syntax: scss; partial: true}\n",
        )
        .mock_file(
            "conflict/all/other.css",
            "a {syntax: css; partial: false}\n",
        )
        .mock_file(
            "conflict/all/other.sass",
            "a\n  syntax: sass\n  partial: false\n",
        )
        .mock_file(
            "conflict/all/other.scss",
            "a {syntax: scss; partial: false}\n",
        )
        .mock_file(
            "conflict/extension/sass_and_scss/other.sass",
            "a\n  syntax: sass\n",
        )
        .mock_file(
            "conflict/extension/sass_and_scss/other.scss",
            "a {syntax: scss}\n",
        )
        .mock_file("conflict/index/other/_index.scss", "a {partial: true}\n")
        .mock_file("conflict/index/other/index.scss", "a {partial: false}\n")
        .mock_file("conflict/partial/_other.scss", "a {partial: true}\n")
        .mock_file("conflict/partial/other.scss", "a {partial: false}\n")
        .mock_file("conflicting_namespace/explicit/other1.scss", "")
        .mock_file("conflicting_namespace/explicit/other2.scss", "")
        .mock_file("conflicting_namespace/implicit/dir1/other.scss", "")
        .mock_file("conflicting_namespace/implicit/dir2/other.scss", "")
        .mock_file("conflicting_namespace/mixed/other.scss", "")
        .mock_file("conflicting_namespace/mixed/other2.scss", "")
        .mock_file("dir_dot_scss/dir.scss/index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("loop/import_to_use/other.scss", "@use \"input\";\n")
        .mock_file("loop/use_to_import/other.scss", "@import \"input\";\n")
        .mock_file("loop/use_to_use/other.scss", "@use \"input\";\n")
        .mock_file("no_extension/other", "a {b: c}\n")
        .mock_file("loop/import_to_use/input.scss", "@import \"other\";\n")
        .mock_file("loop/use_self/input.scss", "@use \"input\";\n")
        .mock_file("loop/use_to_import/input.scss", "@use \"other\";\n")
        .mock_file("loop/use_to_use/input.scss", "@use \"other\";\n")
}

mod conflict {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("conflict")
    }

    #[test]
    #[ignore] // missing error
    fn all() {
        let runner = runner().with_cwd("all");
        assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it has conflicting partials *and*\
             \n// conflicting extensions.\
             \n@use \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.sass\
         \n  other.sass\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
    mod extension {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("extension")
        }

        #[test]
        #[ignore] // missing error
        fn sass_and_scss() {
            let runner = runner().with_cwd("sass_and_scss");
            assert_eq!(
        runner.err(
            "// This import can\'t be resolved because it could refer to either the \".sass\" or\
             \n// \".scss\" file.\
             \n@use \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  other.sass\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
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
             \n@use \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  other/_index.scss\
         \n  other/index.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
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
             \n@use \"other\";\n"
        ),
        "Error: It\'s not clear which file to import. Found:\
         \n  _other.scss\
         \n  other.scss\
         \n  ,\
         \n3 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
}
mod conflicting_namespace {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("conflicting_namespace")
    }

    #[test]
    #[ignore] // missing error
    fn built_in() {
        let runner = runner().with_cwd("built_in");
        assert_eq!(
            runner.err(
                "// Regression test for sass/dart-sass#1047\
             \n@use \"sass:math\";\
             \n@use \"sass:math\";\n"
            ),
            "Error: There\'s already a module with namespace \"math\".\
         \n  ,\
         \n2 | @use \"sass:math\";\
         \n  | ================ original @use\
         \n3 | @use \"sass:math\";\
         \n  | ^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn explicit() {
        let runner = runner().with_cwd("explicit");
        assert_eq!(
            runner.err(
                "@use \"other1\" as other;\
             \n@use \"other2\" as other;\n"
            ),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"other1\" as other;\
         \n  | ====================== original @use\
         \n2 | @use \"other2\" as other;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn implicit() {
        let runner = runner().with_cwd("implicit");
        assert_eq!(
            runner.err(
                "@use \"dir1/other\";\
             \n@use \"dir2/other\";\n"
            ),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"dir1/other\";\
         \n  | ================= original @use\
         \n2 | @use \"dir2/other\";\
         \n  | ^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixed() {
        let runner = runner().with_cwd("mixed");
        assert_eq!(
            runner.err(
                "@use \"other\";\
             \n@use \"other2\" as other;\n"
            ),
            "Error: There\'s already a module with namespace \"other\".\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ============ original @use\
         \n2 | @use \"other2\" as other;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^ new @use\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
#[test]
fn dir_dot_scss() {
    let runner = runner().with_cwd("dir_dot_scss");
    assert_eq!(
        runner.err("@use \"dir.scss\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"dir.scss\";\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod test_loop {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("loop")
    }

    #[test]
    #[ignore] // wrong error
    fn import_to_use() {
        let runner = runner().with_cwd("import_to_use");
        assert_eq!(
        runner.err(
            "@import \"other\";\n"
        ),
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"other\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @import\
         \n  input.scss 1:9  root stylesheet",
    );
    }
    #[test]
    fn use_self() {
        let runner = runner().with_cwd("use_self");
        assert_eq!(
            runner.err("@use \"input\";\n"),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn use_to_import() {
        let runner = runner().with_cwd("use_to_import");
        assert_eq!(
        runner.err(
            "@use \"other\";\n"
        ),
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"input\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    other.scss 1:9  @use\
         \n    input.scss 1:1  root stylesheet\n\
         \nError: This file is already being loaded.\
         \n  ,\
         \n1 | @import \"input\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  other.scss 1:9  @use\
         \n  input.scss 1:1  root stylesheet",
    );
    }
    #[test]
    fn use_to_use() {
        let runner = runner().with_cwd("use_to_use");
        assert_eq!(
            runner.err("@use \"other\";\n"),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
#[test]
fn missing() {
    let runner = runner().with_cwd("missing");
    assert_eq!(
        runner.err("@use \"other\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn no_extension() {
    let runner = runner().with_cwd("no_extension");
    assert_eq!(
        runner.err("@use \"other\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn unknown_scheme() {
    let runner = runner().with_cwd("unknown_scheme");
    assert_eq!(
        runner.err("@use \"scheme:bar\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @use \"scheme:bar\";\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
