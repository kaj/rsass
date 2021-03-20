//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/same_module.hrx"

#[test]
fn built_in() {
    assert_eq!(
        crate::rsass(
            "$lighten-fn: get-function(lighten);\
            \n\
            \na {b: call($lighten-fn, red, 30%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ff9999;\
        \n}\
        \n"
    );
}
mod dash_insensitive {
    #[test]
    fn dash_to_underscore() {
        assert_eq!(
            crate::rsass(
                "@function add_two($v) {@return $v + 2}\
            \n\
            \na {b: call(get-function(add-two), 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 12;\
        \n}\
        \n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        assert_eq!(
            crate::rsass(
                "@function add-two($v) {@return $v + 2}\
            \n\
            \na {b: call(get-function(add_two), 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 12;\
        \n}\
        \n"
        );
    }
}
#[test]
fn plain_css() {
    assert_eq!(
        crate::rsass(
            "$sass-fn: get-function(lighten);\
            \n$css-fn: get-function(lighten, $css: true);\
            \n\
            \na {\
            \n  sass-fn: call($sass-fn, red, 30%);\
            \n  css-fn: call($css-fn, red, 30%);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  sass-fn: #ff9999;\
        \n  css-fn: lighten(red, 30%);\
        \n}\
        \n"
    );
}
#[test]
fn redefined() {
    assert_eq!(
        crate::rsass(
            "@function add-two($v) {@return $v + 2}\
            \n$add-two-fn: get-function(add-two);\
            \n\
            \n// The function returned by `get-function()` is locked in place when it\'s\
            \n// called. Redefining the function after the fact shouldn\'t affect the stored\
            \n// value.\
            \n@function add-two($v) {@error \"Should not be called\"}\
            \n\
            \na {b: call($add-two-fn, 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    assert_eq!(
        crate::rsass(
            "@import \"other\";\
            \na {b: call(get-function(add-two), 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
#[test]
fn user_defined() {
    assert_eq!(
        crate::rsass(
            "@function add-two($v) {@return $v + 2}\
            \n$add-two-fn: get-function(add-two);\
            \n\
            \na {b: call($add-two-fn, 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
