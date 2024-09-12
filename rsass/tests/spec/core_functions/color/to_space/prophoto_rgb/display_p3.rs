//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/display_p3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.1681406496 0.272751421 0.3768280764 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.1681406496 0.272751421 0.3768280764 / 0);\
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
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.5723067532 0.5723067532 0.5723067532);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.4263141523 0.4923548598 0.8503908535);\
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.0601855627 0.2760901066 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.0396944837 none 0.3821042494);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 none 0.278189942 0.3765750961);\
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
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -40921.237624831 15288.5098255533 -4976.1422545962);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -1.5123725709 0.6119230865 1.7528266609);\
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
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
