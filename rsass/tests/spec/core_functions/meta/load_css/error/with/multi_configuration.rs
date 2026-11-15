//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/multi_configuration.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multi_configuration")
        .mock_file("double_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("double_load/through_forward/_forwarded.scss", "// This only throws an error because it defines a configurable variable.\n$c: f !default;\n")
        .mock_file("double_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: e !default;\n")
        .mock_file("double_load/unconfigured_first/_other.scss", "$a: c !default;\n")
        .mock_file("use_and_load/both_configured/_other.scss", "$a: c !default;\n")
        .mock_file("use_and_load/load_first/_loads.scss", "@use \"sass:meta\";\n@include meta.load-css(\"other\", $with: (a: b));\n")
        .mock_file("use_and_load/load_first/_other.scss", "$a: c !default;\n")
        .mock_file("use_and_load/through_forward/_forwarded.scss", "// This only throws an error because it defines a configurable variable.\n$c: f !default;\n")
        .mock_file("use_and_load/through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: e !default;\n")
        .mock_file("use_and_load/unconfigured_first/_other.scss", "$a: c !default;\n")
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
