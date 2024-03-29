//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/55_variable_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("55_variable_exists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function exists($name) {\
             \n  @return variable-exists($name);\
             \n}\n\
             \n@function f() {\
             \n  $foo: hi;\
             \n  @return g();\
             \n}\n\
             \n@function g() {\
             \n  @return variable-exists(foo);\
             \n}\n\
             \ndiv {\
             \n  foo: variable-exists(x);\
             \n  foo: variable-exists(\"x\");\n\
             \n  span {\
             \n    $x: false;\n\
             \n    foo: variable-exists(x);\
             \n    foo: variable-exists(\"x\");\
             \n    foo: variable-exists(y);\
             \n    foo: variable-exists(\"y\");\
             \n    foo: exists(x);\
             \n    foo: exists(\"x\");\n\
             \n    p {\
             \n      foo: variable-exists(x);\
             \n      foo: variable-exists(\"x\");\
             \n      foo: exists(x);\
             \n      foo: exists(\"x\");\
             \n      foo: variable-exists(y);\
             \n      foo: variable-exists(\"y\");\
             \n      foo: f();\
             \n      $y: blah;\
             \n    }\
             \n  }\n\
             \n}"),
        "div {\
         \n  foo: false;\
         \n  foo: false;\
         \n}\
         \ndiv span {\
         \n  foo: true;\
         \n  foo: true;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n  foo: false;\
         \n}\
         \ndiv span p {\
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
