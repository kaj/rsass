//! Tests auto-converted from "sass-spec/spec/values/modules.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("modules")
}

mod error {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn addition() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.get-module(meta) + meta.get-module(meta);\n"
            ),
            "Error: Undefined operation \"get-module(\"meta\") + get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: meta.get-module(meta) + meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn division() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.get-module(meta) / meta.get-module(meta);\n"
            ),
            "Error: Undefined operation \"get-module(\"meta\") / get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: meta.get-module(meta) / meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn modulo() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.get-module(meta) % meta.get-module(meta);\n"
            ),
            "Error: Undefined operation \"get-module(\"meta\") % get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: meta.get-module(meta) % meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn multiplication() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.get-module(meta) * meta.get-module(meta);\n"
            ),
            "Error: Undefined operation \"get-module(\"meta\") * get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: meta.get-module(meta) * meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn subtraction() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$_: meta.get-module(meta) - meta.get-module(meta);\n"
            ),
            "Error: Undefined operation \"get-module(\"meta\") - get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: meta.get-module(meta) - meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    mod unary {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn minus() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\
             \n$_: -(meta.get-module(meta));\n"
                ),
                "Error: Undefined operation \"-(get-module(\"meta\"))\".\
         \n  ,\
         \n2 | $_: -(meta.get-module(meta));\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn plus() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\
             \n$_: +meta.get-module(meta);\n"
                ),
                "Error: Undefined operation \"+get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: +meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn slash() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\
             \n$_: /meta.get-module(meta);\n"
                ),
                "Error: Undefined operation \"/get-module(\"meta\")\".\
         \n  ,\
         \n2 | $_: /meta.get-module(meta);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
            );
        }
    }
}
