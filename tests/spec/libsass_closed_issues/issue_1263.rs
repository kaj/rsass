//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1263.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @ap#{pl}y;\
            \n  @apply(--bar);\
            \n  @apply  (  --bar  );\
            \n  @ap#{pl}y   (   --bar , --foo  )  ;\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  @apply;\
        \n  @apply (--bar);\
        \n  @apply (  --bar  );\
        \n  @apply (   --bar , --foo  );\
        \n}\
        \n"
    );
}
