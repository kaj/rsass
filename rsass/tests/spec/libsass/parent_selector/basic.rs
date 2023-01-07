//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo bar {\
             \n    baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n\
             \nfoo {\
             \n    bar baz & {\
             \n        bam: true;\
             \n    }\
             \n}\n"),
        "baz foo bar {\
         \n  bam: true;\
         \n}\
         \nbar baz foo {\
         \n  bam: true;\
         \n}\n"
    );
}
