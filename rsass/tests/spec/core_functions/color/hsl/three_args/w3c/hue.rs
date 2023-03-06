//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c/hue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hue")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin test-hues($name, $hues...) {\
             \n  #{$name} {\
             \n    @each $hue in $hues {\
             \n      hue-#{$hue}: hsl($hue, 100%, 50%);\
             \n    }\
             \n  }\
             \n}\n\
             \n@include test-hues(\"red\", 0, -360, 360, 6120);\
             \n@include test-hues(\"yellow\", 60, -300, 420, -9660);\
             \n@include test-hues(\"green\", 120, -240, 480, 99840);\
             \n@include test-hues(\"cyan\", 180, -180, 540, -900);\
             \n@include test-hues(\"blue\", 240, -120, 600, -104880);\
             \n@include test-hues(\"purple\", 300, -60, 660, 2820);\n"),
        "red {\
         \n  hue-0: hsl(0, 100%, 50%);\
         \n  hue--360: hsl(0, 100%, 50%);\
         \n  hue-360: hsl(0, 100%, 50%);\
         \n  hue-6120: hsl(0, 100%, 50%);\
         \n}\
         \nyellow {\
         \n  hue-60: hsl(60, 100%, 50%);\
         \n  hue--300: hsl(60, 100%, 50%);\
         \n  hue-420: hsl(60, 100%, 50%);\
         \n  hue--9660: hsl(60, 100%, 50%);\
         \n}\
         \ngreen {\
         \n  hue-120: hsl(120, 100%, 50%);\
         \n  hue--240: hsl(120, 100%, 50%);\
         \n  hue-480: hsl(120, 100%, 50%);\
         \n  hue-99840: hsl(120, 100%, 50%);\
         \n}\
         \ncyan {\
         \n  hue-180: hsl(180, 100%, 50%);\
         \n  hue--180: hsl(180, 100%, 50%);\
         \n  hue-540: hsl(180, 100%, 50%);\
         \n  hue--900: hsl(180, 100%, 50%);\
         \n}\
         \nblue {\
         \n  hue-240: hsl(240, 100%, 50%);\
         \n  hue--120: hsl(240, 100%, 50%);\
         \n  hue-600: hsl(240, 100%, 50%);\
         \n  hue--104880: hsl(240, 100%, 50%);\
         \n}\
         \npurple {\
         \n  hue-300: hsl(300, 100%, 50%);\
         \n  hue--60: hsl(300, 100%, 50%);\
         \n  hue-660: hsl(300, 100%, 50%);\
         \n  hue-2820: hsl(300, 100%, 50%);\
         \n}\n"
    );
}
