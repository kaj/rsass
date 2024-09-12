//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(rgb(10 20 30 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(210, 50%, 7.8431372549%, 0.4);\
         \n  space: hwb;\
         \n  channels: 210deg 3.9215686275% 88.2352941176% / 0.4;\
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
             \n@include utils.inspect(color.to-space(rgb(10 20 30 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(210, 50%, 7.8431372549%, 0);\
         \n  space: hwb;\
         \n  channels: 210deg 3.9215686275% 88.2352941176% / 0;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(#000, hwb));\n"),
        "a {\
         \n  value: black;\
         \n  space: hwb;\
         \n  channels: 0deg 0% 100% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(\
             \n  color.to-space(rgb(50.123456789 100.987654321 200.192837465), hwb)\
             \n);\n"
        ),
        "a {\
         \n  value: hsl(219.6637272829, 59.9519025013%, 49.0816263243%);\
         \n  space: hwb;\
         \n  channels: 219.6637272829deg 19.6562575643% 21.4930049157% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(#aaa, hwb));\n"),
        "a {\
         \n  value: #aaaaaa;\
         \n  space: hwb;\
         \n  channels: 0deg 66.6666666667% 33.3333333333% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(#28d, hwb));\n"),
        "a {\
         \n  value: #2288dd;\
         \n  space: hwb;\
         \n  channels: 207.2727272727deg 13.3333333333% 13.3333333333% / 1;\
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
             \na {b: color.to-space(rgb(10 20 none), hwb)}\n"),
            "a {\
         \n  b: #0a1400;\
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
             \n@include utils.inspect(color.to-space(rgb(10 none 30), hwb));\n"
        ),
        "a {\
         \n  value: #0a001e;\
         \n  space: hwb;\
         \n  channels: 260deg 0% 88.2352941176% / 1;\
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
             \n@include utils.inspect(color.to-space(rgb(none 20 30), hwb));\n"
        ),
        "a {\
         \n  value: #00141e;\
         \n  space: hwb;\
         \n  channels: 200deg 0% 88.2352941176% / 1;\
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
             \n@include utils.inspect(color.to-space(color.change(black, $red: -999999), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 100%, -196078.2352941177%);\
         \n  space: hwb;\
         \n  channels: 180deg -392156.4705882354% 100% / 1;\
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
             \n@include utils.inspect(color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), hwb));\n"
        ),
        "a {\
         \n  value: hsl(220, 281.25%, 68.6274509804%);\
         \n  space: hwb;\
         \n  channels: 220deg -19.6078431373% -56.862745098% / 1;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(#fff, hwb));\n"),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
