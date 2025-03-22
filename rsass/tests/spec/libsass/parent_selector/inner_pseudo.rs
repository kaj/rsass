//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/inner-pseudo.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inner-pseudo")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n    &:bar baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    &:bar + baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    &:bar > baz {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    &:bar ~ baz {\
             \n        bam: true;\
             \n    }\
             \n}\n"),
        "foo:bar baz {\
         \n  bam: true;\
         \n}\
         \nfoo:bar + baz {\
         \n  bam: true;\
         \n}\
         \nfoo:bar > baz {\
         \n  bam: true;\
         \n}\
         \nfoo:bar ~ baz {\
         \n  bam: true;\
         \n}\n"
    );
}
