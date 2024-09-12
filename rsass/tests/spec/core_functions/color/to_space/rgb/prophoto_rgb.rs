//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/prophoto_rgb.hrx"

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
             \na {b: color.to-space(rgb(10 20 30 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0568847736 0.0623636876 0.0861178613 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0568847736 0.0623636876 0.0861178613 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3371488337 0.3261543399 0.6930068512);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.6027153447 0.6027153447 0.6027153447);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.3990854669 0.4493912175 0.7905294798);\
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
             \na {b: color.to-space(rgb(10 20 none), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb 0.0459932777 0.0604706464 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb 0.0427390372 none 0.0828155088);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb none 0.0608214606 0.0859153209);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -40436.5951245925 -15876.4400699537 -5962.4442405194);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.6182230836 0.3944820605 1.6451734949);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
