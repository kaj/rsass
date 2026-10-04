//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/hwb.hrx"

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
             \n@include utils.inspect(color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(197.5454983219, 213.6685362748%, 8.663307507%, 0.4);\
         \n  space: hwb;\
         \n  channels: 197.5454983219deg -9.8474548362% 72.8259301499% / 0.4;\
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
         \n  value: hsla(197.5454983219, 213.6685362748%, 8.663307507%, 0);\
         \n  space: hwb;\
         \n  channels: 197.5454983219deg -9.8474548362% 72.8259301499% / 0;\
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
         \n  value: hsl(0, 0%, 47.25%);\
         \n  space: hwb;\
         \n  channels: 0deg 47.25% 52.75% / 1;\
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
         \n  value: hsl(203.7709520539, 214.3683584242%, 26.1752302928%);\
         \n  space: hwb;\
         \n  channels: 203.7709520539deg -29.9361811997% 17.7133582147% / 1;\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none none none), hwb)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(130.1929265324, 239.2223908609%, 4.873488282%);\
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
         \n  value: hsl(249.4455101576, 109.394326268%, 13.2040950469%);\
         \n  space: hwb;\
         \n  channels: 249.4455101576deg -1.2404357694% 72.3513741368% / 1;\
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
         \n  value: hsl(195.6868905228, 298.0635998534%, 6.8306811002%);\
         \n  space: hwb;\
         \n  channels: 195.6868905228deg -13.5290928816% 72.8095449179% / 1;\
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
             \n@include utils.inspect(color.to-space(color(rec2020 -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(351.602223225, 202.9643386172%, -43015573.24931286%);\
         \n  space: hwb;\
         \n  channels: 171.602223225deg -130321846.99719346% -44290600.49856773% / 1;\
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
         \n  value: hsl(208.2154252683, 458.8282922904%, 38.5998726017%);\
         \n  space: hwb;\
         \n  channels: 208.2154252683deg -138.5072636829% -115.7070088863% / 1;\
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
