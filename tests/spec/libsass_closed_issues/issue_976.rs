//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_976.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".debug {\
             \n  @debug-this {\
             \n    foo: bar;\
             \n  }\
             \n}"),
        "@debug-this {\
         \n  .debug {\
         \n    foo: bar;\
         \n  }\
         \n}\n"
    );
}
