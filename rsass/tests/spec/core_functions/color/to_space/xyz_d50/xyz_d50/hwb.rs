//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/xyz-d50/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(184.0103843189, 495.2078632431%, 10.9589006248%, 0.4);\
         \n  space: hwb;\
         \n  channels: 184.0103843189deg -43.310436994% 34.7717617565% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(184.0103843189, 495.2078632431%, 10.9589006248%, 0);\
         \n  space: hwb;\
         \n  channels: 184.0103843189deg -43.310436994% 34.7717617565% / 0;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(252.6622302958, 18.6415390171%, 76.8792521782%);\
         \n  space: hwb;\
         \n  channels: 252.6622302958deg 72.569188952% 18.8106845955% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(187.9353554297, 490.1229331153%, 17.2918334784%);\
         \n  space: hwb;\
         \n  channels: 187.9353554297deg -67.4594079552% -2.043074912% / 1;\
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(111.1172729557, 222.5792269318%, 17.6906141138%);\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(285.6927471827, 282.9980963007%, 17.9207568387%);\
         \n  space: hwb;\
         \n  channels: 285.6927471827deg -32.7946438576% 31.363842465% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(359.4153454139, 2475.3715741602%, -2.7790249918%);\
         \n  space: hwb;\
         \n  channels: 179.4153454139deg -71.5702196777% 33.9878303059% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(329.431996419, 420.4439814741%, -10316.9080915763%);\
         \n  space: hwb;\
         \n  channels: 149.431996419deg -53693.7272368212% -32959.9110536687% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(3.9698519642, 796.3834139233%, -21.9385057601%);\
         \n  space: hwb;\
         \n  channels: 183.9698519642deg -196.6531268961% -52.7761153759% / 1;\
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
             \n@include utils.inspect(color.to-space(color(xyz-d50 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: hsl(72.6622302958, 128.9066481357%, 104.4631089642%);\
         \n  space: hwb;\
         \n  channels: 252.6622302958deg 98.7098647958% -10.2163531326% / 1;\
         \n}\n"
    );
}
