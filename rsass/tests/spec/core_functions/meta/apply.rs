//! Tests auto-converted from "sass-spec/spec/core_functions/meta/apply.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("apply")
}

mod args {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn named() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\n\
             \n@mixin a {\
             \n  b: c;\
             \n}\n\
             \na {@include meta.apply($mixin: meta.get-mixin(\"a\"))}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    mod passes {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn named() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a) {b: $a}\n\
             \na {@include meta.apply(meta.get-mixin(\"a\"), $a: c)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positional() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a) {b: $a}\n\
             \na {@include meta.apply(meta.get-mixin(\"a\"), c)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        mod rest {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn named() {
                assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\n\
             \n@mixin a($a...) {b: meta.inspect(meta.keywords($a))}\n\
             \na {@include meta.apply(meta.get-mixin(\"a\"), $a: a, $b: b, $c: c)}\n"
        ),
        "a {\
         \n  b: (a: a, b: b, c: c);\
         \n}\n"
    );
            }
            #[test]
            #[ignore] // unexepected error
            fn positional() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a...) {b: $a}\n\
             \na {@include meta.apply(meta.get-mixin(\"a\"), a, b, c)}\n"),
                    "a {\
         \n  b: a, b, c;\
         \n}\n"
                );
            }
        }
    }
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn missing_mixin_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \n@mixin a($a) {}\
             \n$a: meta.get-mixin(\"a\");\n\
             \na {@include meta.apply($a)}\n"
            ),
            "Error: Missing argument $a.\
         \n    ,\
         \n3   | @mixin a($a) {}\
         \n    |        ===== declaration\
         \n... |\
         \n6   | a {@include meta.apply($a)}\
         \n    |    ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 6:4  a()\
         \n  input.scss 6:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn no_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \na {@include meta.apply()}\n"
            ),
            "Error: Missing argument $mixin.\
         \n  ,--> input.scss\
         \n3 | a {@include meta.apply()}\
         \n  |    ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin apply($mixin, $args...) {\
         \n  |        ======================= declaration\
         \n  \'\
         \n  input.scss 3:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \n@mixin a {}\
             \n$a: meta.get-mixin(\"a\");\n\
             \na {@include meta.apply($a, 2px)}\n"
            ),
            "Error: Only 0 arguments allowed, but 1 was passed.\
         \n    ,\
         \n3   | @mixin a {}\
         \n    |        = declaration\
         \n... |\
         \n6   | a {@include meta.apply($a, 2px)}\
         \n    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 6:4  a()\
         \n  input.scss 6:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args_mixin_accepts_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \n@mixin a($a) {}\
             \n$a: meta.get-mixin(\"a\");\n\
             \na {@include meta.apply($a, 2px, 3px)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n    ,\
         \n3   | @mixin a($a) {}\
         \n    |        ===== declaration\
         \n... |\
         \n6   | a {@include meta.apply($a, 2px, 3px)}\
         \n    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 6:4  a()\
         \n  input.scss 6:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn use_as_function() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \n@mixin a {}\
             \n$a: meta.get-mixin(\"a\");\n\
             \na {b: meta.apply($a)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n6 | a {b: meta.apply($a)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn wrong_arg_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \na {@include meta.apply(2px)}\n"
            ),
            "Error: $mixin: 2px is not a mixin reference.\
         \n  ,\
         \n3 | a {@include meta.apply(2px)}\
         \n  |    ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:4  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn wrong_named_arg() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n@mixin a($a) {b: $a}\n\
             \na {@include meta.apply(meta.get-mixin(\"a\"), $b: c)}\n"
        ),
        "Error: Missing argument $a.\
         \n    ,\
         \n3   | @mixin a($a) {b: $a}\
         \n    |        ===== declaration\
         \n... |\
         \n5   | a {@include meta.apply(meta.get-mixin(\"a\"), $b: c)}\
         \n    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 5:4  a()\
         \n  input.scss 5:4  root stylesheet",
    );
    }
}
mod rest {
    use super::runner;

    mod includes_mixin {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn named() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a, $b) {b: $a, $b}\n\
             \n$rest: (mixin: meta.get-mixin(\"a\"), a: 1, b: 2);\
             \na {@include meta.apply($rest...)}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positional() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a, $b) {b: $a, $b}\n\
             \n$rest: meta.get-mixin(\"a\") 1 2;\
             \na {@include meta.apply($rest...)}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
    }
    mod mixin_separate {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn named() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a, $b) {b: $a, $b}\n\
             \n$rest: (a: 1, b: 2);\
             \na {@include meta.apply(meta.get-mixin(\"a\"), $rest...)}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positional() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\n\
             \n@mixin a($a, $b) {b: $a, $b}\n\
             \n$rest: 1 2;\
             \na {@include meta.apply(meta.get-mixin(\"a\"), $rest...)}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
    }
}
