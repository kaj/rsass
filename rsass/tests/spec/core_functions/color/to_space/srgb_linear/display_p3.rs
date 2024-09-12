//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/display_p3.hrx"

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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.377671895 0.4807798914 0.5742767089 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.377671895 0.4807798914 0.5742767089 / 0);\
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
             \na {b: color.to-space(color(srgb-linear 0 0 0), display-p3)}\n"
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
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.7353569831 0.7353569831 0.7353569831);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.5225476892 0.6601803521 0.8864121553);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.377671895 0.4807798914 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.3175781541 none 0.5609588905);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 none 0.4769934816 0.5727260345);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -307.4724395956 -80.6737432685 -61.1542078437);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.8815767709 0.6290624928 1.3029857539);\
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
             \na {b: color.to-space(color(srgb-linear 1 1 1), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
