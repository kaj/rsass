//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/namespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("namespace")
        .mock_file(
            "_midstream.scss",
            "@use \"upstream\";\nupstream.$a: c !default;\n",
        )
        .mock_file("_upstream.scss", "$a: d;\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
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
