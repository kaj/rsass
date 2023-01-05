//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/inner-combinator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inner-combinator")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n    & bar baz {\
             \n        bam: true;\
             \n    }\
             \n    bar baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    & bar + baz {\
             \n        bam: true;\
             \n    }\
             \n    bar + baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    & bar > baz {\
             \n        bam: true;\
             \n    }\
             \n    bar > baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    & bar ~ baz {\
             \n        bam: true;\
             \n    }\
             \n    bar ~ baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n"),
        "foo bar baz {\
         \n  bam: true;\
         \n}\
         \nbar baz foo {\
         \n  bam: true;\
         \n}\
         \nfoo bar + baz {\
         \n  bam: true;\
         \n}\
         \nbar + baz foo {\
         \n  bam: true;\
         \n}\
         \nfoo bar > baz {\
         \n  bam: true;\
         \n}\
         \nbar > baz foo {\
         \n  bam: true;\
         \n}\
         \nfoo bar ~ baz {\
         \n  bam: true;\
         \n}\
         \nbar ~ baz foo {\
         \n  bam: true;\
         \n}\n"
    );
}
