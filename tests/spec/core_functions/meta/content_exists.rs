//! Tests auto-converted from "sass-spec/spec/core_functions/meta/content_exists.hrx"

mod error {
    #[test]
    #[ignore] // wrong error
    fn in_content() {
        assert_eq!(
            crate::rsass(
                "@mixin call-content {\
             \n  @content;\
             \n}\
             \n\
             \n@include call-content {\
             \n  a {b: content-exists()}\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n6 |   a {b: content-exists()}\
         \n  |         ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:9  @content\
         \n  input.scss 2:3  call-content()\
         \n  input.scss 5:1  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_function_called_by_mixin() {
        assert_eq!(
            crate::rsass(
                "@function call-content-exists() {\
             \n  @return content-exists();\
             \n}\
             \n\
             \n@mixin call-function {\
             \n  a {b: call-content-exists()};\
             \n}\
             \n\
             \n@include call-function;\
             \n"
            )
            .unwrap_err(),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n2 |   @return content-exists();\
         \n  |           ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:11  call-content-exists()\
         \n  input.scss 6:9   call-function()\
         \n  input.scss 9:1   root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn outside_mixin() {
        assert_eq!(
            crate::rsass(
                "a {b: content-exists()}\
             \n"
            )
            .unwrap_err(),
            "Error: content-exists() may only be called within a mixin.\
         \n  ,\
         \n1 | a {b: content-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@mixin a {\
             \n  b {c: content-exists(1)}\
             \n}\
             \n@include a;\
             \n"
            )
            .unwrap_err(),
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
         \n  input.scss 4:1  root stylesheet\
         \n",
        );
    }
}
mod test_false {
    #[test]
    #[ignore] // unexepected error
    fn through_content() {
        assert_eq!(
            crate::rsass(
                "@mixin call-content {\
            \n  @content;\
            \n}\
            \n\
            \n@mixin print-content-exists {\
            \n  a {b: content-exists()}\
            \n}\
            \n\
            \n@include call-content {\
            \n  @include print-content-exists;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn top_level() {
        assert_eq!(
            crate::rsass(
                "@mixin a {\
            \n  b {c: content-exists()}\
            \n}\
            \n@include a;\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: false;\
        \n}\
        \n"
        );
    }
}
mod test_true {
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@mixin a {\
            \n  b {c: content-exists()}\
            \n  @content;\
            \n}\
            \n@include a {}\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: true;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "@mixin a {\
            \n  b {c: content-exists()}\
            \n  @content;\
            \n}\
            \n@include a {\
            \n  d {e: f}\
            \n}\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: true;\
        \n}\
        \nd {\
        \n  e: f;\
        \n}\
        \n"
        );
    }
}
