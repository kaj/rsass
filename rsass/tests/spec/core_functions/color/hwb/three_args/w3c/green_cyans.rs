//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c/green_cyans.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("green_cyans")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \'../test-hue\' as *;\
             \n@include test-hue(150);\n"),
        "whiteness-0 {\
         \n  blackness-0: hsl(150, 100%, 50%);\
         \n  blackness-20: #00cc66;\
         \n  blackness-40: hsl(150, 100%, 30%);\
         \n  blackness-60: #006633;\
         \n  blackness-80: hsl(150, 100%, 10%);\
         \n  blackness-100: black;\
         \n}\
         \nwhiteness-20 {\
         \n  blackness-0: #33ff99;\
         \n  blackness-20: hsl(150, 60%, 50%);\
         \n  blackness-40: #339966;\
         \n  blackness-60: hsl(150, 33.3333333333%, 30%);\
         \n  blackness-80: #333333;\
         \n  blackness-100: hsl(0, 0%, 16.6666666667%);\
         \n}\
         \nwhiteness-40 {\
         \n  blackness-0: hsl(150, 100%, 70%);\
         \n  blackness-20: #66cc99;\
         \n  blackness-40: hsl(150, 20%, 50%);\
         \n  blackness-60: #666666;\
         \n  blackness-80: #555555;\
         \n  blackness-100: hsl(0, 0%, 28.5714285714%);\
         \n}\
         \nwhiteness-60 {\
         \n  blackness-0: #99ffcc;\
         \n  blackness-20: hsl(150, 33.3333333333%, 70%);\
         \n  blackness-40: #999999;\
         \n  blackness-60: hsl(0, 0%, 50%);\
         \n  blackness-80: hsl(0, 0%, 42.8571428571%);\
         \n  blackness-100: hsl(0, 0%, 37.5%);\
         \n}\
         \nwhiteness-80 {\
         \n  blackness-0: hsl(150, 100%, 90%);\
         \n  blackness-20: #cccccc;\
         \n  blackness-40: #aaaaaa;\
         \n  blackness-60: hsl(0, 0%, 57.1428571429%);\
         \n  blackness-80: hsl(0, 0%, 50%);\
         \n  blackness-100: hsl(0, 0%, 44.4444444444%);\
         \n}\
         \nwhiteness-100 {\
         \n  blackness-0: white;\
         \n  blackness-20: hsl(0, 0%, 83.3333333333%);\
         \n  blackness-40: hsl(0, 0%, 71.4285714286%);\
         \n  blackness-60: hsl(0, 0%, 62.5%);\
         \n  blackness-80: hsl(0, 0%, 55.5555555556%);\
         \n  blackness-100: hsl(0, 0%, 50%);\
         \n}\n"
    );
}
