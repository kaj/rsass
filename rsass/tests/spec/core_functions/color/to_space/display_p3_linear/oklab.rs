//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.6425373683% -0.0373900665 -0.0535263799 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.6425373683% -0.0373900665 -0.0535263799 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(79.3700525984% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(72.6776426801% -0.0374141809 -0.1088046304);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(53.2843793949% -0.1024723411 0.1284163638);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(38.9886281594% 0.1244730264 -0.1712881884);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(53.4259235831% -0.1040953535 -0.0684449379);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -486570.4620772681 -228974.3350951847 0.0000000001) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(38.7754419937% -2.1877445581 -0.4122088742);\
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
             \na {b: color.to-space(color(display-p3-linear 1 1 1), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(100% 0 0);\
         \n}\n"
    );
}
