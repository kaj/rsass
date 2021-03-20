//! Tests auto-converted from "sass-spec/spec/css/functions/min_max/plain_css.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
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
