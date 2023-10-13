//! Tests auto-converted from "sass-spec/spec/values/mixins.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixins")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn addition() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@mixin a() {}\
             \n@mixin b() {}\
             \na {b: meta.get-mixin(a) + meta.get-mixin(b)}\n"
            ),
            "Error: get-mixin(\"a\") isn\'t a valid CSS value.\
         \n  ,\
         \n4 | a {b: meta.get-mixin(a) + meta.get-mixin(b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn division() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@mixin a() {}\
             \n@mixin b() {}\
             \na {b: meta.get-mixin(a) / meta.get-mixin(b)}\n"
            ),
            "Error: get-mixin(\"a\") isn\'t a valid CSS value.\
         \n  ,\
         \n4 | a {b: meta.get-mixin(a) / meta.get-mixin(b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn modulo() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@mixin a() {}\
             \n@mixin b() {}\
             \na {b: meta.get-mixin(a) % meta.get-mixin(b)}\n"
        ),
        "Error: Undefined operation \"get-mixin(\"a\") % get-mixin(\"b\")\".\
         \n  ,\
         \n4 | a {b: meta.get-mixin(a) % meta.get-mixin(b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn multiplication() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@mixin a() {}\
             \n@mixin b() {}\
             \na {b: meta.get-mixin(a) * meta.get-mixin(b)}\n"
        ),
        "Error: Undefined operation \"get-mixin(\"a\") * get-mixin(\"b\")\".\
         \n  ,\
         \n4 | a {b: meta.get-mixin(a) * meta.get-mixin(b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn subtraction() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@mixin a() {}\
             \n@mixin b() {}\
             \na {b: meta.get-mixin(a) - meta.get-mixin(b)}\n"
            ),
            "Error: get-mixin(\"a\") isn\'t a valid CSS value.\
         \n  ,\
         \n4 | a {b: meta.get-mixin(a) - meta.get-mixin(b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
}
