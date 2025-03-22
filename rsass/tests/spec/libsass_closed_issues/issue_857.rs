//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_857.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_857")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n$list: \"item-1\" \"item-2\" \"item-3\";\n\
             \n#hello {\
             \n  @if list.length($list) % 2 == 0 {\
             \n    color: blue;\
             \n  }\n\
             \n  @else {\
             \n    color: red;\
             \n  }\
             \n}"),
        "#hello {\
         \n  color: red;\
         \n}\n"
    );
}
