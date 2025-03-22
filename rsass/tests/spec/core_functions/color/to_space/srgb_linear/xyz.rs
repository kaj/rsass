//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/xyz.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1669001843 0.1859553309 0.3109316835 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1669001843 0.1859553309 0.3109316835 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.4752279635 0.5 0.5445288754);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.3698965263 0.386349125 0.8119697975);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), xyz)}\n"
            ),
            "a {\
         \n  b: color(xyz 0.1127559478 0.1642976363 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), xyz)}\n"
            ),
            "a {\
         \n  b: color(xyz 0.0953833164 none 0.2870927275);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), xyz)}\n"
            ),
            "a {\
         \n  b: color(xyz none 0.1646914304 0.3089986016);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz -412390.3868751602 -212638.7932325045 -19330.7993847731);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0916045133 0.2178130964 1.9294113977);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559271 1 1.0890577508);\
         \n}\n"
    );
}
