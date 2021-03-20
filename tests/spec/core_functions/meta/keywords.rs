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

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "non_arg_list", error tests are not supported yet.

        // Ignoring "non_list", error tests are not supported yet.
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
