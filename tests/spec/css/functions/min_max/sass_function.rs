//! Tests auto-converted from "sass-spec/spec/css/functions/min_max/sass_function.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// If a min() or max() function uses a SassScript feature, parse it as Sass.\
            \n\
            \n.sass-function {\
            \n  $two-px: 2px;\
            \n  variable: min(1px, $two-px) max(1px, $two-px);\
            \n\
            \n  function: min(1px, floor(10.6px)) max(1px, floor(10.6px));\
            \n  nested-min-max: min(1px, max($two-px, 3px)) max(1px, min($two-px, 3px));\
            \n  modulo: min(1px, 7px % 4) max(1px, 7px % 4);\
            \n  trailing-comma: min(1px, 2px,) max(1px, 2px,);\
            \n}\
            \n"
        )
        .unwrap(),
        ".sass-function {\
        \n  variable: 1px 2px;\
        \n  function: 1px 10px;\
        \n  nested-min-max: 1px 2px;\
        \n  modulo: 1px 3px;\
        \n  trailing-comma: 1px 2px;\
        \n}\
        \n"
    );
}
