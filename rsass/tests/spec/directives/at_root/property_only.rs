//! Tests auto-converted from "sass-spec/spec/directives/at_root/property_only.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("property_only")
}

#[test]
#[ignore] // unexepected error
fn test() {
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
