//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/58_mixin_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("58_mixin_exists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function exists($name) {\
             \n  @return mixin-exists($name);\
             \n}\n\
             \n@function f() {\
             \n  $foo: hi;\
             \n  @return g();\
             \n}\n\
             \n@function g() {\
             \n  @return mixin-exists(foo);\
             \n}\n\
             \n@function h() {\
             \n  @return mixin-exists(lighten);\
             \n}\n\
             \n@mixin red-text { color: red; }\
             \n@mixin blue-text { color: red; }\
             \n@mixin green-text { color: red; }\n\
             \ndiv {\
             \n  foo: mixin-exists(red-text); \
             \n  foo: mixin-exists(\"red-text\"); \
             \n  foo: mixin-exists(blue-text); \
             \n  foo: mixin-exists(\"blue-text\"); \
             \n  foo: mixin-exists(green-text);   \
             \n  foo: mixin-exists(\"green-text\"); \
             \n  foo: mixin-exists(nope);\
             \n  foo: mixin-exists(\"nope\");\
             \n  foo: g();\
             \n  foo: f();\
             \n  foo: h();\n\n\
             \n  span {\
             \n    foo: mixin-exists(red-text); \
             \n    foo: mixin-exists(\"red-text\"); \
             \n    foo: mixin-exists(blue-text); \
             \n    foo: mixin-exists(\"blue-text\"); \
             \n    foo: mixin-exists(green-text);   \
             \n    foo: mixin-exists(\"green-text\"); \
             \n    foo: mixin-exists(nope);\
             \n    foo: mixin-exists(\"nope\");\
             \n    foo: g();\
             \n    foo: f();\
             \n    foo: h();\
             \n    p {\
             \n      foo: mixin-exists(red-text); \
             \n      foo: mixin-exists(\"red-text\"); \
             \n      foo: mixin-exists(blue-text); \
             \n      foo: mixin-exists(\"blue-text\"); \
             \n      foo: mixin-exists(green-text);   \
             \n      foo: mixin-exists(\"green-text\"); \
             \n      foo: mixin-exists(nope);\
             \n      foo: mixin-exists(\"nope\");\
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
