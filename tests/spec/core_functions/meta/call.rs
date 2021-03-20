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

    // Ignoring "if_args", error tests are not supported yet.

    // Ignoring "invalid_args", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
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
