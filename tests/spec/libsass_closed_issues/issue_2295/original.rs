//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2295/original.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("original")
        .mock_file("_input-bug.scss", "$include-foo: true !default;\r\n.bar { display: none; }\r\n@if ($include-foo) {\r\n  .foo { display: none; }\r\n}")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$include-foo: true !default;\r\
             \n.my-scope {\r\
             \n  .bar {  display: none; }\r\
             \n  @if ($include-foo) {\r\
             \n    .foo { display: none; }\r\
             \n  }\r\
             \n  @import \'input-bug\';\r\
             \n}"),
        ".my-scope .bar {\
         \n  display: none;\
         \n}\
         \n.my-scope .foo {\
         \n  display: none;\
         \n}\
         \n.my-scope .bar {\
         \n  display: none;\
         \n}\
         \n.my-scope .foo {\
         \n  display: none;\
         \n}\n"
    );
}
