//! Tests auto-converted from "sass-spec/spec/core_functions/meta/content_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("content_exists")
}

mod controls {
    use super::runner;

    #[test]
    fn test_false() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n// Regression test for sass/libsass#2842\
             \n@mixin test-content-exists() {\
             \n  @if meta.content-exists() {\
             \n    @content;\
             \n  }\
             \n  @else {\
             \n    content-exists: false;\
             \n  }\
             \n}\n\
             \na {\
             \n  @include test-content-exists();\
             \n}\n"),
            "a {\
         \n  content-exists: false;\
         \n}\n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n// Regression test for sass/libsass#2842\
             \n@mixin test-content-exists() {\
             \n  @if meta.content-exists() {\
             \n    @content;\
             \n  }\
             \n  @else {\
             \n    content-exists: false;\
             \n  }\
             \n}\n\
             \na {\
             \n  @include test-content-exists() {\
             \n    content: present;\
             \n  }\
             \n}\n"),
            "a {\
         \n  content: present;\
         \n}\n"
        );
    }
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn in_content() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@mixin call-content {\
             \n  @content;\
             \n}\n\
             \n@include call-content {\
             \n  a {b: meta.content-exists()}\
             \n}\n"
            ),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n7 |   a {b: meta.content-exists()}\
         \n  |         ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 7:9  @content\
         \n  input.scss 3:3  call-content()\
         \n  input.scss 6:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn in_function_called_by_mixin() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@function call-content-exists() {\
             \n  @return meta.content-exists();\
             \n}\n\
             \n@mixin call-function {\
             \n  a {b: call-content-exists()};\
             \n}\n\
             \n@include call-function;\n"
            ),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n3 |   @return meta.content-exists();\
         \n  |           ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  call-content-exists()\
         \n  input.scss 7:9   call-function()\
         \n  input.scss 10:1  root stylesheet",
        );
    }
    #[test]
    fn outside_mixin() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.content-exists()}\n"
            ),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n2 | a {b: meta.content-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@mixin a {\
             \n  b {c: meta.content-exists(1)}\
             \n}\
             \n@include a;\n"
            ),
            "Error: Only 0 arguments allowed, but 1 was passed.\
         \n  ,--> input.scss\
         \n3 |   b {c: meta.content-exists(1)}\
         \n  |         ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function content-exists() {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 3:9  a()\
         \n  input.scss 5:1  root stylesheet",
        );
    }
}
mod test_false {
    use super::runner;

    #[test]
    fn through_content() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin call-content {\
             \n  @content;\
             \n}\n\
             \n@mixin print-content-exists {\
             \n  a {b: meta.content-exists()}\
             \n}\n\
             \n@include call-content {\
             \n  @include print-content-exists;\
             \n}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn top_level() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin a {\
             \n  b {c: meta.content-exists()}\
             \n}\
             \n@include a;\n"),
            "b {\
         \n  c: false;\
         \n}\n"
        );
    }
}
mod test_true {
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin a {\
             \n  b {c: meta.content-exists()}\
             \n  @content;\
             \n}\
             \n@include a {}\n"),
            "b {\
         \n  c: true;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin a {\
             \n  b {c: meta.content-exists()}\
             \n  @content;\
             \n}\
             \n@include a {\
             \n  d {e: f}\
             \n}\n"),
            "b {\
         \n  c: true;\
         \n}\
         \nd {\
         \n  e: f;\
         \n}\n"
        );
    }
}
