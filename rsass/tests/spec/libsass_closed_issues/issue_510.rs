//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_510.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_510")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n$before: map.remove((foo: 1, bar: 2, baz: 3, burp: 4), bar, baz);\
             \n$after: (foo: 1, burp: 4);\n\
             \ndiv {\
             \n  foo: $before == $after;\
             \n}"
        ),
        "div {\
         \n  foo: true;\
         \n}\n"
    );
}
