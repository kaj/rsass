//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/xyz_d50.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

#[test]
#[ignore] // unexepected error
fn clip() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(xyz-d50 123 -456 999999), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 123 -456 999999);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn local_minde() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(xyz-d50 123 -456 999999), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 123 -456 999999);\
         \n}\n"
    );
}
