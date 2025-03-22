//! Tests auto-converted from "sass-spec/spec/css/media/logic/or.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("or")
}

#[test]
fn comment_after() {
    assert_eq!(
        runner().ok("@media (a) or/**/(b) {x {y: z}}\n"),
        "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn interpolation() {
    assert_eq!(
        runner().ok("@media (a) or #{\"(b) or (c)\"} {x {y: z}}\n"),
        "@media (a) or (b) or (c) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn mixed_case() {
    assert_eq!(
        runner().ok("@media (a) oR (b) {x {y: z}}\n"),
        "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("@media (a) or (b) or (c) or (d) {x {y: z}}\n"),
        "@media (a) or (b) or (c) or (d) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn no_whitespace_before() {
    assert_eq!(
        runner().ok("@media (a)or (b) {x {y: z}}\n"),
        "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn one() {
    assert_eq!(
        runner().ok("@media (a) or (b) {x {y: z}}\n"),
        "@media (a) or (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
