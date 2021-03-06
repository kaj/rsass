//! Tests auto-converted from "sass-spec/spec/directives/at_root.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod keyframes {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@keyframes a {\
             \n  @at-root (without: all) {\
             \n    b {c: d}\
             \n  }\
             \n}\n"),
            "@keyframes a {}\
         \nb {\
         \n  c: d;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn property_only() {
    assert_eq!(
        runner().ok("@media print {\
             \n  a {\
             \n    @at-root (without: media) {\
             \n      b: c;\
             \n    }\
             \n  }\
             \n}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
