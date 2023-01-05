//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_512.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_512")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$list: a b c;\
             \n.css {\
             \n  debug: index($list, a);\n\
             \n  @if type-of(index($list, 2)) == \"null\" {\
             \n    debug: foo;\
             \n  }\
             \n}\n"),
        ".css {\
         \n  debug: 1;\
         \n  debug: foo;\
         \n}\n"
    );
}
