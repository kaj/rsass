//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/hwb.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), hwb)}\n"),
            "a {\
         \n  b: hsla(19.0047477469, 6337.718112733%, 0.3924703093%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), hwb)}\n"),
            "a {\
         \n  b: hsla(19.0047477469, 6337.718112733%, 0.3924703093%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), hwb)}\n"),
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
             \na {b: color.to-space(oklab(50% 0 0), hwb)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 38.8572859046%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), hwb)}\n"),
        "a {\
         \n  b: hsl(280.3037190117, 185.1123146142%, 35.6118905914%);\
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
             \na {b: color.to-space(oklab(10% none 0.3), hwb)}\n"),
            "a {\
         \n  b: hsl(229.544295526, 280.5312491892%, -5.8764824076%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(339.4567051743, 263.6331125505%, 4.4011033198%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), hwb)}\n"),
            "a {\
         \n  b: hsl(261.4365776935, 230.0242829233%, -11.2533873127%);\
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
             \na {b: color.to-space(oklab(50% -999999 0), hwb)}\n"),
            "a {\
         \n  b: hsl(340.1123874029, 426.4426843996%, -360093996.6269262%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), hwb)}\n"
        ),
        "a {\
         \n  b: hsl(280.1786400518, 318.2272010349%, -118.1342753197%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), hwb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
