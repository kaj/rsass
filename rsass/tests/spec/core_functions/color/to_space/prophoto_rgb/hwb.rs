//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(190.4112342736, 389.9464516401%, 7.9139613859%, 0.4);\
         \n  space: hwb;\
         \n  channels: 190.4112342736deg -22.9462502225% 61.2258270058% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(190.4112342736, 389.9464516401%, 7.9139613859%, 0);\
         \n  space: hwb;\
         \n  channels: 190.4112342736deg -22.9462502225% 61.2258270058% / 0;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 57.2306753164%);\
         \n  space: hwb;\
         \n  channels: 0deg 57.2306753164% 42.7693246836% / 1;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(195.9034030348, 388.9813639336%, 17.9876851056%);\
         \n  space: hwb;\
         \n  channels: 195.9034030348deg -51.9810577583% 12.0435720304% / 1;\
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
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), hwb)}\n"
        ),
        "a {\
         \n  b: hsl(119.2084673976, 194.643672602%, 9.5495891256%);\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(241.7396943935, 128.7470611392%, 17.4883701258%);\
         \n  space: hwb;\
         \n  channels: 241.7396943935deg -5.0273924523% 59.9958672962% / 1;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(188.6270203536, 825.3071653223%, 4.1925835467%);\
         \n  space: hwb;\
         \n  channels: 188.6270203536deg -30.409108876% 61.2057240306% / 1;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(347.1631207662, 234.6485806965%, -1340219.878310844%);\
         \n  space: hwb;\
         \n  channels: 167.1631207662deg -4485026.800979206% -1804487.0443575173% / 1;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(199.2935266227, 2154.1559841675%, 8.1167706475%);\
         \n  space: hwb;\
         \n  channels: 199.2935266227deg -166.731129976% -82.9646712709% / 1;\
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
             \n@include utils.inspect(color.to-space(color(prophoto-rgb 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
