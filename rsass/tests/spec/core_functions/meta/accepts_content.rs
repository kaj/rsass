//! Tests auto-converted from "sass-spec/spec/core_functions/meta/accepts_content.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("accepts_content")
}

mod accepts {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn builtin() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\n\
             \na {b: meta.accepts-content(meta.get-mixin(apply, meta))}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn direct_child() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\n\
             \n@mixin a() {@content}\n\
             \na {b: meta.accepts-content(meta.get-mixin(\"a\"))}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn nested_child() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\n\
             \n@mixin a() {\
             \n  @if false {@content}\
             \n}\n\
             \na {b: meta.accepts-content(meta.get-mixin(\"a\"))}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod args {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn keyword() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\n\
             \na {b: meta.accepts-content($mixin: meta.get-mixin(apply, meta))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
mod doesnt_accept {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn builtin() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@mixin a() {}\n\
             \na {b: meta.accepts-content(meta.get-mixin(load-css, meta))}\n"
            ),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin a() {}\n\
             \na {b: meta.accepts-content(meta.get-mixin(\"a\"))}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod error {
    use super::runner;

    mod args {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn too_few() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\n\
             \na {b: meta.accepts-content()}\n"
                ),
                "Error: Missing argument $mixin.\
         \n  ,--> input.scss\
         \n3 | a {b: meta.accepts-content()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function accepts-content($mixin) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn too_many() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\
             \n@mixin a() {}\n\
             \na {b: meta.accepts-content(a, a)}\n"
                ),
                "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n4 | a {b: meta.accepts-content(a, a)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function accepts-content($mixin) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 4:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn wrong_type() {
            assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \na {b: meta.accepts-content(meta.get-function(\"red\"))}\n"
        ),
        "Error: $mixin: get-function(\"red\") is not a mixin reference.\
         \n  ,\
         \n3 | a {b: meta.accepts-content(meta.get-function(\"red\"))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
        }
    }
}
