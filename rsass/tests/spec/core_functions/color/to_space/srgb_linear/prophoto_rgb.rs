//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/prophoto_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3626946772 0.4009240289 0.4977561426 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3626946772 0.4009240289 0.4977561426 / 0);\
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
             \na {b: color.to-space(color(srgb-linear 0 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.6803950001 0.6803950001 0.6803950001);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.5584197658 0.5940140048 0.8477459947);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3064298164 0.3910752998 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2705939482 none 0.4744752789);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb none 0.3894388678 0.4961157804);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -1512.946773752 -594.0215467819 -223.0865568571);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -0.302285213 0.5192197728 1.3694597345);\
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
             \na {b: color.to-space(color(srgb-linear 1 1 1), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
