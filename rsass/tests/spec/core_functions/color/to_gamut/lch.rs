//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/lch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // unexepected error
fn clip() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(lch(50% 200% 70deg), $lightness: 150%),\
             \n    $method: clip\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 3.9677504831 2.839747694 -0.024493753) 100%, black);\
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
             \n    color.change(lch(50% 200% 70deg), $lightness: 150%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 3.9677504831 2.839747694 -0.024493753) 100%, black);\
         \n}\n"
    );
}
