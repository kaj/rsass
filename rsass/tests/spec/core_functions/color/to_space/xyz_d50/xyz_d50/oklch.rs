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
         \n  b: oklch(56.9970926622% 0.1921417442 201.5843389185deg / 0.4);\
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
         \n  b: oklch(56.9970926622% 0.1921417442 201.5843389185deg / 0);\
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
         \n  b: oklch(79.6577658839% 0.0311073605 295.3584786053deg);\
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
         \n  b: oklch(71.9979200541% 0.2904967986 209.0787246058deg);\
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
         \n  b: oklch(56.4114760121% 0.2339193759 138.7662056048deg);\
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
         \n  b: oklch(29.1994683846% 0.4226922374 316.7740736717deg);\
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
         \n  b: oklch(53.737032336% 0.4868840398 188.5426015282deg);\
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
         \n  b: color-mix(in oklch, color(xyz -955472.4660146533 28369.6809641542 -12314.0025504659) 100%, black);\
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
         \n  b: oklch(42.8855772157% 3.2937678635 186.548953155deg);\
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
