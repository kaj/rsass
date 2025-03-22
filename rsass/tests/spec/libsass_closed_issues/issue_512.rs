//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_512.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_512")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$list: a b c;\
             \n.css {\
             \n  debug: list.index($list, a);\n\
             \n  @if meta.type-of(list.index($list, 2)) == \"null\" {\
             \n    debug: foo;\
             \n  }\
             \n}\n"),
        ".css {\
         \n  debug: 1;\
         \n  debug: foo;\
         \n}\n"
    );
}
