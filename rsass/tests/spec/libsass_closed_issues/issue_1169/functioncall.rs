//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/functioncall.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("functioncall")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$count: 0;\n\
             \n@function counter() {\
             \n  $count: $count + 1 !global;\
             \n  @return $count;\
             \n}\n\
             \n$map: (\
             \n  counter(): \'bar\',\
             \n  counter(): \'foo\',\
             \n);\n\
             \n.foo {\
             \n  content: inspect($map);\
             \n}"),
        ".foo {\
         \n  content: (1: \"bar\", 2: \"foo\");\
         \n}\n"
    );
}
