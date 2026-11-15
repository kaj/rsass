//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists/escape.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escape")
}

mod quoted {
    use super::runner;

    #[test]
    fn dash_insensitive() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$a-b: null;\
             \nc {d: meta.global-variable-exists(\"a\\\\5f b\")}\n"),
            "c {\
         \n  d: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_normalized() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(\"\\\\24\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn normalized() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(\"\\\\$\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn semantic() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(\"$\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod unquoted {
    use super::runner;

    #[test]
    fn dash_insensitive() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$a-b: null;\
             \nc {d: meta.global-variable-exists(unquoted(\"a\\\\5f b\"))}\n"
        ),
        "c {\
         \n  d: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_normalized() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(unquoted(\"\\\\24\"))}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn normalized() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(\\$)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn semantic() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$\\$: null;\
             \na {b: meta.global-variable-exists(string.unquote(\"$\"))}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
