//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1153.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1153")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("/* precision: 0 */\
             \n$foo: 123px;\
             \nfoo {\
             \n  bar: $foo;\
             \n}"),
        "/* precision: 0 */\
         \nfoo {\
         \n  bar: 123px;\
         \n}\n"
    );
}
