//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("w3c")
}

mod black_to_white_through {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(240, 100%, 0%);\
             \n  step-2: hsl(240, 100%, 10%);\
             \n  step-3: hsl(240, 100%, 20%);\
             \n  step-4: hsl(240, 100%, 30%);\
             \n  step-5: hsl(240, 100%, 40%);\
             \n  step-6: hsl(240, 100%, 50%);\
             \n  step-7: hsl(240, 100%, 60%);\
             \n  step-8: hsl(240, 100%, 70%);\
             \n  step-9: hsl(240, 100%, 80%);\
             \n  step-10: hsl(240, 100%, 90%);\
             \n  step-11: hsl(240, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(240deg, 100%, 0%);\
         \n  step-2: hsl(240deg, 100%, 10%);\
         \n  step-3: hsl(240deg, 100%, 20%);\
         \n  step-4: hsl(240deg, 100%, 30%);\
         \n  step-5: hsl(240deg, 100%, 40%);\
         \n  step-6: hsl(240deg, 100%, 50%);\
         \n  step-7: hsl(240deg, 100%, 60%);\
         \n  step-8: hsl(240deg, 100%, 70%);\
         \n  step-9: hsl(240deg, 100%, 80%);\
         \n  step-10: hsl(240deg, 100%, 90%);\
         \n  step-11: hsl(240deg, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn cyan() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(180, 100%, 0%);\
             \n  step-2: hsl(180, 100%, 10%);\
             \n  step-3: hsl(180, 100%, 20%);\
             \n  step-4: hsl(180, 100%, 30%);\
             \n  step-5: hsl(180, 100%, 40%);\
             \n  step-6: hsl(180, 100%, 50%);\
             \n  step-7: hsl(180, 100%, 60%);\
             \n  step-8: hsl(180, 100%, 70%);\
             \n  step-9: hsl(180, 100%, 80%);\
             \n  step-10: hsl(180, 100%, 90%);\
             \n  step-11: hsl(180, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(180deg, 100%, 0%);\
         \n  step-2: hsl(180deg, 100%, 10%);\
         \n  step-3: hsl(180deg, 100%, 20%);\
         \n  step-4: hsl(180deg, 100%, 30%);\
         \n  step-5: hsl(180deg, 100%, 40%);\
         \n  step-6: hsl(180deg, 100%, 50%);\
         \n  step-7: hsl(180deg, 100%, 60%);\
         \n  step-8: hsl(180deg, 100%, 70%);\
         \n  step-9: hsl(180deg, 100%, 80%);\
         \n  step-10: hsl(180deg, 100%, 90%);\
         \n  step-11: hsl(180deg, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(120, 100%, 0%);\
             \n  step-2: hsl(120, 100%, 10%);\
             \n  step-3: hsl(120, 100%, 20%);\
             \n  step-4: hsl(120, 100%, 30%);\
             \n  step-5: hsl(120, 100%, 40%);\
             \n  step-6: hsl(120, 100%, 50%);\
             \n  step-7: hsl(120, 100%, 60%);\
             \n  step-8: hsl(120, 100%, 70%);\
             \n  step-9: hsl(120, 100%, 80%);\
             \n  step-10: hsl(120, 100%, 90%);\
             \n  step-11: hsl(120, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(120deg, 100%, 0%);\
         \n  step-2: hsl(120deg, 100%, 10%);\
         \n  step-3: hsl(120deg, 100%, 20%);\
         \n  step-4: hsl(120deg, 100%, 30%);\
         \n  step-5: hsl(120deg, 100%, 40%);\
         \n  step-6: hsl(120deg, 100%, 50%);\
         \n  step-7: hsl(120deg, 100%, 60%);\
         \n  step-8: hsl(120deg, 100%, 70%);\
         \n  step-9: hsl(120deg, 100%, 80%);\
         \n  step-10: hsl(120deg, 100%, 90%);\
         \n  step-11: hsl(120deg, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn purple() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(300, 100%, 0%);\
             \n  step-2: hsl(300, 100%, 10%);\
             \n  step-3: hsl(300, 100%, 20%);\
             \n  step-4: hsl(300, 100%, 30%);\
             \n  step-5: hsl(300, 100%, 40%);\
             \n  step-6: hsl(300, 100%, 50%);\
             \n  step-7: hsl(300, 100%, 60%);\
             \n  step-8: hsl(300, 100%, 70%);\
             \n  step-9: hsl(300, 100%, 80%);\
             \n  step-10: hsl(300, 100%, 90%);\
             \n  step-11: hsl(300, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(300deg, 100%, 0%);\
         \n  step-2: hsl(300deg, 100%, 10%);\
         \n  step-3: hsl(300deg, 100%, 20%);\
         \n  step-4: hsl(300deg, 100%, 30%);\
         \n  step-5: hsl(300deg, 100%, 40%);\
         \n  step-6: hsl(300deg, 100%, 50%);\
         \n  step-7: hsl(300deg, 100%, 60%);\
         \n  step-8: hsl(300deg, 100%, 70%);\
         \n  step-9: hsl(300deg, 100%, 80%);\
         \n  step-10: hsl(300deg, 100%, 90%);\
         \n  step-11: hsl(300deg, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(0, 100%, 0%);\
             \n  step-2: hsl(0, 100%, 10%);\
             \n  step-3: hsl(0, 100%, 20%);\
             \n  step-4: hsl(0, 100%, 30%);\
             \n  step-5: hsl(0, 100%, 40%);\
             \n  step-6: hsl(0, 100%, 50%);\
             \n  step-7: hsl(0, 100%, 60%);\
             \n  step-8: hsl(0, 100%, 70%);\
             \n  step-9: hsl(0, 100%, 80%);\
             \n  step-10: hsl(0, 100%, 90%);\
             \n  step-11: hsl(0, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(0deg, 100%, 0%);\
         \n  step-2: hsl(0deg, 100%, 10%);\
         \n  step-3: hsl(0deg, 100%, 20%);\
         \n  step-4: hsl(0deg, 100%, 30%);\
         \n  step-5: hsl(0deg, 100%, 40%);\
         \n  step-6: hsl(0deg, 100%, 50%);\
         \n  step-7: hsl(0deg, 100%, 60%);\
         \n  step-8: hsl(0deg, 100%, 70%);\
         \n  step-9: hsl(0deg, 100%, 80%);\
         \n  step-10: hsl(0deg, 100%, 90%);\
         \n  step-11: hsl(0deg, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn yellow() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(60, 100%, 0%);\
             \n  step-2: hsl(60, 100%, 10%);\
             \n  step-3: hsl(60, 100%, 20%);\
             \n  step-4: hsl(60, 100%, 30%);\
             \n  step-5: hsl(60, 100%, 40%);\
             \n  step-6: hsl(60, 100%, 50%);\
             \n  step-7: hsl(60, 100%, 60%);\
             \n  step-8: hsl(60, 100%, 70%);\
             \n  step-9: hsl(60, 100%, 80%);\
             \n  step-10: hsl(60, 100%, 90%);\
             \n  step-11: hsl(60, 100%, 100%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(60deg, 100%, 0%);\
         \n  step-2: hsl(60deg, 100%, 10%);\
         \n  step-3: hsl(60deg, 100%, 20%);\
         \n  step-4: hsl(60deg, 100%, 30%);\
         \n  step-5: hsl(60deg, 100%, 40%);\
         \n  step-6: hsl(60deg, 100%, 50%);\
         \n  step-7: hsl(60deg, 100%, 60%);\
         \n  step-8: hsl(60deg, 100%, 70%);\
         \n  step-9: hsl(60deg, 100%, 80%);\
         \n  step-10: hsl(60deg, 100%, 90%);\
         \n  step-11: hsl(60deg, 100%, 100%);\
         \n}\n"
        );
    }
}
#[test]
fn blue_to_red() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(240, 100%, 50%);\
             \n  step-2: hsl(252, 100%, 50%);\
             \n  step-3: hsl(264, 100%, 50%);\
             \n  step-4: hsl(276, 100%, 50%);\
             \n  step-5: hsl(288, 100%, 50%);\
             \n  step-6: hsl(300, 100%, 50%);\
             \n  step-7: hsl(312, 100%, 50%);\
             \n  step-8: hsl(324, 100%, 50%);\
             \n  step-9: hsl(336, 100%, 50%);\
             \n  step-10: hsl(348, 100%, 50%);\
             \n  step-11: hsl(360, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(240deg, 100%, 50%);\
         \n  step-2: hsl(252deg, 100%, 50%);\
         \n  step-3: hsl(264deg, 100%, 50%);\
         \n  step-4: hsl(276deg, 100%, 50%);\
         \n  step-5: hsl(288deg, 100%, 50%);\
         \n  step-6: hsl(300deg, 100%, 50%);\
         \n  step-7: hsl(312deg, 100%, 50%);\
         \n  step-8: hsl(324deg, 100%, 50%);\
         \n  step-9: hsl(336deg, 100%, 50%);\
         \n  step-10: hsl(348deg, 100%, 50%);\
         \n  step-11: hsl(0deg, 100%, 50%);\
         \n}\n"
    );
}
mod gray_to {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(240, 20%, 50%);\
             \n  step-2: hsl(240, 60%, 50%);\
             \n  step-3: hsl(240, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(240deg, 20%, 50%);\
         \n  step-2: hsl(240deg, 60%, 50%);\
         \n  step-3: hsl(240deg, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn cyan() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(180, 20%, 50%);\
             \n  step-2: hsl(180, 60%, 50%);\
             \n  step-3: hsl(180, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(180deg, 20%, 50%);\
         \n  step-2: hsl(180deg, 60%, 50%);\
         \n  step-3: hsl(180deg, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(120, 20%, 50%);\
             \n  step-2: hsl(120, 60%, 50%);\
             \n  step-3: hsl(120, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(120deg, 20%, 50%);\
         \n  step-2: hsl(120deg, 60%, 50%);\
         \n  step-3: hsl(120deg, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn purple() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(300, 20%, 50%);\
             \n  step-2: hsl(300, 60%, 50%);\
             \n  step-3: hsl(300, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(300deg, 20%, 50%);\
         \n  step-2: hsl(300deg, 60%, 50%);\
         \n  step-3: hsl(300deg, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(0, 20%, 50%);\
             \n  step-2: hsl(0, 60%, 50%);\
             \n  step-3: hsl(0, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(0deg, 20%, 50%);\
         \n  step-2: hsl(0deg, 60%, 50%);\
         \n  step-3: hsl(0deg, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn yellow() {
        assert_eq!(
            runner().ok("a {\
             \n  step-1: hsl(60, 20%, 50%);\
             \n  step-2: hsl(60, 60%, 50%);\
             \n  step-3: hsl(60, 100%, 50%);\
             \n}\n"),
            "a {\
         \n  step-1: hsl(60deg, 20%, 50%);\
         \n  step-2: hsl(60deg, 60%, 50%);\
         \n  step-3: hsl(60deg, 100%, 50%);\
         \n}\n"
        );
    }
}
#[test]
fn green_to_blue() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(120, 100%, 50%);\
             \n  step-2: hsl(132, 100%, 50%);\
             \n  step-3: hsl(144, 100%, 50%);\
             \n  step-4: hsl(156, 100%, 50%);\
             \n  step-5: hsl(168, 100%, 50%);\
             \n  step-6: hsl(180, 100%, 50%);\
             \n  step-7: hsl(192, 100%, 50%);\
             \n  step-8: hsl(204, 100%, 50%);\
             \n  step-9: hsl(216, 100%, 50%);\
             \n  step-10: hsl(228, 100%, 50%);\
             \n  step-11: hsl(240, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(120deg, 100%, 50%);\
         \n  step-2: hsl(132deg, 100%, 50%);\
         \n  step-3: hsl(144deg, 100%, 50%);\
         \n  step-4: hsl(156deg, 100%, 50%);\
         \n  step-5: hsl(168deg, 100%, 50%);\
         \n  step-6: hsl(180deg, 100%, 50%);\
         \n  step-7: hsl(192deg, 100%, 50%);\
         \n  step-8: hsl(204deg, 100%, 50%);\
         \n  step-9: hsl(216deg, 100%, 50%);\
         \n  step-10: hsl(228deg, 100%, 50%);\
         \n  step-11: hsl(240deg, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn hue() {
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
         \n  hue-0: hsl(0deg, 100%, 50%);\
         \n  hue--360: hsl(0deg, 100%, 50%);\
         \n  hue-360: hsl(0deg, 100%, 50%);\
         \n  hue-6120: hsl(0deg, 100%, 50%);\
         \n}\
         \nyellow {\
         \n  hue-60: hsl(60deg, 100%, 50%);\
         \n  hue--300: hsl(60deg, 100%, 50%);\
         \n  hue-420: hsl(60deg, 100%, 50%);\
         \n  hue--9660: hsl(60deg, 100%, 50%);\
         \n}\
         \ngreen {\
         \n  hue-120: hsl(120deg, 100%, 50%);\
         \n  hue--240: hsl(120deg, 100%, 50%);\
         \n  hue-480: hsl(120deg, 100%, 50%);\
         \n  hue-99840: hsl(120deg, 100%, 50%);\
         \n}\
         \ncyan {\
         \n  hue-180: hsl(180deg, 100%, 50%);\
         \n  hue--180: hsl(180deg, 100%, 50%);\
         \n  hue-540: hsl(180deg, 100%, 50%);\
         \n  hue--900: hsl(180deg, 100%, 50%);\
         \n}\
         \nblue {\
         \n  hue-240: hsl(240deg, 100%, 50%);\
         \n  hue--120: hsl(240deg, 100%, 50%);\
         \n  hue-600: hsl(240deg, 100%, 50%);\
         \n  hue--104880: hsl(240deg, 100%, 50%);\
         \n}\
         \npurple {\
         \n  hue-300: hsl(300deg, 100%, 50%);\
         \n  hue--60: hsl(300deg, 100%, 50%);\
         \n  hue-660: hsl(300deg, 100%, 50%);\
         \n  hue-2820: hsl(300deg, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn red_to_green() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(0, 100%, 50%);\
             \n  step-2: hsl(12, 100%, 50%);\
             \n  step-3: hsl(24, 100%, 50%);\
             \n  step-4: hsl(36, 100%, 50%);\
             \n  step-5: hsl(48, 100%, 50%);\
             \n  step-6: hsl(60, 100%, 50%);\
             \n  step-7: hsl(72, 100%, 50%);\
             \n  step-8: hsl(84, 100%, 50%);\
             \n  step-9: hsl(96, 100%, 50%);\
             \n  step-10: hsl(108, 100%, 50%);\
             \n  step-11: hsl(120, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(0deg, 100%, 50%);\
         \n  step-2: hsl(12deg, 100%, 50%);\
         \n  step-3: hsl(24deg, 100%, 50%);\
         \n  step-4: hsl(36deg, 100%, 50%);\
         \n  step-5: hsl(48deg, 100%, 50%);\
         \n  step-6: hsl(60deg, 100%, 50%);\
         \n  step-7: hsl(72deg, 100%, 50%);\
         \n  step-8: hsl(84deg, 100%, 50%);\
         \n  step-9: hsl(96deg, 100%, 50%);\
         \n  step-10: hsl(108deg, 100%, 50%);\
         \n  step-11: hsl(120deg, 100%, 50%);\
         \n}\n"
    );
}
