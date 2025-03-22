//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/predefined.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("predefined")
}

#[test]
#[ignore] // unexepected error
fn rgb() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(color(display-p3 1 0 0), color(display-p3 0 1 0), 20%, oklch)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.6684751748 0.8535262895 -0.4281892011);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rgb_explicit_method() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(color(display-p3 1 0 0), color(display-p3 0 1 0), 60%, $method: hsl)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1.0849635599 0.8595336595 -0.252822726);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(color(xyz 0.15 0.24 0), color(xyz 1 .2 0), 65%, lch)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.2607187352 0.230899759 -0.0359437687);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn xyz_explicit_method() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(color(xyz-d50 0.15 0.24 0), color(xyz-d65 1 .2 0), $method: hwb)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.5250037958 0.2975068714 -0.1396614468);\
         \n}\n"
    );
}
