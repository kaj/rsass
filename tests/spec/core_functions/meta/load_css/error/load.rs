//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/load.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file(
            "loop/_other.scss",
            "@use \"sass:meta\";\n@include meta.load-css(\"input\");\n",
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
