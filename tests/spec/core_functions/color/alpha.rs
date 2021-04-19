//! Tests auto-converted from "sass-spec/spec/core_functions/color/alpha.hrx"

mod color {
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(red)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(rgba(red, 0.42))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.42;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(rgba(red, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha($color: rgba(red, 0.73))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.73;\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    #[ignore] // missing error
    fn quoted_string() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(\"c=d\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: \"c=d\" is not a color.\
         \n  ,\
         \n1 | a {b: alpha(\"c=d\")}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,\
         \n1 | a {b: alpha()}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(red, green)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: alpha(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: alpha(1)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod unquoted_string {
        #[test]
        #[ignore] // missing error
        fn no_equals() {
            assert_eq!(
                crate::rsass(
                    "a {b: alpha(cd)}\
             \n"
                )
                .unwrap_err(),
                "Error: $color: cd is not a color.\
         \n  ,\
         \n1 | a {b: alpha(cd)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn non_identifier_before_equals() {
            assert_eq!(
                crate::rsass(
                    "a {b: alpha(unquote(\"1=c\"))}\
             \n"
                )
                .unwrap_err(),
                "Error: $color: 1=c is not a color.\
         \n  ,\
         \n1 | a {b: alpha(unquote(\"1=c\"))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod filter {
    #[test]
    fn multi_args() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(c=d, e=f, g=h)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(c=d)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
        );
    }
    #[test]
    fn space_before_equals() {
        assert_eq!(
            crate::rsass(
                "a {b: alpha(unquote(\"c = d\"))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: alpha(c = d);\
        \n}\
        \n"
        );
    }
}
mod opacity {
    #[test]
    fn filter() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity(10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: opacity(10%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity($color: rgba(red, 0.2))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            crate::rsass(
                "a {b: opacity(rgba(red, 0.2))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.2;\
        \n}\
        \n"
        );
    }
}
