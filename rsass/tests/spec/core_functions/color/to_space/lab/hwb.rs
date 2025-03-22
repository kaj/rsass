//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.4), hwb)}\n"),
            "a {\
         \n  b: hsla(28.3698264077, 277.3239474447%, 5.9154081349%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), hwb)}\n"),
            "a {\
         \n  b: hsla(28.3698264077, 277.3239474447%, 5.9154081349%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), hwb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), hwb)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 46.6326609284%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), hwb)}\n"),
        "a {\
         \n  b: hsl(260.0900301863, 93.9881625698%, 65.88440774%);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none 30), hwb)}\n"),
            "a {\
         \n  b: hsl(49.7972674498, 639.4765067331%, 2.0005058387%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), hwb)}\n"),
            "a {\
         \n  b: hsl(337.1245847104, 56.1707811732%, 12.7694440441%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), hwb)}\n"),
            "a {\
         \n  b: hsl(17.5913578322, 6051.6428880587%, 0.2688304082%);\
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
             \na {b: color.to-space(lab(50% -999999 0), hwb)}\n"),
            "a {\
         \n  b: hsl(329.5753543003, 419.147137703%, -324.1554346051%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), hwb)}\n"
        ),
        "a {\
         \n  b: hsl(289.8791142497, 47.7721599837%, -30.374201454%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), hwb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
