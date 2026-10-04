//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/lab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), lab)}\n"
        ),
        "a {\
         \n  b: lab(14.3089075958% -11.2831148392 -23.5554727619 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), lab)}\n"
        ),
        "a {\
         \n  b: lab(14.3089075958% -11.2831148392 -23.5554727619 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), lab)}\n"),
        "a {\
         \n  b: lab(50.6245045898% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), lab)}\n"),
        "a {\
         \n  b: lab(39.1500175267% 1.6091507246 -70.6857900987);\
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
             \na {b: color.to-space(color(rec2020 none none none), lab)}\n"),
            "a {\
         \n  b: lab(none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), lab)}\n"),
            "a {\
         \n  b: lab(12.7980788363% -30.2799799635 20.8912193128);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), lab)}\n"),
            "a {\
         \n  b: lab(3.2949420149% 24.5286595615 -42.1891966298);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), lab)}\n"),
            "a {\
         \n  b: lab(13.6661446251% -17.2720826701 -24.6680105138);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -999999 0 0), lab)}\n"),
            "a {\
         \n  b: color-mix(in lab, color(xyz -159996243697881.16 -65987151372172.414 4360948.267700195) 100%, black);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), lab)}\n"),
            "a {\
         \n  b: lab(22.5264188072% -76.6956223143 -277.947778702);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), lab)}\n"),
        "a {\
         \n  b: lab(100% 0 0);\
         \n}\n"
    );
}
