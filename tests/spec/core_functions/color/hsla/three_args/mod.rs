//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/hsla/three_args/clamped.hrx"
mod clamped {
    #[allow(unused)]
    use super::rsass;
    mod lightness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, 100%, 500%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, 100%, -100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, 500%, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, -100%, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: gray;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/hsla/three_args/named.hrx"
#[test]
fn named() {
    assert_eq!(
        rsass(
            "a {b: hsla($hue: 0, $saturation: 100%, $lightness: 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/color/hsla/three_args/special_functions.hrx"
#[test]
fn special_functions() {
    assert_eq!(
        rsass(
            "a {\
            \n  calc-1: hsla(calc(1), 2%, 3%);\
            \n  calc-2: hsla(1, calc(2%), 3%);\
            \n  calc-3: hsla(1, 2%, calc(3%));\
            \n\
            \n  var-1: hsla(var(--foo), 2%, 3%);\
            \n  var-2: hsla(1, var(--foo), 3%);\
            \n  var-3: hsla(1, 2%, var(--foo));\
            \n\
            \n  env-1: hsla(env(--foo), 2%, 3%);\
            \n  env-2: hsla(1, env(--foo), 3%);\
            \n  env-3: hsla(1, 2%, env(--foo));\
            \n\
            \n  min-1: hsla(min(1), 2%, 3%);\
            \n  min-2: hsla(1, min(2%), 3%);\
            \n  min-3: hsla(1, 2%, min(3%));\
            \n\
            \n  max-1: hsla(max(1), 2%, 3%);\
            \n  max-2: hsla(1, max(2%), 3%);\
            \n  max-3: hsla(1, 2%, max(3%));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  calc-1: hsla(calc(1), 2%, 3%);\
        \n  calc-2: hsla(1, calc(2%), 3%);\
        \n  calc-3: hsla(1, 2%, calc(3%));\
        \n  var-1: hsla(var(--foo), 2%, 3%);\
        \n  var-2: hsla(1, var(--foo), 3%);\
        \n  var-3: hsla(1, 2%, var(--foo));\
        \n  env-1: hsla(env(--foo), 2%, 3%);\
        \n  env-2: hsla(1, env(--foo), 3%);\
        \n  env-3: hsla(1, 2%, env(--foo));\
        \n  min-1: hsla(min(1), 2%, 3%);\
        \n  min-2: hsla(1, min(2%), 3%);\
        \n  min-3: hsla(1, 2%, min(3%));\
        \n  max-1: hsla(max(1), 2%, 3%);\
        \n  max-2: hsla(1, max(2%), 3%);\
        \n  max-3: hsla(1, 2%, max(3%));\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/color/hsla/three_args/units.hrx"
mod units {
    #[allow(unused)]
    use super::rsass;
    mod hue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn deg() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0deg, 100%, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, 100%, 50)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: hsla(0, 50, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/hsla/three_args/w3c.hrx"
mod w3c {
    #[allow(unused)]
    use super::rsass;
    mod black_to_white_through {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn blue() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(240, 100%, 0%);\
            \n  step-2: hsla(240, 100%, 10%);\
            \n  step-3: hsla(240, 100%, 20%);\
            \n  step-4: hsla(240, 100%, 30%);\
            \n  step-5: hsla(240, 100%, 40%);\
            \n  step-6: hsla(240, 100%, 50%);\
            \n  step-7: hsla(240, 100%, 60%);\
            \n  step-8: hsla(240, 100%, 70%);\
            \n  step-9: hsla(240, 100%, 80%);\
            \n  step-10: hsla(240, 100%, 90%);\
            \n  step-11: hsla(240, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #000033;\
        \n  step-3: #000066;\
        \n  step-4: #000099;\
        \n  step-5: #0000cc;\
        \n  step-6: blue;\
        \n  step-7: #3333ff;\
        \n  step-8: #6666ff;\
        \n  step-9: #9999ff;\
        \n  step-10: #ccccff;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn cyan() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(180, 100%, 0%);\
            \n  step-2: hsla(180, 100%, 10%);\
            \n  step-3: hsla(180, 100%, 20%);\
            \n  step-4: hsla(180, 100%, 30%);\
            \n  step-5: hsla(180, 100%, 40%);\
            \n  step-6: hsla(180, 100%, 50%);\
            \n  step-7: hsla(180, 100%, 60%);\
            \n  step-8: hsla(180, 100%, 70%);\
            \n  step-9: hsla(180, 100%, 80%);\
            \n  step-10: hsla(180, 100%, 90%);\
            \n  step-11: hsla(180, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #003333;\
        \n  step-3: #006666;\
        \n  step-4: #009999;\
        \n  step-5: #00cccc;\
        \n  step-6: aqua;\
        \n  step-7: #33ffff;\
        \n  step-8: #66ffff;\
        \n  step-9: #99ffff;\
        \n  step-10: #ccffff;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(120, 100%, 0%);\
            \n  step-2: hsla(120, 100%, 10%);\
            \n  step-3: hsla(120, 100%, 20%);\
            \n  step-4: hsla(120, 100%, 30%);\
            \n  step-5: hsla(120, 100%, 40%);\
            \n  step-6: hsla(120, 100%, 50%);\
            \n  step-7: hsla(120, 100%, 60%);\
            \n  step-8: hsla(120, 100%, 70%);\
            \n  step-9: hsla(120, 100%, 80%);\
            \n  step-10: hsla(120, 100%, 90%);\
            \n  step-11: hsla(120, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #003300;\
        \n  step-3: #006600;\
        \n  step-4: #009900;\
        \n  step-5: #00cc00;\
        \n  step-6: lime;\
        \n  step-7: #33ff33;\
        \n  step-8: #66ff66;\
        \n  step-9: #99ff99;\
        \n  step-10: #ccffcc;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn purple() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(300, 100%, 0%);\
            \n  step-2: hsla(300, 100%, 10%);\
            \n  step-3: hsla(300, 100%, 20%);\
            \n  step-4: hsla(300, 100%, 30%);\
            \n  step-5: hsla(300, 100%, 40%);\
            \n  step-6: hsla(300, 100%, 50%);\
            \n  step-7: hsla(300, 100%, 60%);\
            \n  step-8: hsla(300, 100%, 70%);\
            \n  step-9: hsla(300, 100%, 80%);\
            \n  step-10: hsla(300, 100%, 90%);\
            \n  step-11: hsla(300, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #330033;\
        \n  step-3: #660066;\
        \n  step-4: #990099;\
        \n  step-5: #cc00cc;\
        \n  step-6: fuchsia;\
        \n  step-7: #ff33ff;\
        \n  step-8: #ff66ff;\
        \n  step-9: #ff99ff;\
        \n  step-10: #ffccff;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(0, 100%, 0%);\
            \n  step-2: hsla(0, 100%, 10%);\
            \n  step-3: hsla(0, 100%, 20%);\
            \n  step-4: hsla(0, 100%, 30%);\
            \n  step-5: hsla(0, 100%, 40%);\
            \n  step-6: hsla(0, 100%, 50%);\
            \n  step-7: hsla(0, 100%, 60%);\
            \n  step-8: hsla(0, 100%, 70%);\
            \n  step-9: hsla(0, 100%, 80%);\
            \n  step-10: hsla(0, 100%, 90%);\
            \n  step-11: hsla(0, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #330000;\
        \n  step-3: #660000;\
        \n  step-4: #990000;\
        \n  step-5: #cc0000;\
        \n  step-6: red;\
        \n  step-7: #ff3333;\
        \n  step-8: #ff6666;\
        \n  step-9: #ff9999;\
        \n  step-10: #ffcccc;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn yellow() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(60, 100%, 0%);\
            \n  step-2: hsla(60, 100%, 10%);\
            \n  step-3: hsla(60, 100%, 20%);\
            \n  step-4: hsla(60, 100%, 30%);\
            \n  step-5: hsla(60, 100%, 40%);\
            \n  step-6: hsla(60, 100%, 50%);\
            \n  step-7: hsla(60, 100%, 60%);\
            \n  step-8: hsla(60, 100%, 70%);\
            \n  step-9: hsla(60, 100%, 80%);\
            \n  step-10: hsla(60, 100%, 90%);\
            \n  step-11: hsla(60, 100%, 100%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: black;\
        \n  step-2: #333300;\
        \n  step-3: #666600;\
        \n  step-4: #999900;\
        \n  step-5: #cccc00;\
        \n  step-6: yellow;\
        \n  step-7: #ffff33;\
        \n  step-8: #ffff66;\
        \n  step-9: #ffff99;\
        \n  step-10: #ffffcc;\
        \n  step-11: white;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn blue_to_red() {
        assert_eq!(
            rsass(
                "a {\
            \n  step-1: hsla(240, 100%, 50%);\
            \n  step-2: hsla(252, 100%, 50%);\
            \n  step-3: hsla(264, 100%, 50%);\
            \n  step-4: hsla(276, 100%, 50%);\
            \n  step-5: hsla(288, 100%, 50%);\
            \n  step-6: hsla(300, 100%, 50%);\
            \n  step-7: hsla(312, 100%, 50%);\
            \n  step-8: hsla(324, 100%, 50%);\
            \n  step-9: hsla(336, 100%, 50%);\
            \n  step-10: hsla(348, 100%, 50%);\
            \n  step-11: hsla(360, 100%, 50%);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  step-1: blue;\
        \n  step-2: #3300ff;\
        \n  step-3: #6600ff;\
        \n  step-4: #9900ff;\
        \n  step-5: #cc00ff;\
        \n  step-6: fuchsia;\
        \n  step-7: #ff00cc;\
        \n  step-8: #ff0099;\
        \n  step-9: #ff0066;\
        \n  step-10: #ff0033;\
        \n  step-11: red;\
        \n}\
        \n"
        );
    }
    mod gray_to {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn blue() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(240, 20%, 50%);\
            \n  step-2: hsla(240, 60%, 50%);\
            \n  step-3: hsla(240, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #666699;\
        \n  step-2: #3333cc;\
        \n  step-3: blue;\
        \n}\
        \n"
            );
        }
        #[test]
        fn cyan() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(180, 20%, 50%);\
            \n  step-2: hsla(180, 60%, 50%);\
            \n  step-3: hsla(180, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #669999;\
        \n  step-2: #33cccc;\
        \n  step-3: aqua;\
        \n}\
        \n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(120, 20%, 50%);\
            \n  step-2: hsla(120, 60%, 50%);\
            \n  step-3: hsla(120, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #669966;\
        \n  step-2: #33cc33;\
        \n  step-3: lime;\
        \n}\
        \n"
            );
        }
        #[test]
        fn purple() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(300, 20%, 50%);\
            \n  step-2: hsla(300, 60%, 50%);\
            \n  step-3: hsla(300, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #996699;\
        \n  step-2: #cc33cc;\
        \n  step-3: fuchsia;\
        \n}\
        \n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(0, 20%, 50%);\
            \n  step-2: hsla(0, 60%, 50%);\
            \n  step-3: hsla(0, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #996666;\
        \n  step-2: #cc3333;\
        \n  step-3: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn yellow() {
            assert_eq!(
                rsass(
                    "a {\
            \n  step-1: hsla(60, 20%, 50%);\
            \n  step-2: hsla(60, 60%, 50%);\
            \n  step-3: hsla(60, 100%, 50%);\
            \n}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  step-1: #999966;\
        \n  step-2: #cccc33;\
        \n  step-3: yellow;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn green_to_blue() {
        assert_eq!(
            rsass(
                "a {\
            \n  step-1: hsla(120, 100%, 50%);\
            \n  step-2: hsla(132, 100%, 50%);\
            \n  step-3: hsla(144, 100%, 50%);\
            \n  step-4: hsla(156, 100%, 50%);\
            \n  step-5: hsla(168, 100%, 50%);\
            \n  step-6: hsla(180, 100%, 50%);\
            \n  step-7: hsla(192, 100%, 50%);\
            \n  step-8: hsla(204, 100%, 50%);\
            \n  step-9: hsla(216, 100%, 50%);\
            \n  step-10: hsla(228, 100%, 50%);\
            \n  step-11: hsla(240, 100%, 50%);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  step-1: lime;\
        \n  step-2: #00ff33;\
        \n  step-3: #00ff66;\
        \n  step-4: #00ff99;\
        \n  step-5: #00ffcc;\
        \n  step-6: aqua;\
        \n  step-7: #00ccff;\
        \n  step-8: #0099ff;\
        \n  step-9: #0066ff;\
        \n  step-10: #0033ff;\
        \n  step-11: blue;\
        \n}\
        \n"
        );
    }
    #[test]
    fn hue() {
        assert_eq!(
            rsass(
                "@mixin test-hues($name, $hues...) {\
            \n  #{$name} {\
            \n    @each $hue in $hues {\
            \n      hue-#{$hue}: hsla($hue, 100%, 50%);\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n@include test-hues(\"red\", 0, -360, 360, 6120);\
            \n@include test-hues(\"yellow\", 60, -300, 420, -9660);\
            \n@include test-hues(\"green\", 120, -240, 480, 99840);\
            \n@include test-hues(\"cyan\", 180, -180, 540, -900);\
            \n@include test-hues(\"blue\", 240, -120, 600, -104880);\
            \n@include test-hues(\"purple\", 300, -60, 660, 2820);\
            \n"
            )
            .unwrap(),
            "red {\
        \n  hue-0: red;\
        \n  hue--360: red;\
        \n  hue-360: red;\
        \n  hue-6120: red;\
        \n}\
        \nyellow {\
        \n  hue-60: yellow;\
        \n  hue--300: yellow;\
        \n  hue-420: yellow;\
        \n  hue--9660: yellow;\
        \n}\
        \ngreen {\
        \n  hue-120: lime;\
        \n  hue--240: lime;\
        \n  hue-480: lime;\
        \n  hue-99840: lime;\
        \n}\
        \ncyan {\
        \n  hue-180: aqua;\
        \n  hue--180: aqua;\
        \n  hue-540: aqua;\
        \n  hue--900: aqua;\
        \n}\
        \nblue {\
        \n  hue-240: blue;\
        \n  hue--120: blue;\
        \n  hue-600: blue;\
        \n  hue--104880: blue;\
        \n}\
        \npurple {\
        \n  hue-300: fuchsia;\
        \n  hue--60: fuchsia;\
        \n  hue-660: fuchsia;\
        \n  hue-2820: fuchsia;\
        \n}\
        \n"
        );
    }
    #[test]
    fn red_to_green() {
        assert_eq!(
            rsass(
                "a {\
            \n  step-1: hsla(0, 100%, 50%);\
            \n  step-2: hsla(12, 100%, 50%);\
            \n  step-3: hsla(24, 100%, 50%);\
            \n  step-4: hsla(36, 100%, 50%);\
            \n  step-5: hsla(48, 100%, 50%);\
            \n  step-6: hsla(60, 100%, 50%);\
            \n  step-7: hsla(72, 100%, 50%);\
            \n  step-8: hsla(84, 100%, 50%);\
            \n  step-9: hsla(96, 100%, 50%);\
            \n  step-10: hsla(108, 100%, 50%);\
            \n  step-11: hsla(120, 100%, 50%);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  step-1: red;\
        \n  step-2: #ff3300;\
        \n  step-3: #ff6600;\
        \n  step-4: #ff9900;\
        \n  step-5: #ffcc00;\
        \n  step-6: yellow;\
        \n  step-7: #ccff00;\
        \n  step-8: #99ff00;\
        \n  step-9: #66ff00;\
        \n  step-10: #33ff00;\
        \n  step-11: lime;\
        \n}\
        \n"
        );
    }
}
