//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_877.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_877")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function _test1() {\
             \n  @return \'hello\';\
             \n}\n\
             \n@function -test2() {\
             \n  @return \'hello\';\
             \n}\n\
             \n@function test() {\
             \n  @return \'world\';\
             \n}\n\
             \n@mixin _test1() {\
             \n  mixin: true;\
             \n}\n\
             \n@mixin -test2() {\
             \n  mixin: true;\
             \n}\n\
             \n@mixin test() {\
             \n  mixin: true;\
             \n}\n\
             \n$-test1: true;\
             \n$_test2: true;\
             \n$test: true;\n\
             \n.test {\
             \n  function: meta.function-exists(\'_test1\');\
             \n  function: meta.function-exists(\'-test1\');\
             \n  function: meta.function-exists(\'_test2\');\
             \n  function: meta.function-exists(\'-test2\');\
             \n  function: meta.function-exists(\'test1\');\
             \n  function: meta.function-exists(\'test2\');\
             \n  function: meta.function-exists(\'test\');\
             \n  mixin: meta.mixin-exists(\'_test1\');\
             \n  mixin: meta.mixin-exists(\'-test1\');\
             \n  mixin: meta.mixin-exists(\'_test2\');\
             \n  mixin: meta.mixin-exists(\'-test2\');\
             \n  mixin: meta.mixin-exists(\'test1\');\
             \n  mixin: meta.mixin-exists(\'test2\');\
             \n  mixin: meta.mixin-exists(\'test\');\
             \n  variable: meta.variable-exists(\'_test1\');\
             \n  variable: meta.variable-exists(\'-test1\');\
             \n  variable: meta.variable-exists(\'_test2\');\
             \n  variable: meta.variable-exists(\'-test2\');\
             \n  variable: meta.variable-exists(\'test1\');\
             \n  variable: meta.variable-exists(\'test2\');\
             \n  variable: meta.variable-exists(\'test\');\
             \n  global-variable: meta.global-variable-exists(\'_test1\');\
             \n  global-variable: meta.global-variable-exists(\'-test1\');\
             \n  global-variable: meta.global-variable-exists(\'_test2\');\
             \n  global-variable: meta.global-variable-exists(\'-test2\');\
             \n  global-variable: meta.global-variable-exists(\'test1\');\
             \n  global-variable: meta.global-variable-exists(\'test2\');\
             \n  global-variable: meta.global-variable-exists(\'test\');\
             \n}\n"),
        ".test {\
         \n  function: true;\
         \n  function: true;\
         \n  function: true;\
         \n  function: true;\
         \n  function: false;\
         \n  function: false;\
         \n  function: true;\
         \n  mixin: true;\
         \n  mixin: true;\
         \n  mixin: true;\
         \n  mixin: true;\
         \n  mixin: false;\
         \n  mixin: false;\
         \n  mixin: true;\
         \n  variable: true;\
         \n  variable: true;\
         \n  variable: true;\
         \n  variable: true;\
         \n  variable: false;\
         \n  variable: false;\
         \n  variable: true;\
         \n  global-variable: true;\
         \n  global-variable: true;\
         \n  global-variable: true;\
         \n  global-variable: true;\
         \n  global-variable: false;\
         \n  global-variable: false;\
         \n  global-variable: true;\
         \n}\n"
    );
}
