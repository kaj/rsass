//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/a98_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.4), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0827079329 0.1047180692 0.1375163198 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0827079329 0.1047180692 0.1375163198 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2781926905 0.395603604 0.7684686532);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.660735938 0.660735938 0.660735938);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.3199480242 0.5287075634 0.8521762133);\
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
             \na {b: color.to-space(rgb(10 20 none), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0827079329 0.1047180692 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0615069668 none 0.1360938515);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb none 0.1047180692 0.1375163198);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -6760.0211192379 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.1411766693 0.3918977213 1.5718485866);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
}
