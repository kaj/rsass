//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/xyz.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.4), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0060963544 0.0065855902 0.0132332803 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0060963544 0.0065855902 0.0132332803 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, xyz)}\n"),
        "a {\
         \n  b: color(xyz 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1642050952 0.1416464224 0.5663237374);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.3820621634 0.4019777798 0.4377770168);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.2251320227 0.2316759373 0.7169422823);\
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
             \na {b: color.to-space(rgb(10 20 none), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0037531665 0.005648315 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0035949053 none 0.0123994639);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), xyz)}\n"),
            "a {\
         \n  b: color(xyz none 0.0059401734 0.0131746061);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz -152693379.439195 -78732523.77333492 -7157502.1612122655);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.5403326817 0.2875237342 2.689600722);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559271 1 1.0890577508);\
         \n}\n"
    );
}
