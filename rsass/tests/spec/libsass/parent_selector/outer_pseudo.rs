//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/outer-pseudo.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("outer-pseudo")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo bar {\
             \n    &:baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo + bar {\
             \n    &:baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo > bar {\
             \n    &:baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo ~ bar {\
             \n    &:baz {\
             \n        bam: true;\
             \n    }\
             \n}\n"),
        "foo bar:baz {\
         \n  bam: true;\
         \n}\
         \nfoo + bar:baz {\
         \n  bam: true;\
         \n}\
         \nfoo > bar:baz {\
         \n  bam: true;\
         \n}\
         \nfoo ~ bar:baz {\
         \n  bam: true;\
         \n}\n"
    );
}
