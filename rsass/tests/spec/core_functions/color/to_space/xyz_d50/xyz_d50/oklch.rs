//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/xyz-d50/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.9970930189% 0.1921417558 201.5843445791deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.9970930189% 0.1921417558 201.5843445791deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), oklch)}\n"),
        "a {\
         \n  b: oklch(79.6577663953% 0.0311073874 295.3584561891deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), oklch)}\n"),
        "a {\
         \n  b: oklch(71.9979204728% 0.2904968159 209.0787289615deg);\
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), oklch)}\n"),
            "a {\
         \n  b: oklch(56.4114763984% 0.2339193642 138.7662106424deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(29.1994680372% 0.4226922646 316.7740746587deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(53.7370326457% 0.4868840502 188.5426040002deg);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -955472.4660146532 28369.6809641542 -12314.002550467) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(42.8855762363% 3.2937679404 186.5489535718deg);\
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
             \na {b: color.to-space(color(xyz-d50 1 1 1), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 0.9956342097 1.0026671299 1.3221722918) 100%, black);\
         \n}\n"
    );
}
