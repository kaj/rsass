//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/undefined.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("undefined")
        .mock_file("_other.scss", "// This file defines no variables.\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
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
