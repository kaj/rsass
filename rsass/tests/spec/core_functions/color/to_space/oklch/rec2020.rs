//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/rec2020.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
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
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.0256576867 -0.0056343197 -0.0019025127 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0256576867 -0.0056343197 -0.0019025127 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.064418765 -0.0232002216 -0.0029907849);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.3319485728 0.3319485809 0.3319486233);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.0256576867 -0.0056343197 -0.0019025127);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% none 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0044999999 0.0045000001 0.0045000011);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0225650026 -0.0048194756 0.0030470892);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0007134353 0.0000902745 -0.002575042);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 56131778.24188723 -38257541.3895273 -10955273.518136343);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2237639493 -0.1587187375 -0.0339392569);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.9999999872 1.0000000081 1.0000001161);\
         \n}\n"
    );
}
