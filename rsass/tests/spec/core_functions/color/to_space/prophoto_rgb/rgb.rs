//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), rgb)}\n"
        ),
        "a {\
         \n  b: hsla(190.4112342736, 389.9464516401%, 7.9139613859%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), rgb)}\n"
        ),
        "a {\
         \n  b: hsla(190.4112342736, 389.9464516401%, 7.9139613859%, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(145.9382220568, 145.9382220568, 145.9382220568);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(195.9034030348, 388.9813639336%, 17.9876851056%);\
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(133.9801165671, 187.2672802248%, 9.7948015855%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(234.8931237502, 120.5152085415%, 18.1412125578%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(0, 73.552095814, 98.9254037219);\
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
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), rgb)}\n"
            ),
            "a {\
         \n  b: hsl(347.1631207662, 234.6485806965%, -1340219.8783108443%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), rgb)}\n"),
            "a {\
         \n  b: hsl(199.2935266227, 2154.1559841675%, 8.1167706475%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
