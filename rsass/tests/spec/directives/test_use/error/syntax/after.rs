//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/after.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("after")
}

mod at_rule {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn css() {
        assert_eq!(
            runner().err(
                "@keyframes foo {};\
             \n@use \"other\";\n"
            ),
            "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn import() {
        assert_eq!(
            runner().err(
                "@import \"other1\";\
             \n@use \"other2\";\n"
            ),
            "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other2\";\
         \n  | ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn sass() {
        assert_eq!(
            runner().err(
                "@if true {};\
             \n@use \"other\";\n"
            ),
            "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn unknown() {
        assert_eq!(
            runner().err(
                "@fblthp;\
             \n@use \"other\";\n"
            ),
            "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
mod indented {
    #[allow(unused)]
    use super::runner;
}
#[test]
#[ignore] // wrong error
fn style_rule() {
    assert_eq!(
        runner().err(
            "a {};\
             \n@use \"other\";\n"
        ),
        "Error: @use rules must be written before any other rules.\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
