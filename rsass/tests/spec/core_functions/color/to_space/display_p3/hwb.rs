//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(205.417452683, 67.7488400949%, 18.4043245596%, 0.4);\
         \n  space: hwb;\
         \n  channels: 205.417452683deg 5.9356081432% 69.1269590239% / 0.4;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(205.417452683, 67.7488400949%, 18.4043245596%, 0);\
         \n  space: hwb;\
         \n  channels: 205.417452683deg 5.9356081432% 69.1269590239% / 0;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0 0 0), hwb));\n"
        ),
        "a {\
         \n  value: black;\
         \n  space: hwb;\
         \n  channels: 0deg 0% 100% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 50%);\
         \n  space: hwb;\
         \n  channels: 0deg 50% 50% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(214.974668252, 77.667451385%, 46.5940983001%);\
         \n  space: hwb;\
         \n  channels: 214.974668252deg 10.4056496546% 17.2174530544% / 1;\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(display-p3 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(96.0435532608, 143.3480015017%, 8.3456369204%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(262.3902012265, 103.5328302701%, 15.4158592357%);\
         \n  space: hwb;\
         \n  channels: 262.3902012265deg -0.5446161415% 68.6236653872% / 1;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(196.0636856013, 172.1527011916%, 11.3581215767%);\
         \n  space: hwb;\
         \n  channels: 196.0636856013deg -8.1951915222% 69.0885653244% / 1;\
         \n}\n"
    );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(356.7852726724, 165.043052964%, -41057989.10847679%);\
         \n  space: hwb;\
         \n  channels: 176.7852726724deg -108821347.81871557% -26705269.601762004% / 1;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(210.5153182162, 325.7395089334%, 48.9349840904%);\
         \n  space: hwb;\
         \n  channels: 210.5153182162deg -110.4655927824% -108.3355609633% / 1;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
