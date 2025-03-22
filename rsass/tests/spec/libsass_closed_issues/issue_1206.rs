//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1206.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1206")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: #{0/0};\
             \n  bar: #{0/1};\
             \n  bar: #{1/2};\
             \n}\n"),
        "foo {\
         \n  bar: 0/0;\
         \n  bar: 0/1;\
         \n  bar: 1/2;\
         \n}\n"
    );
}
