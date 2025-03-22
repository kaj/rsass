//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(210, 50%, 20%, 0.4);\
         \n  space: hwb;\
         \n  channels: 210deg 10% 70% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(210, 50%, 20%, 0);\
         \n  space: hwb;\
         \n  channels: 210deg 10% 70% / 0;\
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
             \n@include utils.inspect(color.to-space(color(srgb 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(srgb 0.5 0.5 0.5), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(srgb 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: #3366cc;\
         \n  space: hwb;\
         \n  channels: 220deg 20% 20% / 1;\
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
             \na {b: color.to-space(color(srgb 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(90, 100%, 10%);\
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
             \n@include utils.inspect(color.to-space(color(srgb 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(260, 100%, 15%);\
         \n  space: hwb;\
         \n  channels: 260deg 0% 70% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(200, 100%, 15%);\
         \n  space: hwb;\
         \n  channels: 200deg 0% 70% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 100%, -49999950%);\
         \n  space: hwb;\
         \n  channels: 180deg -99999900% 100% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(212, 300%, 50%);\
         \n  space: hwb;\
         \n  channels: 212deg -100% -100% / 1;\
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
             \n@include utils.inspect(color.to-space(color(srgb 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
