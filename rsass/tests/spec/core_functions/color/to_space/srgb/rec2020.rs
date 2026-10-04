//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1973951735 0.2382060577 0.3275824924 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1973951735 0.2382060577 0.3275824924 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.5 0.5 0.5), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5260663507 0.5260663507 0.5260663507);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.3678067823 0.4291867351 0.7812845626);\
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
             \na {b: color.to-space(color(srgb 0.1 0.2 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1839457841 0.2356023353 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1434273868 none 0.3217178377);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none 0.2360419775 0.3272557162);\
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
             \na {b: color.to-space(color(srgb -999999 0 0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -780533.9562094096 -311307.2375841276 -170937.5910520199);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -0.66014094 0.3976830191 1.8595773836);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 1 1 1), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
