//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1263.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1263")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @ap#{pl}y;\
             \n  @apply(--bar);\
             \n  @apply  (  --bar  );\
             \n  @ap#{pl}y   (   --bar , --foo  )  ;\
             \n}"),
        "foo {\
         \n  @apply;\
         \n  @apply (--bar);\
         \n  @apply ( --bar );\
         \n  @apply ( --bar , --foo );\
         \n}\n"
    );
}
