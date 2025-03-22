//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/lab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

#[test]
#[ignore] // unexepected error
fn clip() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(lab(50% 500 -999999), $lightness: 150%),\
             \n    $method: clip\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 6530020637.921538 2172031124.122868 137328815479.04425) 100%, black);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn local_minde() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(lab(50% 500 -999999), $lightness: 150%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 6530020637.921538 2172031124.122868 137328815479.04425) 100%, black);\
         \n}\n"
    );
}
