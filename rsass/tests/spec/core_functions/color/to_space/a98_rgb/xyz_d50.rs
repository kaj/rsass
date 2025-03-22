//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/xyz_d50.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0203795694 0.0246044104 0.054623998 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0203795694 0.0246044104 0.054623998 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.2099807143 0.2177555281 0.1796710885);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1364163751 0.1311360929 0.4645989467);\
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 0.0098137269 0.0201278488 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 0.0144201903 none 0.0528568205);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 none 0.0226378123 0.0545009257);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 -9560512850977.73 -4878046244787.3545 -305274677130.3227);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.1028590408 0.062612892 3.4087872511);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 1 1 1), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.9642956764 1 0.8251046025);\
         \n}\n"
    );
}
