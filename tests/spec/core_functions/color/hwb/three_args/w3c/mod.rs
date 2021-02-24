//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/w3c"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/blue_magentas.hrx"
#[test]
fn blue_magentas() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(270);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: #8000ff;\
        \n  blackness-20: #6600cc;\
        \n  blackness-40: #4d0099;\
        \n  blackness-60: #330066;\
        \n  blackness-80: #1a0033;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #9933ff;\
        \n  blackness-20: #8033cc;\
        \n  blackness-40: rebeccapurple;\
        \n  blackness-60: #4d3366;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #b366ff;\
        \n  blackness-20: #9966cc;\
        \n  blackness-40: #806699;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #cc99ff;\
        \n  blackness-20: #b399cc;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #e6ccff;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/blues.hrx"
#[test]
fn blues() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/cyan_blues.hrx"
#[test]
fn cyan_blues() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(210);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: #0080ff;\
        \n  blackness-20: #0066cc;\
        \n  blackness-40: #004d99;\
        \n  blackness-60: #003366;\
        \n  blackness-80: #001a33;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #3399ff;\
        \n  blackness-20: #3380cc;\
        \n  blackness-40: #336699;\
        \n  blackness-60: #334d66;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #66b3ff;\
        \n  blackness-20: #6699cc;\
        \n  blackness-40: #668099;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #99ccff;\
        \n  blackness-20: #99b3cc;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #cce6ff;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/cyans.hrx"
#[test]
fn cyans() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(180);\
            \n"
        )
        .unwrap(),
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
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #66ffff;\
        \n  blackness-20: #66cccc;\
        \n  blackness-40: #669999;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #99ffff;\
        \n  blackness-20: #99cccc;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ccffff;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/green_cyans.hrx"
#[test]
fn green_cyans() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(150);\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/greens.hrx"
#[test]
fn greens() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(120);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: lime;\
        \n  blackness-20: #00cc00;\
        \n  blackness-40: #009900;\
        \n  blackness-60: #006600;\
        \n  blackness-80: #003300;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #33ff33;\
        \n  blackness-20: #33cc33;\
        \n  blackness-40: #339933;\
        \n  blackness-60: #336633;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #66ff66;\
        \n  blackness-20: #66cc66;\
        \n  blackness-40: #669966;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #99ff99;\
        \n  blackness-20: #99cc99;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ccffcc;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/magenta_reds.hrx"
#[test]
fn magenta_reds() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(330);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: #ff0080;\
        \n  blackness-20: #cc0066;\
        \n  blackness-40: #99004d;\
        \n  blackness-60: #660033;\
        \n  blackness-80: #33001a;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #ff3399;\
        \n  blackness-20: #cc3380;\
        \n  blackness-40: #993366;\
        \n  blackness-60: #66334d;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #ff66b3;\
        \n  blackness-20: #cc6699;\
        \n  blackness-40: #996680;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ff99cc;\
        \n  blackness-20: #cc99b3;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ffcce6;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/magentas.hrx"
#[test]
fn magentas() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(300);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: fuchsia;\
        \n  blackness-20: #cc00cc;\
        \n  blackness-40: #990099;\
        \n  blackness-60: #660066;\
        \n  blackness-80: #330033;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #ff33ff;\
        \n  blackness-20: #cc33cc;\
        \n  blackness-40: #993399;\
        \n  blackness-60: #663366;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #ff66ff;\
        \n  blackness-20: #cc66cc;\
        \n  blackness-40: #996699;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ff99ff;\
        \n  blackness-20: #cc99cc;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ffccff;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/oranges.hrx"
#[test]
fn oranges() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(30);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: #ff8000;\
        \n  blackness-20: #cc6600;\
        \n  blackness-40: #994d00;\
        \n  blackness-60: #663300;\
        \n  blackness-80: #331a00;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #ff9933;\
        \n  blackness-20: #cc8033;\
        \n  blackness-40: #996633;\
        \n  blackness-60: #664d33;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #ffb366;\
        \n  blackness-20: #cc9966;\
        \n  blackness-40: #998066;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ffcc99;\
        \n  blackness-20: #ccb399;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ffe6cc;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/reds.hrx"
#[test]
fn reds() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(0);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: red;\
        \n  blackness-20: #cc0000;\
        \n  blackness-40: #990000;\
        \n  blackness-60: #660000;\
        \n  blackness-80: #330000;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #ff3333;\
        \n  blackness-20: #cc3333;\
        \n  blackness-40: #993333;\
        \n  blackness-60: #663333;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #ff6666;\
        \n  blackness-20: #cc6666;\
        \n  blackness-40: #996666;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ff9999;\
        \n  blackness-20: #cc9999;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ffcccc;\
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/yellow_greens.hrx"
#[test]
fn yellow_greens() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/core_functions/color/hwb/three_args/w3c/yellows.hrx"
#[test]
fn yellows() {
    assert_eq!(
        rsass(
            "@use \'../test-hue\' as *;\
            \n@include test-hue(60);\
            \n"
        )
        .unwrap(),
        "whiteness-0 {\
        \n  blackness-0: yellow;\
        \n  blackness-20: #cccc00;\
        \n  blackness-40: #999900;\
        \n  blackness-60: #666600;\
        \n  blackness-80: #333300;\
        \n  blackness-100: black;\
        \n}\
        \nwhiteness-20 {\
        \n  blackness-0: #ffff33;\
        \n  blackness-20: #cccc33;\
        \n  blackness-40: #999933;\
        \n  blackness-60: #666633;\
        \n  blackness-80: #333333;\
        \n  blackness-100: #2b2b2b;\
        \n}\
        \nwhiteness-40 {\
        \n  blackness-0: #ffff66;\
        \n  blackness-20: #cccc66;\
        \n  blackness-40: #999966;\
        \n  blackness-60: #666666;\
        \n  blackness-80: #555555;\
        \n  blackness-100: #494949;\
        \n}\
        \nwhiteness-60 {\
        \n  blackness-0: #ffff99;\
        \n  blackness-20: #cccc99;\
        \n  blackness-40: #999999;\
        \n  blackness-60: gray;\
        \n  blackness-80: #6d6d6d;\
        \n  blackness-100: #606060;\
        \n}\
        \nwhiteness-80 {\
        \n  blackness-0: #ffffcc;\
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
