//! Tests auto-converted from "sass-spec/spec/directives/forward/error/load.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file(
            "loop/forward_to_forward/other.scss",
            "@forward \"input\";\n",
        )
        .mock_file(
            "loop/forward_to_import/other.scss",
            "@import \"input\";\n",
        )
        .mock_file("loop/forward_to_use/other.scss", "@use \"input\";\n")
        .mock_file("loop/forward_self/input.scss", "@forward \"input\";\n")
        .mock_file(
            "loop/forward_to_forward/input.scss",
            "@forward \"other\";\n",
        )
        .mock_file(
            "loop/forward_to_import/input.scss",
            "@forward \"other\";\n",
        )
        .mock_file("loop/forward_to_use/input.scss", "@forward \"other\";\n")
}

mod test_loop {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("loop")
    }

    #[test]
    fn forward_self() {
        let runner = runner().with_cwd("forward_self");
        assert_eq!(
            runner.err("@forward \"input\";\n"),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @forward \"input\";\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    fn forward_to_forward() {
        let runner = runner().with_cwd("forward_to_forward");
        assert_eq!(
            runner.err("@forward \"other\";\n"),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @forward \"input\";\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @forward\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn forward_to_import() {
        let runner = runner().with_cwd("forward_to_import");
        assert_eq!(
        runner.err(
            "@forward \"other\";\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"input\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    other.scss 1:9  @forward\
         \n    input.scss 1:1  root stylesheet\n\
         \nError: This file is already being loaded.\
         \n  ,\
         \n1 | @import \"input\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  other.scss 1:9  @forward\
         \n  input.scss 1:1  root stylesheet",
    );
    }
    #[test]
    fn forward_to_use() {
        let runner = runner().with_cwd("forward_to_use");
        assert_eq!(
            runner.err("@forward \"other\";\n"),
            "Error: Module loop: this module is already being loaded.\
         \n  ,\
         \n1 | @use \"input\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  other.scss 1:1  @forward\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn missing() {
    let runner = runner().with_cwd("missing");
    assert_eq!(
        runner.err("@forward \"other\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @forward \"other\";\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
