//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_492.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_492")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (\
             \n  foo: bar,\
             \n  baz: monkey,\
             \n);\n\
             \n.css {\
             \n  @each $key, $value in $map {\
             \n    #{$key}: $value;\
             \n  }\
             \n}\n\
             \n$list: one two, three four five, six seven;\n\
             \n.list {\
             \n  @each $foo, $bar, $baz in $list {\
             \n    #{$foo}: $bar $baz;\
             \n  }\
             \n}\n"),
        ".css {\
         \n  foo: bar;\
         \n  baz: monkey;\
         \n}\
         \n.list {\
         \n  one: two;\
         \n  three: four five;\
         \n  six: seven;\
         \n}\n"
    );
}
