//! Tests auto-converted from "sass-spec/spec/css/media/logic/and.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and")
}

#[test]
fn comment_after() {
    assert_eq!(
        runner().ok("@media (a) and/**/(b) {x {y: z}}\n"),
        "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn interpolation() {
    assert_eq!(
        runner().ok("@media (a) and #{\"(b) and (c)\"} {x {y: z}}\n"),
        "@media (a) and (b) and (c) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn mixed_case() {
    assert_eq!(
        runner().ok("@media (a) AnD (b) {x {y: z}}\n"),
        "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("@media (a) and (b) and (c) and (d) {x {y: z}}\n"),
        "@media (a) and (b) and (c) and (d) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn no_whitespace_before() {
    assert_eq!(
        runner().ok("@media (a)and (b) {x {y: z}}\n"),
        "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn one() {
    assert_eq!(
        runner().ok("@media (a) and (b) {x {y: z}}\n"),
        "@media (a) and (b) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
