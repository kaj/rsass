//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/equality.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("equality")
}

mod built_in {
    use super::runner;

    #[test]
    fn different() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.get-function(lighten) == meta.get-function(darken)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn same() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.get-function(lighten) == meta.get-function(lighten)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
#[test]
fn same_value() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$lighten-fn: meta.get-function(lighten);\
             \na {b: $lighten-fn == $lighten-fn}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod user_defined {
    use super::runner;

    #[test]
    fn different() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@function user-defined-1() {@return null}\
             \n@function user-defined-2() {@return null}\
             \na {b: meta.get-function(user-defined-1) == meta.get-function(user-defined-2)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn redefined() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@function user-defined() {@return null}\
             \n$first-reference: meta.get-function(user-defined);\n\
             \n@function user-defined() {@return null}\
             \n$second-reference: meta.get-function(user-defined);\
             \na {b: $first-reference == $second-reference}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@function user-defined() {@return null}\
             \na {b: meta.get-function(user-defined) == meta.get-function(user-defined)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
