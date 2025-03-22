//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/58_mixin_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("58_mixin_exists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function exists($name) {\
             \n  @return meta.mixin-exists($name);\
             \n}\n\
             \n@function f() {\
             \n  $foo: hi;\
             \n  @return g();\
             \n}\n\
             \n@function g() {\
             \n  @return meta.mixin-exists(foo);\
             \n}\n\
             \n@function h() {\
             \n  @return meta.mixin-exists(lighten);\
             \n}\n\
             \n@mixin red-text { color: red; }\
             \n@mixin blue-text { color: red; }\
             \n@mixin green-text { color: red; }\n\
             \ndiv {\
             \n  foo: meta.mixin-exists(red-text); \
             \n  foo: meta.mixin-exists(\"red-text\"); \
             \n  foo: meta.mixin-exists(blue-text); \
             \n  foo: meta.mixin-exists(\"blue-text\"); \
             \n  foo: meta.mixin-exists(green-text);   \
             \n  foo: meta.mixin-exists(\"green-text\"); \
             \n  foo: meta.mixin-exists(nope);\
             \n  foo: meta.mixin-exists(\"nope\");\
             \n  foo: g();\
             \n  foo: f();\
             \n  foo: h();\n\n\
             \n  span {\
             \n    foo: meta.mixin-exists(red-text); \
             \n    foo: meta.mixin-exists(\"red-text\"); \
             \n    foo: meta.mixin-exists(blue-text); \
             \n    foo: meta.mixin-exists(\"blue-text\"); \
             \n    foo: meta.mixin-exists(green-text);   \
             \n    foo: meta.mixin-exists(\"green-text\"); \
             \n    foo: meta.mixin-exists(nope);\
             \n    foo: meta.mixin-exists(\"nope\");\
             \n    foo: g();\
             \n    foo: f();\
             \n    foo: h();\
             \n    p {\
             \n      foo: meta.mixin-exists(red-text); \
             \n      foo: meta.mixin-exists(\"red-text\"); \
             \n      foo: meta.mixin-exists(blue-text); \
             \n      foo: meta.mixin-exists(\"blue-text\"); \
             \n      foo: meta.mixin-exists(green-text);   \
             \n      foo: meta.mixin-exists(\"green-text\"); \
             \n      foo: meta.mixin-exists(nope);\
             \n      foo: meta.mixin-exists(\"nope\");\
             \n      foo: g();\
             \n      foo: f();\
             \n      foo: h();\
             \n    }\
             \n  }\n\
             \n}"),
        "div {\
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
         \n  foo: false;\
         \n}\
         \ndiv span {\
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
         \n  foo: false;\
         \n}\
         \ndiv span p {\
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
         \n  foo: false;\
         \n}\n"
    );
}
