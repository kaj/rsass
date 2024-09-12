//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

#[test]
#[ignore] // unexepected error
fn clip() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(oklch(50% 200% 70deg), $lightness: 150%),\
             \n    $method: clip\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 5.2395754107 2.9713210909 -1.434870419) 100%, black);\
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
             \n    color.change(oklch(50% 200% 70deg), $lightness: 150%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 5.2395754107 2.9713210909 -1.434870419) 100%, black);\
         \n}\n"
    );
}
