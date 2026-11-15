//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/core_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("core_module")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
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
