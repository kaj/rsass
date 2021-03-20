//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/equality.hrx"

mod built_in {
    #[test]
    fn different() {
        assert_eq!(
            crate::rsass(
                "a {b: get-function(lighten) == get-function(darken)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "a {b: get-function(lighten) == get-function(lighten)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
#[test]
fn same_value() {
    assert_eq!(
        crate::rsass(
            "$lighten-fn: get-function(lighten);\
            \na {b: $lighten-fn == $lighten-fn}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
mod user_defined {
    #[test]
    fn different() {
        assert_eq!(
        crate::rsass(
            "@function user-defined-1() {@return null}\
            \n@function user-defined-2() {@return null}\
            \na {b: get-function(user-defined-1) == get-function(user-defined-2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    #[test]
    fn redefined() {
        assert_eq!(
            crate::rsass(
                "@function user-defined() {@return null}\
            \n$first-reference: get-function(user-defined);\
            \n\
            \n@function user-defined() {@return null}\
            \n$second-reference: get-function(user-defined);\
            \na {b: $first-reference == $second-reference}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "@function user-defined() {@return null}\
            \na {b: get-function(user-defined) == get-function(user-defined)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
