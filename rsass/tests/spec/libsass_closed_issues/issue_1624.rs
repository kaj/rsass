//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1624.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1624")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($foo) {\
             \n  @return $foo;\
             \n}\n\
             \n@function data($foo) {\
             \n  @return \'[data-\' + $foo + \']\';\
             \n}\n\
             \n#{foo(foo)} {\
             \n  #{data(\'bar\')} {\
             \n    baz: bam;\
             \n  }\
             \n}\n"),
        "foo [data-bar] {\
         \n  baz: bam;\
         \n}\n"
    );
}
