//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/explicit_method.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("explicit_method")
}

#[test]
#[ignore] // unexepected error
fn non_srgb() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(lab(54.3 80.8 69.9), lch(46.3 68 134), $method: oklch)}\n"
        ),
        "a {\
         \n  b: lab(50.3820988862% 23.7394813109 159.7498356926);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn previously_invalid() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n// An earlier draft of CSS Colors 4 didn\'t allow some spaces because they were\
             \n// redundant with others, but they should be allowed now.\
             \na {b: color.mix(lab(54.3 80.8 69.9), lch(46.3 68 134), $method: display-p3)}\n"
        ),
        "a {\
         \n  b: lab(43.9078099681% 26.0060363023 48.9943374049);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn srgb() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.mix(red, green, $method: xyz)}\n"),
        "a {\
         \n  b: rgb(187.5160306784, 92.3735312967, 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn weighted() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(lab(54.3 80.8 69.9), lch(46.3 68 134), 30%, oklch)}\n"
        ),
        "a {\
         \n  b: lab(48.9468096831% -7.9262402391 131.5295999957);\
         \n}\n"
    );
}
