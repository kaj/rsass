//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_976.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_976")
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
