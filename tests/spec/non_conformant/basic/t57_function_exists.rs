//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/57_function_exists.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function exists($name) {\
            \n  @return function-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return function-exists(foo);\
            \n}\
            \n\
            \n@function h() {\
            \n  @return function-exists(lighten);\
            \n}\
            \n\
            \ndiv {\
            \n  foo: function-exists(lighten); \
            \n  foo: function-exists(\"lighten\"); \
            \n  foo: function-exists(exists);\
            \n  foo: function-exists(\"exists\"); \
            \n  foo: function-exists(f);\
            \n  foo: function-exists(\"f\"); \
            \n  foo: function-exists(g);\
            \n  foo: function-exists(\"g\"); \
            \n  foo: function-exists(nope);\
            \n  foo: function-exists(\"nope\"); \
            \n  foo: g();\
            \n  foo: f();\
            \n  foo: h();\
            \n\
            \n\
            \n  span {\
            \n    foo: function-exists(lighten); \
            \n    foo: function-exists(\"lighten\"); \
            \n    foo: function-exists(exists);\
            \n    foo: function-exists(\"exists\"); \
            \n    foo: function-exists(f);\
            \n    foo: function-exists(\"f\"); \
            \n    foo: function-exists(g);\
            \n    foo: function-exists(\"g\"); \
            \n    foo: function-exists(nope);\
            \n    foo: function-exists(\"nope\"); \
            \n    foo: g();\
            \n    foo: f();\
            \n    foo: h();\
            \n    p {\
            \n      foo: function-exists(lighten); \
            \n      foo: function-exists(\"lighten\"); \
            \n      foo: function-exists(exists);\
            \n      foo: function-exists(\"exists\"); \
            \n      foo: function-exists(f);\
            \n      foo: function-exists(\"f\"); \
            \n      foo: function-exists(g);\
            \n      foo: function-exists(\"g\"); \
            \n      foo: function-exists(nope);\
            \n      foo: function-exists(\"nope\"); \
            \n      foo: g();\
            \n      foo: f();\
            \n      foo: h();\
            \n    }\
            \n  }\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \ndiv span {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \ndiv span p {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \n"
    );
}
