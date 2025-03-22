//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

#[test]
#[ignore] // unexepected error
fn clip() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.to-gamut(\
             \n    color.change(oklab(50% 500 -999999), $lightness: 150%),\
             \n    $method: clip\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 593644542057412224 -153762246556647904 3418717351297831936) 100%, black);\
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
             \n    color.change(oklab(50% 500 -999999), $lightness: 150%),\
             \n    $method: local-minde\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 593644542057412224 -153762246556647904 3418717351297831936) 100%, black);\
         \n}\n"
    );
}
