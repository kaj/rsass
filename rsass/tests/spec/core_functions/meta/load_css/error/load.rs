//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/load.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file(
            "loop/_other.scss",
            "@use \"sass:meta\";\n@include meta.load-css(\"input\");\n",
        )
        .mock_file(
            "nested/with_use/_midstream.scss",
            "@use 'upstream';\n@mixin b { c: d }\n@include b;\n",
        )
        .mock_file(
            "nested/with_use/_upstream.scss",
            "// Intentionally empty.\n",
        )
        .mock_file(
            "nested/without_use/_upstream.scss",
            "@mixin b { c: d }\n@include b;\n",
        )
        .mock_file(
            "top_level_include_declaration/_upstream.scss",
            "@mixin a { b: c }\n@include a;\n",
        )
        .mock_file(
            "loop/input.scss",
            "@use \"sass:meta\";\n@include meta.load-css(\"other\");\n",
        )
}

#[test]
#[ignore] // wrong error
fn test_loop() {
    let runner = runner().with_cwd("loop");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
        ),
        "Error: Module loop: input.scss is already being loaded.\
         \n  ,\
         \n2 | @include meta.load-css(\"input\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 2:1  load-css()\
         \n  input.scss 2:1   root stylesheet",
    );
}
#[test]
fn missing() {
    let runner = runner().with_cwd("missing");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\n"
        ),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
mod nested {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("nested")
    }

    #[test]
    #[ignore] // missing error
    fn with_use() {
        let runner = runner().with_cwd("with_use");
        assert_eq!(
            runner.err(
                "@use \'sass:meta\';\
             \n.a {\
             \n  @include meta.load-css(\'midstream\');\
             \n}\n"
            ),
            "Error: Declarations may only be used within style rules.\
         \n  ,\
         \n2 | @mixin b { c: d }\
         \n  |            ^^^^^\
         \n  \'\
         \n  _midstream.scss 2:12  b()\
         \n  _midstream.scss 3:1   load-css()\
         \n  input.scss 3:3        root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn without_use() {
        let runner = runner().with_cwd("without_use");
        assert_eq!(
            runner.err(
                "@use \'sass:meta\';\
             \n.a {\
             \n  @include meta.load-css(\'upstream\');\
             \n}\n"
            ),
            "Error: Declarations may only be used within style rules.\
         \n  ,\
         \n1 | @mixin b { c: d }\
         \n  |            ^^^^^\
         \n  \'\
         \n  _upstream.scss 1:12  b()\
         \n  _upstream.scss 2:1   load-css()\
         \n  input.scss 3:3       root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn top_level_include_declaration() {
    let runner = runner().with_cwd("top_level_include_declaration");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"upstream\");\n"
        ),
        "Error: Declarations may only be used within style rules.\
         \n  ,\
         \n1 | @mixin a { b: c }\
         \n  |            ^^^^^\
         \n  \'\
         \n  _upstream.scss 1:12  a()\
         \n  _upstream.scss 2:1   load-css()\
         \n  input.scss 2:1       root stylesheet",
    );
}
