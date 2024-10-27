//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
#[ignore] // unexepected error
fn polar_space() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix($color1: #91e16f, $color2: #0144bf, $weight: 92%, $method: hsl decreasing hue)}\n"
        ),
        "a {\
         \n  b: rgb(177.749777646, 225.4953896552, 98.9846103448);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn rectangular_space() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix($color1: #91e16f, $color2: #0144bf, $weight: 92%, $method: lab)}\n"
        ),
        "a {\
         \n  b: rgb(141.3483384924, 211.5499489073, 120.4340844852);\
         \n}\n"
    );
}
