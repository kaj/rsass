//! Tests auto-converted from "sass-spec/spec/core_functions/meta/call.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod args {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
        runner().ok(
            "a {b: call(get-function(\"rgb\"), $blue: 1, $green: 2, $red: 3)}\n"
        ),
        "a {\
         \n  b: #030201;\
         \n}\n"
    );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("@function a() {@return b}\
             \nc {d: call(get-function(\"a\"))}\n"),
            "c {\
         \n  d: b;\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            runner().ok("a {b: call(get-function(\"rgb\"), 1, 2, 3)}\n"),
            "a {\
         \n  b: #010203;\
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
            "$positional: 1 2;\
             \n$named: (\"blue\": 3);\
             \na {b: call(get-function(\"rgb\"), $positional..., $named...)}\n"
        ),
        "a {\
         \n  b: #010203;\
         \n}\n"
    );
        }
        #[test]
        fn named() {
            assert_eq!(
                runner()
                    .ok("$args: (\"green\": 1, \"blue\": 2, \"red\": 3);\
             \na {b: call(get-function(\"rgb\"), $args...)}\n"),
                "a {\
         \n  b: #030102;\
         \n}\n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                runner().ok("$args: 1, 2, 3;\
             \na {b: call(get-function(\"rgb\"), $args...)}\n"),
                "a {\
         \n  b: #010203;\
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
            "// The if() function has a special behavior to avoid evaluating the\
             \n// non-returned argument but that behavior does not propagate to call()\
             \n// itself when using call() to call if().\
             \na {b: call(get-function(\"if\"), true, \"\", $undefined)}\n"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n4 | a {b: call(get-function(\"if\"), true, \"\", $undefined)}\
         \n  |                                          ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:42  root stylesheet",
    );
    }
    #[test]
    fn invalid_args() {
        assert_eq!(
            runner().err("a {b: call(get-function(\"rgb\"), 1)}\n"),
            "Error: Missing element $green.\
         \n  ,\
         \n1 | a {b: call(get-function(\"rgb\"), 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: call()}\n"),
            "Error: Missing argument $function.\
         \n  ,--> input.scss\
         \n1 | a {b: call()}\
         \n  |       ^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function call($function, $args...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: call(1)}\n"),
            "Error: $function: 1 is not a function reference.\
         \n  ,\
         \n1 | a {b: call(1)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: call($function: get-function(\"rgb\"), $red: 1, $green: 2, $blue: 3)}\n"
        ),
        "a {\
         \n  b: #010203;\
         \n}\n"
    );
}
mod string {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn built_in() {
        assert_eq!(
            runner().ok("a {b: call(\"rgb\", 1, 2, 3)}\n"),
            "a {\
         \n  b: #010203;\
         \n}\n"
        );
    }
    #[test]
    fn local() {
        assert_eq!(
            runner().ok("@function a($arg) {@return $arg + 1}\
             \na {b: call(\"a\", 1)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
}
