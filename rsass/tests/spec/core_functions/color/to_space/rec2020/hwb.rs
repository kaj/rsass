//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(194.5479244469, 123.11735267%, 16.5168092719%, 0.4);\
         \n  space: hwb;\
         \n  channels: 194.5479244469deg -3.8182490492% 63.148132407% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(194.5479244469, 123.11735267%, 16.5168092719%, 0);\
         \n  space: hwb;\
         \n  channels: 194.5479244469deg -3.8182490492% 63.148132407% / 0;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(rec2020 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 54.6583590878%);\
         \n  space: hwb;\
         \n  channels: 0deg 54.6583590878% 45.3416409122% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(200.8128966593, 189.0732219315%, 29.5081773497%);\
         \n  space: hwb;\
         \n  channels: 200.8128966593deg -26.2838842987% 14.6997610019% / 1;\
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
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(97.48398538, 169.4637013095%, 10.0907863158%);\
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
             \n@include utils.inspect(color.to-space(color(rec2020 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(273.3326815842, 128.5408000864%, 16.5048029077%);\
         \n  space: hwb;\
         \n  channels: 273.3326815842deg -4.7106028025% 62.2797913821% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(189.4362583411, 400.9276505087%, 7.3695655834%);\
         \n  space: hwb;\
         \n  channels: 189.4362583411deg -22.1770605629% 63.0838082703% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(351.6022221471, 202.9643125658%, -14161586.907056702%);\
         \n  space: hwb;\
         \n  channels: 171.6022221471deg -42904554.421379425% -14581280.607266026% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(204.9795970204, 570.1567645938%, 29.20918492%);\
         \n  space: hwb;\
         \n  channels: 204.9795970204deg -137.3289587842% -95.7473286243% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
