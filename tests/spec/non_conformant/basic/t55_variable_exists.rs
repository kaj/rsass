//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/55_variable_exists.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function exists($name) {\
            \n  @return variable-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return variable-exists(foo);\
            \n}\
            \n\
            \ndiv {\
            \n  foo: variable-exists(x);\
            \n  foo: variable-exists(\"x\");\
            \n\
            \n  span {\
            \n    $x: false;\
            \n\
            \n    foo: variable-exists(x);\
            \n    foo: variable-exists(\"x\");\
            \n    foo: variable-exists(y);\
            \n    foo: variable-exists(\"y\");\
            \n    foo: exists(x);\
            \n    foo: exists(\"x\");\
            \n\
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
            \n  }\
            \n\
            \n}"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
