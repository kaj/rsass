//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("with")
        .mock_file("conflict/_left.scss", "$a: left;\n")
        .mock_file("conflict/_midstream.scss", "@use \"left\" as *;\n@use \"right\" as *;\n\n$a: c !default;\n")
        .mock_file("conflict/_right.scss", "$a: right;\n")
        .mock_file("multi_configuration/double_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/double_load/through_forward/_forwarded.scss", "// This only throws an error because it defines a configurable variable.\n$c: f !default;\n")
        .mock_file("multi_configuration/double_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: e !default;\n")
        .mock_file("multi_configuration/double_load/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/use_and_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/use_and_load/load_first/_loads.scss", "@use \"sass:meta\";\n@include meta.load-css(\"other\", $with: (a: b));\n")
        .mock_file("multi_configuration/use_and_load/load_first/_other.scss", "$a: c !default;\n")
        .mock_file("multi_configuration/use_and_load/through_forward/_forwarded.scss", "// This only throws an error because it defines a configurable variable.\n$c: f !default;\n")
        .mock_file("multi_configuration/use_and_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: e !default;\n")
        .mock_file("multi_configuration/use_and_load/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("namespace/_midstream.scss", "@use \"upstream\";\nupstream.$a: c !default;\n")
        .mock_file("namespace/_upstream.scss", "$a: d;\n")
        .mock_file("nested/_other.scss", "c {$a: d !default}\n")
        .mock_file("not_default/_other.scss", "$a: c;\n")
        .mock_file("repeated_variable/_other.scss", "$a-b: c !default;\n")
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
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
        ),
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
         \n  _midstream.scss 4:1  load-css()\
         \n  input.scss 2:1       root stylesheet",
    );
}
#[test]
fn core_module() {
    let runner = runner().with_cwd("core_module");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"sass:color\", $with: (a: b));\n"
        ),
        "Error: Built-in module sass:color can\'t be configured.\
         \n  ,\
         \n2 | @include meta.load-css(\"sass:color\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
mod multi_configuration {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("multi_configuration")
    }

    mod double_load {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("double_load")
        }

        #[test]
        #[ignore] // missing error
        fn both_configured() {
            let runner = runner().with_cwd("both_configured");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn through_forward() {
            let runner = runner().with_cwd("through_forward");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"forwarded\");\
             \n@include meta.load-css(\"midstream\", $with: (a: b, c: d));\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"forwarded\");\
         \n  | =================================== original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b, c: d));\
         \n  | ======================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn unconfigured_first() {
            let runner = runner().with_cwd("unconfigured_first");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | =============================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
        }
    }
    mod use_and_load {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("use_and_load")
        }

        #[test]
        #[ignore] // missing error
        fn both_configured() {
            let runner = runner().with_cwd("both_configured");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other\" with ($a: b);\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn load_first() {
            let runner = runner().with_cwd("load_first");
            assert_eq!(
        runner.err(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
             \n// we begin loading `used`.\
             \n@use \"loads\";\
             \n@use \"other\" with ($a: b);\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> input.scss\
         \n4 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _loads.scss\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n  \'\
         \n  input.scss 4:1  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn through_forward() {
            let runner = runner().with_cwd("through_forward");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"forwarded\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b, c: d));\n"
        ),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @use \"forwarded\";\
         \n  | ================ original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b, c: d));\
         \n  | ======================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn unconfigured_first() {
            let runner = runner().with_cwd("unconfigured_first");
            assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ============ original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
        }
    }
}
#[test]
#[ignore] // missing error
fn namespace() {
    let runner = runner().with_cwd("namespace");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not_default() {
    let runner = runner().with_cwd("not_default");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn repeated_variable() {
    let runner = runner().with_cwd("repeated_variable");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\n"
        ),
        "Error: The variable $a-b was configured twice.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
mod through_forward {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    #[ignore] // missing error
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn with() {
        let runner = runner().with_cwd("with");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\n"
            ),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn undefined() {
    let runner = runner().with_cwd("undefined");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\n"
        ),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
