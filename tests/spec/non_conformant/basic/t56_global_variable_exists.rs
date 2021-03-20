//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/56_global_variable_exists.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function exists($name) {\
            \n  @return global-variable-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return global-variable-exists(foo);\
            \n}\
            \n\
            \n$z: hi;\
            \n\
            \ndiv {\
            \n  foo: global-variable-exists(x);    \
            \n  foo: global-variable-exists(\"x\");    \
            \n  foo: global-variable-exists(z);\
            \n  foo: global-variable-exists(\"z\");    \
            \n\
            \n  span {\
            \n    $x: false;\
            \n\
            \n    foo: global-variable-exists(x);\
            \n    foo: global-variable-exists(\"x\");    \
            \n    foo: global-variable-exists(y);\
            \n    foo: global-variable-exists(\"y\");    \
            \n\
            \n    foo: global-variable-exists(z);\
            \n    foo: global-variable-exists(\"z\");    \
            \n\
            \n    p {\
            \n      foo: global-variable-exists(x);\
            \n      foo: global-variable-exists(\"x\");    \
            \n      foo: exists(x);\
            \n      foo: exists(\"x\");    \
            \n      foo: global-variable-exists(z);\
            \n      foo: global-variable-exists(\"z\");    \
            \n      foo: global-variable-exists(y);\
            \n      foo: global-variable-exists(\"y\");    \
            \n      foo: f();\
            \n      $y: blah;\
            \n      //TODO: check for shadowing\
            \n    }\
            \n  }\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
