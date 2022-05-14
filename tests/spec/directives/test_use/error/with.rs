//! Tests auto-converted from "sass-spec/spec/directives/use/error/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("with")
        .mock_file("conflict/_left.scss", "$a: left;\n")
        .mock_file("conflict/_midstream.scss", "@use \"left\" as *;\n@use \"right\" as *;\n\n$a: c !default;\n")
        .mock_file("conflict/_right.scss", "$a: right;\n")
        .mock_file("invalid_expression/error/_other.scss", "$a: c !default;\n")
        .mock_file("invalid_expression/module_loaded_later/_configured.scss", "$a: c !default;\n")
        .mock_file("invalid_expression/module_loaded_later/_other.scss", "$b: d;\n")
        .mock_file("invalid_expression/variable_defined_later/_other.scss", "$a: d !default;\n")
        .mock_file("multi_configuration/multi_file/_left.scss", "@use \"other\" with ($a: b);\n")
        .mock_file("multi_configuration/multi_file/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/multi_file/_right.scss", "@use \"other\" with ($a: b);\n")
        .mock_file("multi_configuration/one_file/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/through_forward/_forwarded.scss", "// This file defines no variables, but it still may not be loaded both with and\n// without configuration.\n")
        .mock_file("multi_configuration/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: c !default;\n")
        .mock_file("multi_configuration/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("namespace/_midstream.scss", "@use \"upstream\";\nupstream.$a: c !default;\n")
        .mock_file("namespace/_upstream.scss", "$a: d;\n")
        .mock_file("nested/_other.scss", "c {$a: d !default}\n")
        .mock_file("not_default/_other.scss", "$a: c;\n")
        .mock_file("through_forward/as/_forwarded.scss", "$a: d !default;\n")
        .mock_file("through_forward/as/_used.scss", "@forward \"forwarded\" as c-*;\n")
        .mock_file("through_forward/hide/_forwarded.scss", "$a: d !default;\n")
        .mock_file("through_forward/hide/_used.scss", "@forward \"forwarded\" hide $a;\n")
        .mock_file("through_forward/show/_forwarded.scss", "$a: d !default;\n")
        .mock_file("through_forward/show/_used.scss", "@forward \"forwarded\" show $b;\n")
        .mock_file("through_forward/with/_forwarded.scss", "$a: d !default;\n")
        .mock_file("through_forward/with/_used.scss", "@forward \"forwarded\" with ($a: c);\n")
        .mock_file("undefined/_other.scss", "// This file defines no variables.\n")
}

#[test]
#[ignore] // missing error
fn conflict() {
    let runner = runner().with_cwd("conflict");
    assert_eq!(
        runner.err("@use \"midstream\" with ($a: b);\n"),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"left\" as *;\
         \n    | ================ includes variable\
         \n2   | @use \"right\" as *;\
         \n    | ================= includes variable\
         \n... |\
         \n4   | $a: c !default;\
         \n    | ^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  _midstream.scss 4:1  @use\
         \n  input.scss 1:1       root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn core_module() {
    let runner = runner().with_cwd("core_module");
    assert_eq!(
        runner.err("@use \"sass:color\" with ($a: b);\n"),
        "Error: Built-in modules can\'t be configured.\
         \n  ,\
         \n1 | @use \"sass:color\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
mod invalid_expression {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("invalid_expression")
    }

    #[test]
    #[ignore] // missing error
    fn error() {
        let runner = runner().with_cwd("error");
        assert_eq!(
            runner.err("@use \"other\" with ($a: 1px + 1em);\n"),
            "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | @use \"other\" with ($a: 1px + 1em);\
         \n  |                        ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    fn module_loaded_later() {
        let runner = runner().with_cwd("module_loaded_later");
        assert_eq!(
            runner.err(
                "@use \"configured\" with ($a: other.$b);\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | @use \"configured\" with ($a: other.$b);\
         \n  |                             ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:29  root stylesheet",
        );
    }
    #[test]
    fn variable_defined_later() {
        let runner = runner().with_cwd("variable_defined_later");
        assert_eq!(
            runner.err(
                "@use \"other\" with ($a: $b);\
             \n$b: c;\n"
            ),
            "Error: Undefined variable.\
         \n  ,\
         \n1 | @use \"other\" with ($a: $b);\
         \n  |                        ^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
}
mod multi_configuration {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("multi_configuration")
    }

    #[test]
    #[ignore] // missing error
    fn multi_file() {
        let runner = runner().with_cwd("multi_file");
        assert_eq!(
        runner.err(
            "@use \"left\";\
             \n@use \"right\";\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _right.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _left.scss\
         \n1 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n  \'\
         \n  _right.scss 1:1  @use\
         \n  input.scss 2:1   root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn one_file() {
        let runner = runner().with_cwd("one_file");
        assert_eq!(
        runner.err(
            "@use \"other\" as o1 with ($a: b);\
             \n@use \"other\" as o2 with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1 with ($a: b);\
         \n  | =============================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn through_forward() {
        let runner = runner().with_cwd("through_forward");
        assert_eq!(
        runner.err(
            "@use \"forwarded\";\
             \n@use \"midstream\" with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n1 | @use \"forwarded\";\
         \n  | ================ original load\
         \n2 | @use \"midstream\" with ($a: b);\
         \n  | ============================= configuration\
         \n  \'\
         \n  _midstream.scss 1:1  @use\
         \n  input.scss 2:1       root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn unconfigured_first() {
        let runner = runner().with_cwd("unconfigured_first");
        assert_eq!(
        runner.err(
            "@use \"other\" as o1;\
             \n@use \"other\" as o2 with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n1 | @use \"other\" as o1;\
         \n  | ================== original load\
         \n2 | @use \"other\" as o2 with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong error
fn namespace() {
    let runner = runner().with_cwd("namespace");
    assert_eq!(
        runner.err(
            "@use \"midstream\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"midstream\" with ($a: b);\
         \n  |                        ^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.err(
            "@use \"other\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not_default() {
    let runner = runner().with_cwd("not_default");
    assert_eq!(
        runner.err(
            "@use \"other\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn repeated_variable() {
    let runner = runner().with_cwd("repeated_variable");
    assert_eq!(
        runner.err("@use \"other\" with ($a: b, $a: c);\n"),
        "Error: The same variable may only be configured once.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b, $a: c);\
         \n  |                           ^^^^^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
mod through_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    #[ignore] // missing error
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn with() {
        let runner = runner().with_cwd("with");
        assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn undefined() {
    let runner = runner().with_cwd("undefined");
    assert_eq!(
        runner.err(
            "@use \"other\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
