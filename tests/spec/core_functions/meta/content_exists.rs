//! Tests auto-converted from "sass-spec/spec/core_functions/meta/content_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod controls {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn test_false() {
        assert_eq!(
            runner().ok("// Regression test for sass/libsass#2842\
             \n@mixin test-content-exists() {\
             \n  @if content-exists() {\
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
    #[ignore] // wrong result
    fn test_true() {
        assert_eq!(
            runner().ok("// Regression test for sass/libsass#2842\
             \n@mixin test-content-exists() {\
             \n  @if content-exists() {\
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
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // wrong error
    fn in_content() {
        assert_eq!(
            runner().err(
                "@mixin call-content {\
             \n  @content;\
             \n}\n\
             \n@include call-content {\
             \n  a {b: content-exists()}\
             \n}\n"
            ),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n6 |   a {b: content-exists()}\
         \n  |         ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:9  @content\
         \n  input.scss 2:3  call-content()\
         \n  input.scss 5:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_function_called_by_mixin() {
        assert_eq!(
            runner().err(
                "@function call-content-exists() {\
             \n  @return content-exists();\
             \n}\n\
             \n@mixin call-function {\
             \n  a {b: call-content-exists()};\
             \n}\n\
             \n@include call-function;\n"
            ),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n2 |   @return content-exists();\
         \n  |           ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:11  call-content-exists()\
         \n  input.scss 6:9   call-function()\
         \n  input.scss 9:1   root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn outside_mixin() {
        assert_eq!(
            runner().err("a {b: content-exists()}\n"),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n1 | a {b: content-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@mixin a {\
             \n  b {c: content-exists(1)}\
             \n}\
             \n@include a;\n"
            ),
            "Error: Only 0 arguments allowed, but 1 was passed.\
         \n  ,--> input.scss\
         \n2 |   b {c: content-exists(1)}\
         \n  |         ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function content-exists() {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:9  a()\
         \n  input.scss 4:1  root stylesheet",
        );
    }
}
mod test_false {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn through_content() {
        assert_eq!(
            runner().ok("@mixin call-content {\
             \n  @content;\
             \n}\n\
             \n@mixin print-content-exists {\
             \n  a {b: content-exists()}\
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
            runner().ok("@mixin a {\
             \n  b {c: content-exists()}\
             \n}\
             \n@include a;\n"),
            "b {\
         \n  c: false;\
         \n}\n"
        );
    }
}
mod test_true {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
            runner().ok("@mixin a {\
             \n  b {c: content-exists()}\
             \n  @content;\
             \n}\
             \n@include a {}\n"),
            "b {\
         \n  c: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_empty() {
        assert_eq!(
            runner().ok("@mixin a {\
             \n  b {c: content-exists()}\
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
