//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/57_function_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("57_function_exists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function exists($name) {\
             \n  @return meta.function-exists($name);\
             \n}\n\
             \n@function f() {\
             \n  $foo: hi;\
             \n  @return g();\
             \n}\n\
             \n@function g() {\
             \n  @return meta.function-exists(foo);\
             \n}\n\
             \n@function h() {\
             \n  @return meta.function-exists(lighten);\
             \n}\n\
             \ndiv {\
             \n  foo: meta.function-exists(lighten); \
             \n  foo: meta.function-exists(\"lighten\"); \
             \n  foo: meta.function-exists(exists);\
             \n  foo: meta.function-exists(\"exists\"); \
             \n  foo: meta.function-exists(f);\
             \n  foo: meta.function-exists(\"f\"); \
             \n  foo: meta.function-exists(g);\
             \n  foo: meta.function-exists(\"g\"); \
             \n  foo: meta.function-exists(nope);\
             \n  foo: meta.function-exists(\"nope\"); \
             \n  foo: g();\
             \n  foo: f();\
             \n  foo: h();\n\n\
             \n  span {\
             \n    foo: meta.function-exists(lighten); \
             \n    foo: meta.function-exists(\"lighten\"); \
             \n    foo: meta.function-exists(exists);\
             \n    foo: meta.function-exists(\"exists\"); \
             \n    foo: meta.function-exists(f);\
             \n    foo: meta.function-exists(\"f\"); \
             \n    foo: meta.function-exists(g);\
             \n    foo: meta.function-exists(\"g\"); \
             \n    foo: meta.function-exists(nope);\
             \n    foo: meta.function-exists(\"nope\"); \
             \n    foo: g();\
             \n    foo: f();\
             \n    foo: h();\
             \n    p {\
             \n      foo: meta.function-exists(lighten); \
             \n      foo: meta.function-exists(\"lighten\"); \
             \n      foo: meta.function-exists(exists);\
             \n      foo: meta.function-exists(\"exists\"); \
             \n      foo: meta.function-exists(f);\
             \n      foo: meta.function-exists(\"f\"); \
             \n      foo: meta.function-exists(g);\
             \n      foo: meta.function-exists(\"g\"); \
             \n      foo: meta.function-exists(nope);\
             \n      foo: meta.function-exists(\"nope\"); \
             \n      foo: g();\
             \n      foo: f();\
             \n      foo: h();\
             \n    }\
             \n  }\n\
             \n}\n"),
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
         \n}\n"
    );
}
