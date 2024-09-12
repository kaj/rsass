//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1819.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1819")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \nfoo {\
             \n  bar: meta.type-of(selector.unify(\'p\', \'a\'));\
             \n}"),
        "foo {\
         \n  bar: null;\
         \n}\n"
    );
}
