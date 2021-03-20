//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c/blues.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(240);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: blue;\
        \n  blackness-20: #0000cc;\
        \n  blackness-40: #000099;\
        \n  blackness-60: #000066;\
        \n  blackness-80: #000033;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #3333ff;\
        \n  blackness-20: #3333cc;\
        \n  blackness-40: #333399;\
        \n  blackness-60: #333366;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #6666ff;\
        \n  blackness-20: #6666cc;\
        \n  blackness-40: #666699;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #9999ff;\
        \n  blackness-20: #9999cc;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ccccff;\
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
