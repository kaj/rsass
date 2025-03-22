//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(205.3925362149, 25.148533711%, 46.6510851344%, 0.4);\
         \n  space: hwb;\
         \n  channels: 205.3925362149deg 34.9190212628% 41.616850994% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(205.3925362149, 25.148533711%, 46.6510851344%, 0);\
         \n  space: hwb;\
         \n  channels: 205.3925362149deg 34.9190212628% 41.616850994% / 0;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 73.5356983052%);\
         \n  space: hwb;\
         \n  channels: 0deg 73.5356983052% 26.4643016948% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(214.3023060477, 69.2456926348%, 69.5430478913%);\
         \n  space: hwb;\
         \n  channels: 214.3023060477deg 48.4529204482% 9.3668246656% / 1;\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), hwb)}\n"
            ),
            "a {\
         \n  b: hsl(76.7592364631, 100%, 24.2264602241%);\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(275.8860614996, 100%, 29.191574503%);\
         \n  space: hwb;\
         \n  channels: 275.8860614996deg 0% 41.616850994% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(190.2052342776, 100%, 29.191574503%);\
         \n  space: hwb;\
         \n  channels: 190.2052342776deg 0% 41.616850994% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 100%, -16678.2577069634%);\
         \n  space: hwb;\
         \n  channels: 180deg -33356.5154139268% 100% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(197.5434618594, 666.1615765111%, 17.6628023075%);\
         \n  space: hwb;\
         \n  channels: 197.5434618594deg -100% -35.3256046149% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb-linear 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
