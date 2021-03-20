//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c/yellow_greens.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(90);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: #80ff00;\
        \n  blackness-20: #66cc00;\
        \n  blackness-40: #4d9900;\
        \n  blackness-60: #336600;\
        \n  blackness-80: #1a3300;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #99ff33;\
        \n  blackness-20: #80cc33;\
        \n  blackness-40: #669933;\
        \n  blackness-60: #4d6633;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #b3ff66;\
        \n  blackness-20: #99cc66;\
        \n  blackness-40: #809966;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ccff99;\
        \n  blackness-20: #b3cc99;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #e6ffcc;\
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
        \n}\
        \n"
    );
}
