//! Tests auto-converted from "sass-spec/spec/core_functions/meta/call.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("call")
}

mod args {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.call(meta.get-function(\"rgb\"), $blue: 1, $green: 2, $red: 3)}\n"
        ),
        "a {\
         \n  b: rgb(3, 2, 1);\
         \n}\n"
    );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@function a() {@return b}\
             \nc {d: meta.call(meta.get-function(\"a\"))}\n"),
            "c {\
         \n  d: b;\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.call(meta.get-function(\"rgb\"), 1, 2, 3)}\n"),
            "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
        );
    }
    mod splat {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn combined() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n$positional: 1 2;\
             \n$named: (\"blue\": 3);\
             \na {b: meta.call(meta.get-function(\"rgb\"), $positional..., $named...)}\n"
        ),
        "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
    );
        }
        #[test]
        fn named() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$args: (\"green\": 1, \"blue\": 2, \"red\": 3);\
             \na {b: meta.call(meta.get-function(\"rgb\"), $args...)}\n"),
                "a {\
         \n  b: rgb(3, 1, 2);\
         \n}\n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$args: 1, 2, 3;\
             \na {b: meta.call(meta.get-function(\"rgb\"), $args...)}\n"),
                "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
            );
        }
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn if_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n// The if() function has a special behavior to avoid evaluating the\
             \n// non-returned argument but that behavior does not propagate to call()\
             \n// itself when using call() to call if().\
             \na {b: meta.call(meta.get-function(\"if\"), true, \"\", $undefined)}\n"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n5 | a {b: meta.call(meta.get-function(\"if\"), true, \"\", $undefined)}\
         \n  |                                                    ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:52  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn invalid_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \na {b: meta.call(meta.get-function(\"rgb\"), 1)}\n"
        ),
        "Error: $channels: The rgb color space has 3 channels but 1 has 1.\
         \n  ,\
         \n2 | a {b: meta.call(meta.get-function(\"rgb\"), 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.call()}\n"
            ),
            "Error: Missing argument $function.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.call()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function call($function, $args...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.call(1)}\n"
            ),
            "Error: $function: 1 is not a function reference.\
         \n  ,\
         \n2 | a {b: meta.call(1)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.call($function: meta.get-function(\"rgb\"), $red: 1, $green: 2, $blue: 3)}\n"
        ),
        "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
    );
}
mod string {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn built_in() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.call(\"rgb\", 1, 2, 3)}\n"),
            "a {\
         \n  b: rgb(1, 2, 3);\
         \n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@function a($arg) {@return $arg + 1}\
             \na {b: meta.call(\"a\", 1)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
}
