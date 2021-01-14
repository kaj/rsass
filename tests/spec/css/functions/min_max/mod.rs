//! Tests auto-converted from "sass-spec/spec/css/functions/min_max"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/functions/min_max/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "dangling_operator", error tests are not supported yet.

    // Ignoring "plain_css_function", error tests are not supported yet.
}

// From "sass-spec/spec/css/functions/min_max/plain_css.hrx"
#[test]
#[ignore] // wrong result
fn plain_css() {
    assert_eq!(
        rsass(
            "// If a min() or max() function would be a valid plain CSS value, possibly\
            \n// including interoplation, parse it as plain CSS.\
            \n\
            \n.plain-css {\
            \n  number: min(1px) max(1px);\
            \n  interpolation: min(#{1px + 2px}) max(#{1px + 2px});\
            \n  nested-min-max: min(max(1px, 2px)) max(min(1px, 2px));\
            \n  calc: min(calc(10% + 1px)) max(calc(10% + 1px));\
            \n  env: min(env(--foo), env(@&[*^{$(*)&}@^]%$), env(inter#{p + o}lated))\
            \n       max(env(--foo), env(@&[*^{$(*)&}@^]%$), env(inter#{p + o}lated));\
            \n  var: min(var(--foo), var(@&[*^{$(*)&}@^]%$), var(inter#{p + o}lated))\
            \n       max(var(--foo), var(@&[*^{$(*)&}@^]%$), var(inter#{p + o}lated));\
            \n  clamp: min(clamp(1, 2, 3)) max(clamp(1, 2, 3));\
            \n  operations: min(1px - 2px * 3px / 4px) max(1px - 2px * 3px / 4px);\
            \n  parens: min((1px + 2px) * 3px) max((1px + 2px) * 3px);\
            \n  two-arguments: min(1px, 2px) max(1px, 2px);\
            \n  three-arguments: min(1px, 2px, 3px) max(1px, 2px, 3px);\
            \n  case-insensitive: mIn(1px + 2px) MaX(1px + 2px);\
            \n}\
            \n"
        )
        .unwrap(),
        ".plain-css {\
        \n  number: min(1px) max(1px);\
        \n  interpolation: min(3px) max(3px);\
        \n  nested-min-max: min(max(1px, 2px)) max(min(1px, 2px));\
        \n  calc: min(calc(10% + 1px)) max(calc(10% + 1px));\
        \n  env: min(env(--foo), env(@&[*^{$(*)&}@^]%$), env(interpolated)) max(env(--foo), env(@&[*^{$(*)&}@^]%$), env(interpolated));\
        \n  var: min(var(--foo), var(@&[*^{$(*)&}@^]%$), var(interpolated)) max(var(--foo), var(@&[*^{$(*)&}@^]%$), var(interpolated));\
        \n  clamp: min(clamp(1, 2, 3)) max(clamp(1, 2, 3));\
        \n  operations: min(1px - 2px * 3px / 4px) max(1px - 2px * 3px / 4px);\
        \n  parens: min((1px + 2px) * 3px) max((1px + 2px) * 3px);\
        \n  two-arguments: min(1px, 2px) max(1px, 2px);\
        \n  three-arguments: min(1px, 2px, 3px) max(1px, 2px, 3px);\
        \n  case-insensitive: min(1px + 2px) max(1px + 2px);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/functions/min_max/sass_function.hrx"
#[test]
fn sass_function() {
    assert_eq!(
        rsass(
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
