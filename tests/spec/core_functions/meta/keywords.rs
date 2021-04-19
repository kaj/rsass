//! Tests auto-converted from "sass-spec/spec/core_functions/meta/keywords.hrx"

#[test]
#[ignore] // wrong result
fn dash_insensitive() {
    assert_eq!(
        crate::rsass(
            "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c-d: e, $f_g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c-d: e, f-g: h);\
        \n}\
        \n"
    );
}
mod empty {
    #[test]
    #[ignore] // wrong result
    fn no_args() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \na {b: inspect(args-to-keywords())}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn positional() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \na {b: inspect(args-to-keywords(1, 2, 3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: keywords()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $args.\
         \n  ,--> input.scss\
         \n1 | a {b: keywords()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: keywords(1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: keywords(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function keywords($args) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[test]
        #[ignore] // missing error
        fn non_arg_list() {
            assert_eq!(
                crate::rsass(
                    "a {b: keywords(1 2 3)}\
             \n"
                )
                .unwrap_err(),
                "Error: $args: 1 2 3 is not an argument list.\
         \n  ,\
         \n1 | a {b: keywords(1 2 3)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn non_list() {
            assert_eq!(
                crate::rsass(
                    "a {b: keywords(1)}\
             \n"
                )
                .unwrap_err(),
                "Error: $args: 1 is not an argument list.\
         \n  ,\
         \n1 | a {b: keywords(1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod forwarded {
    #[test]
    #[ignore] // unexepected error
    fn call() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \n\
            \n@function args-to-keywords-forward($args...) {\
            \n  @return call(get-function(\"args-to-keywords\"), $args...);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords-forward($c: d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn content() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \n\
            \n@mixin args-to-keywords-forward($args...) {\
            \n  @content($args...);\
            \n}\
            \n\
            \n@include args-to-keywords-forward($c: d) using ($args...) {\
            \n  a {b: inspect(args-to-keywords($args...))}\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn function() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \n\
            \n@function args-to-keywords-forward($args...) {\
            \n  @return args-to-keywords($args...);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords-forward($c: d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn mixin() {
        assert_eq!(
            crate::rsass(
                "@import \"../../utils\";\
            \n\
            \n@mixin args-to-keywords-forward($args...) {\
            \n  a {b: inspect(args-to-keywords($args...))}\
            \n}\
            \n\
            \n@include args-to-keywords-forward($c: d);\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn multi_arg() {
    assert_eq!(
        crate::rsass(
            "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c: d, $e: f, $g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, e: f, g: h);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "@function args-to-keywords($args...) {\
            \n  @return keywords($args: $args);\
            \n}\
            \n\
            \na {b: inspect(args-to-keywords($c: d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn one_arg() {
    assert_eq!(
        crate::rsass(
            "@import \"../utils\";\
            \na {b: inspect(args-to-keywords($c: d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d);\
        \n}\
        \n"
    );
}
