//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists/same_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("same_module").mock_file(
        "through_import/other.scss",
        "@function global-function() {@return null}\n",
    )
}

mod dash_insensitive {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dash_insensitive")
    }

    #[test]
    fn dash_to_underscore() {
        let runner = runner().with_cwd("dash_to_underscore");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@function a_b() {@return null}\n\
             \nc {d: meta.function-exists(a-b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        let runner = runner().with_cwd("underscore_to_dash");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@function a-b() {@return null}\n\
             \nc {d: meta.function-exists(a_b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
}
mod escape {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("escape")
    }

    mod quoted {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("quoted")
        }

        #[test]
        fn dash_insensitive() {
            let runner = runner().with_cwd("dash_insensitive");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@function a-b() {@return null}\
             \nc {d: meta.function-exists(\"a\\\\5f b\")}\n"),
                "c {\
         \n  d: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn non_normalized() {
            let runner = runner().with_cwd("non_normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(\"\\\\24\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn normalized() {
            let runner = runner().with_cwd("normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(\"\\\\$\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn semantic() {
            let runner = runner().with_cwd("semantic");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(\"$\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    mod unquoted {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("unquoted")
        }

        #[test]
        fn dash_insensitive() {
            let runner = runner().with_cwd("dash_insensitive");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@function a-b() {@return null}\
             \nc {d: meta.function-exists(unquoted(\"a\\\\5f b\"))}\n"),
                "c {\
         \n  d: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn non_normalized() {
            let runner = runner().with_cwd("non_normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(unquoted(\"\\\\24\"))}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn normalized() {
            let runner = runner().with_cwd("normalized");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(\\$)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn semantic() {
            let runner = runner().with_cwd("semantic");
            assert_eq!(
                runner.ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n@function \\$() {@return null}\
             \na {b: meta.function-exists(string.unquote(\"$\"))}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
#[test]
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@function global-function() {@return null}\n\
             \na {b: meta.function-exists(global-function)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn local() {
    let runner = runner().with_cwd("local");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \na {\
             \n  @function local-function() {@return null}\
             \n  b: meta.function-exists(local-function);\
             \n}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn non_existent() {
    let runner = runner().with_cwd("non_existent");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \na {\
             \n  b: meta.function-exists(non-existent);\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@import \"other\";\
             \na {b: meta.function-exists(global-function)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
