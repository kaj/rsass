//! Tests auto-converted from "sass-spec/spec/core_functions/meta/call.hrx"

mod args {
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
        crate::rsass(
            "a {b: call(get-function(\"rgb\"), $blue: 1, $green: 2, $red: 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #030201;\
        \n}\
        \n"
    );
    }
    #[test]
    fn none() {
        assert_eq!(
            crate::rsass(
                "@function a() {@return b}\
            \nc {d: call(get-function(\"a\"))}\
            \n"
            )
            .unwrap(),
            "c {\
        \n  d: b;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            crate::rsass(
                "a {b: call(get-function(\"rgb\"), 1, 2, 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #010203;\
        \n}\
        \n"
        );
    }
    mod splat {
        #[test]
        fn combined() {
            assert_eq!(
                crate::rsass(
                    "$positional: 1 2;\
            \n$named: (\"blue\": 3);\
            \na {b: call(get-function(\"rgb\"), $positional..., $named...)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #010203;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn named() {
            assert_eq!(
                crate::rsass(
                    "$args: (\"green\": 1, \"blue\": 2, \"red\": 3);\
            \na {b: call(get-function(\"rgb\"), $args...)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #030102;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                crate::rsass(
                    "$args: 1, 2, 3;\
            \na {b: call(get-function(\"rgb\"), $args...)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #010203;\
        \n}\
        \n"
            );
        }
    }
}
mod error {
    #[test]
    #[ignore] // wrong error
    fn if_args() {
        assert_eq!(
        crate::rsass(
            "// The if() function has a special behavior to avoid evaluating the\
             \n// non-returned argument but that behavior does not propagate to call()\
             \n// itself when using call() to call if().\
             \na {b: call(get-function(\"if\"), true, \"\", $undefined)}\
             \n"
        ).unwrap_err(),
        "Error: Undefined variable.\
         \n  ,\
         \n4 | a {b: call(get-function(\"if\"), true, \"\", $undefined)}\
         \n  |                                          ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:42  root stylesheet\
         \n",
    );
    }
    #[test]
    #[ignore] // missing error
    fn invalid_args() {
        assert_eq!(
            crate::rsass(
                "a {b: call(get-function(\"rgb\"), 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $green.\
         \n  ,\
         \n1 | a {b: call(get-function(\"rgb\"), 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: call()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $function.\
         \n  ,--> input.scss\
         \n1 | a {b: call()}\
         \n  |       ^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function call($function, $args...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: call(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $function: 1 is not a function reference.\
         \n  ,\
         \n1 | a {b: call(1)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: call($function: get-function(\"rgb\"), $red: 1, $green: 2, $blue: 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #010203;\
        \n}\
        \n"
    );
}
mod string {
    #[test]
    fn built_in() {
        assert_eq!(
            crate::rsass(
                "a {b: call(\"rgb\", 1, 2, 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #010203;\
        \n}\
        \n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
            crate::rsass(
                "@function a($arg) {@return $arg + 1}\
            \na {b: call(\"a\", 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
}
