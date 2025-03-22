//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(200.1785406812, 126.7161502744%, 13.1753745837%, 0.4);\
         \n  space: hwb;\
         \n  channels: 200.1785406812deg -3.519952873% 70.1292979595% / 0.4;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(200.1785406812, 126.7161502744%, 13.1753745837%, 0);\
         \n  space: hwb;\
         \n  channels: 200.1785406812deg -3.519952873% 70.1292979595% / 0;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0 0 0), hwb));\n"
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 50.3992895764%);\
         \n  space: hwb;\
         \n  channels: 0deg 50.3992895764% 49.6007104236% / 1;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(206.798941326, 132.7737671841%, 35.0907131834%);\
         \n  space: hwb;\
         \n  channels: 206.798941326deg -11.500548642% 18.3180249912% / 1;\
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(125.1711076789, 146.5566174361%, 7.5605855126%);\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(258.3433021907, 100%, 15.0612047755%);\
         \n  space: hwb;\
         \n  channels: 258.3433021907deg 0% 69.8775904491% / 1;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(196.5051304863, 215.7701724503%, 9.4596338244%);\
         \n  space: hwb;\
         \n  channels: 196.5051304863deg -10.9514343917% 70.1292979595% / 1;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 100%, -19096022.06943802%);\
         \n  space: hwb;\
         \n  channels: 180deg -38192044.13887604% 100% / 1;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(209.922630637, 396.8439205726%, 39.6562294521%);\
         \n  space: hwb;\
         \n  channels: 209.922630637deg -117.7171062569% -97.0295651612% / 1;\
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
             \n@include utils.inspect(color.to-space(color(a98-rgb 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
