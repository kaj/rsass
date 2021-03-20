//! Tests auto-converted from "sass-spec/spec/core_functions/meta/content_exists.hrx"

mod error {

    // Ignoring "in_content", error tests are not supported yet.

    // Ignoring "in_function_called_by_mixin", error tests are not supported yet.

    // Ignoring "outside_mixin", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
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
