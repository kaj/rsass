//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_557.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_557")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\n\
             \na {\
             \n  foo: map.get((foo: 1, bar: 2), \"bar\");\
             \n}\n"),
        "a {\
         \n  foo: 2;\
         \n}\n"
    );
}
