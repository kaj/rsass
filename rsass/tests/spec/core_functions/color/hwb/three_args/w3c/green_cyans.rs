//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c/green_cyans.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("green_cyans")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \'../test-hue\' as *;\
             \n@include test-hue(150);\n"),
        "whiteness-0 {\
         \n  blackness-0: #00ff80;\
         \n  blackness-20: #00cc66;\
         \n  blackness-40: #00994d;\
         \n  blackness-60: #006633;\
         \n  blackness-80: #00331a;\
         \n  blackness-100: black;\
         \n}\
         \nwhiteness-20 {\
         \n  blackness-0: #33ff99;\
         \n  blackness-20: #33cc80;\
         \n  blackness-40: #339966;\
         \n  blackness-60: #33664d;\
         \n  blackness-80: #333333;\
         \n  blackness-100: #2b2b2b;\
         \n}\
         \nwhiteness-40 {\
         \n  blackness-0: #66ffb3;\
         \n  blackness-20: #66cc99;\
         \n  blackness-40: #669980;\
         \n  blackness-60: #666666;\
         \n  blackness-80: #555555;\
         \n  blackness-100: #494949;\
         \n}\
         \nwhiteness-60 {\
         \n  blackness-0: #99ffcc;\
         \n  blackness-20: #99ccb3;\
         \n  blackness-40: #999999;\
         \n  blackness-60: gray;\
         \n  blackness-80: #6d6d6d;\
         \n  blackness-100: #606060;\
         \n}\
         \nwhiteness-80 {\
         \n  blackness-0: #ccffe6;\
         \n  blackness-20: #cccccc;\
         \n  blackness-40: #aaaaaa;\
         \n  blackness-60: #929292;\
         \n  blackness-80: gray;\
         \n  blackness-100: #717171;\
         \n}\
         \nwhiteness-100 {\
         \n  blackness-0: white;\
         \n  blackness-20: #d5d5d5;\
         \n  blackness-40: #b6b6b6;\
         \n  blackness-60: #9f9f9f;\
         \n  blackness-80: #8e8e8e;\
         \n  blackness-100: gray;\
         \n}\n"
    );
}
