//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_877.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function _test1() {\
            \n  @return \'hello\';\
            \n}\
            \n\
            \n@function -test2() {\
            \n  @return \'hello\';\
            \n}\
            \n\
            \n@function test() {\
            \n  @return \'world\';\
            \n}\
            \n\
            \n@mixin _test1() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n@mixin -test2() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n@mixin test() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n$-test1: true;\
            \n$_test2: true;\
            \n$test: true;\
            \n\
            \n.test {\
            \n  function: function-exists(\'_test1\');\
            \n  function: function-exists(\'-test1\');\
            \n  function: function-exists(\'_test2\');\
            \n  function: function-exists(\'-test2\');\
            \n  function: function-exists(\'test1\');\
            \n  function: function-exists(\'test2\');\
            \n  function: function-exists(\'test\');\
            \n  mixin: mixin-exists(\'_test1\');\
            \n  mixin: mixin-exists(\'-test1\');\
            \n  mixin: mixin-exists(\'_test2\');\
            \n  mixin: mixin-exists(\'-test2\');\
            \n  mixin: mixin-exists(\'test1\');\
            \n  mixin: mixin-exists(\'test2\');\
            \n  mixin: mixin-exists(\'test\');\
            \n  variable: variable-exists(\'_test1\');\
            \n  variable: variable-exists(\'-test1\');\
            \n  variable: variable-exists(\'_test2\');\
            \n  variable: variable-exists(\'-test2\');\
            \n  variable: variable-exists(\'test1\');\
            \n  variable: variable-exists(\'test2\');\
            \n  variable: variable-exists(\'test\');\
            \n  global-variable: global-variable-exists(\'_test1\');\
            \n  global-variable: global-variable-exists(\'-test1\');\
            \n  global-variable: global-variable-exists(\'_test2\');\
            \n  global-variable: global-variable-exists(\'-test2\');\
            \n  global-variable: global-variable-exists(\'test1\');\
            \n  global-variable: global-variable-exists(\'test2\');\
            \n  global-variable: global-variable-exists(\'test\');\
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
