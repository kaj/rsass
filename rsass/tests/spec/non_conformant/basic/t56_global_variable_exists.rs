//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/56_global_variable_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("56_global_variable_exists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function exists($name) {\
             \n  @return meta.global-variable-exists($name);\
             \n}\n\
             \n@function f() {\
             \n  $foo: hi;\
             \n  @return g();\
             \n}\n\
             \n@function g() {\
             \n  @return meta.global-variable-exists(foo);\
             \n}\n\
             \n$z: hi;\n\
             \ndiv {\
             \n  foo: meta.global-variable-exists(x);    \
             \n  foo: meta.global-variable-exists(\"x\");    \
             \n  foo: meta.global-variable-exists(z);\
             \n  foo: meta.global-variable-exists(\"z\");    \n\
             \n  span {\
             \n    $x: false;\n\
             \n    foo: meta.global-variable-exists(x);\
             \n    foo: meta.global-variable-exists(\"x\");    \
             \n    foo: meta.global-variable-exists(y);\
             \n    foo: meta.global-variable-exists(\"y\");    \n\
             \n    foo: meta.global-variable-exists(z);\
             \n    foo: meta.global-variable-exists(\"z\");    \n\
             \n    p {\
             \n      foo: meta.global-variable-exists(x);\
             \n      foo: meta.global-variable-exists(\"x\");    \
             \n      foo: exists(x);\
             \n      foo: exists(\"x\");    \
             \n      foo: meta.global-variable-exists(z);\
             \n      foo: meta.global-variable-exists(\"z\");    \
             \n      foo: meta.global-variable-exists(y);\
             \n      foo: meta.global-variable-exists(\"y\");    \
             \n      foo: f();\
             \n      $y: blah;\
             \n      //TODO: check for shadowing\
             \n    }\
             \n  }\n\
             \n}\n"),
        "div {\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: true;\
         \n  foo: true;\
         \n}\
         \ndiv span {\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: true;\
         \n  foo: true;\
         \n}\
         \ndiv span p {\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: true;\
         \n  foo: true;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n}\n"
    );
}
