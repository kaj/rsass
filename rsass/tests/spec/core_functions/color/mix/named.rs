//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/named.hrx"

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
         \n  b: rgb(69.7057951553%, 88.4295645707%, 38.8174942529%);\
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
         \n  b: rgb(55.4307209774%, 82.9607642774%, 47.2290527393%);\
         \n}\n"
    );
}
