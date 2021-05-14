//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/equality.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod built_in {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different() {
        assert_eq!(
            runner()
                .ok("a {b: get-function(lighten) == get-function(darken)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok(
                "a {b: get-function(lighten) == get-function(lighten)}\n"
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
        runner().ok("$lighten-fn: get-function(lighten);\
             \na {b: $lighten-fn == $lighten-fn}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod user_defined {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different() {
        assert_eq!(
        runner().ok(
            "@function user-defined-1() {@return null}\
             \n@function user-defined-2() {@return null}\
             \na {b: get-function(user-defined-1) == get-function(user-defined-2)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn redefined() {
        assert_eq!(
            runner().ok("@function user-defined() {@return null}\
             \n$first-reference: get-function(user-defined);\n\
             \n@function user-defined() {@return null}\
             \n$second-reference: get-function(user-defined);\
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
            "@function user-defined() {@return null}\
             \na {b: get-function(user-defined) == get-function(user-defined)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
