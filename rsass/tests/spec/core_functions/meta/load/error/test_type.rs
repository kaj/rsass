//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/type.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
#[ignore] // wrong error
fn url() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n$_: meta.load(1);\n"
        ),
        "Error: $url: 1 is not a string.\
         \n  ,\
         \n2 | $_: meta.load(1);\
         \n  |     ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
    );
}
mod with {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn key() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.load(\"other\", $with: (1: null));\n"
            ),
            "Error: $with key: 1 is not a string.\
         \n  ,\
         \n2 | $_: meta.load(\"other\", $with: (1: null));\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn map() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.load(\"other\", $with: 1);\n"
            ),
            "Error: $with: 1 is not a map.\
         \n  ,\
         \n2 | $_: meta.load(\"other\", $with: 1);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
}
