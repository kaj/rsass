//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1263.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
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
         \n  @apply (  --bar  );\
         \n  @apply (   --bar , --foo  );\
         \n}\n"
    );
}
