//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c/cyans.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("cyans")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \'../test-hue\' as *;\
             \n@include test-hue(180);\n"),
        "whiteness-0 {\
         \n  blackness-0: aqua;\
         \n  blackness-20: #00cccc;\
         \n  blackness-40: #009999;\
         \n  blackness-60: #006666;\
         \n  blackness-80: #003333;\
         \n  blackness-100: black;\
         \n}\
         \nwhiteness-20 {\
         \n  blackness-0: #33ffff;\
         \n  blackness-20: #33cccc;\
         \n  blackness-40: #339999;\
         \n  blackness-60: #336666;\
         \n  blackness-80: #333333;\
         \n  blackness-100: hsl(0, 0%, 16.6666666667%);\
         \n}\
         \nwhiteness-40 {\
         \n  blackness-0: #66ffff;\
         \n  blackness-20: #66cccc;\
         \n  blackness-40: #669999;\
         \n  blackness-60: #666666;\
         \n  blackness-80: #555555;\
         \n  blackness-100: hsl(0, 0%, 28.5714285714%);\
         \n}\
         \nwhiteness-60 {\
         \n  blackness-0: #99ffff;\
         \n  blackness-20: #99cccc;\
         \n  blackness-40: #999999;\
         \n  blackness-60: hsl(0, 0%, 50%);\
         \n  blackness-80: hsl(0, 0%, 42.8571428571%);\
         \n  blackness-100: hsl(0, 0%, 37.5%);\
         \n}\
         \nwhiteness-80 {\
         \n  blackness-0: #ccffff;\
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
