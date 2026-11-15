//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/repeated_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("repeated_variable")
        .mock_file("_other.scss", "$a-b: c !default;\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\n"
        ),
        "Error: The variable $a_b was configured twice.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
