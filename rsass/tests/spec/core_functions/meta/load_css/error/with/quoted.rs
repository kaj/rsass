//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/quoted.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("quoted")
        .mock_file(
            "dash_insensitive/_other.scss",
            "$a-b: original !default;\nc {d: $a-b}\n",
        )
        .mock_file(
            "non_normalized/_other.scss",
            "$\\$: original !default;\na {b: $\\$}\n",
        )
        .mock_file(
            "semantic/_other.scss",
            "$\\$: original !default;\na {b: $\\$}\n",
        )
}

#[test]
#[ignore] // missing error
fn dash_insensitive() {
    let runner = runner().with_cwd("dash_insensitive");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"a\\\\5f b\": override));\n"
        ),
        "Error: $a\\5f b was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (\"a\\\\5f b\": override));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn non_normalized() {
    let runner = runner().with_cwd("non_normalized");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"\\\\24\": override));\n"
        ),
        "Error: $\\24 was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (\"\\\\24\": override));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn semantic() {
    let runner = runner().with_cwd("semantic");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"$\": override));\n"
        ),
        "Error: $$ was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (\"$\": override));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
