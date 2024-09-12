//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(179.5022543706, 556.250481638%, 8.7700702541%, 0.4);\
         \n  space: hwb;\
         \n  channels: 179.5022543706deg -40.0134877742% 42.4463717177% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(179.5022543706, 556.250481638%, 8.7700702541%, 0);\
         \n  space: hwb;\
         \n  channels: 179.5022543706deg -40.0134877742% 42.4463717177% / 0;\
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
             \n@include utils.inspect(color.to-space(color(xyz 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(xyz 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(8.6326376323, 19.0960524665%, 75.1815938992%);\
         \n  space: hwb;\
         \n  channels: 8.6326376323deg 70.4422580488% 20.0790702505% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(183.9973689591, 600.9357681928%, 12.7508937669%);\
         \n  space: hwb;\
         \n  channels: 183.9973689591deg -63.8737876426% 10.6244248236% / 1;\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(xyz 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(93.2964667331, 215.664278299%, 17.8710983929%);\
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
             \n@include utils.inspect(color.to-space(color(xyz 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(290.3526254976, 328.3439800543%, 14.0892871543%);\
         \n  space: hwb;\
         \n  channels: 290.3526254976deg -32.1720390494% 39.6493866421% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(355.8794204538, 2697.9214173204%, -2.5244914397%);\
         \n  space: hwb;\
         \n  channels: 175.8794204538deg -70.6332866707% 34.4156962088% / 1;\
         \n}\n"
    );
    }
}
mod out_of_range {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(xyz -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(330.5196564153, 405.9398117154%, -10761.9459979264%);\
         \n  space: hwb;\
         \n  channels: 150.5196564153deg -54448.9693188225% -32825.0773229696% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0.951270101, 523.3395920082%, -31.8043324514%);\
         \n  space: hwb;\
         \n  channels: 180.951270101deg -198.2489961434% -34.6403312406% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: hsl(188.6326376323, 287.948753728%, 102.1970070346%);\
         \n  space: hwb;\
         \n  channels: 8.6326376323deg 95.8707526592% -8.5232614099% / 1;\
         \n}\n"
    );
}
