//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(34.9190212628%, 48.4529204482%, 58.383149006%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(34.9190212628%, 48.4529204482%, 58.383149006%, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), rgb)}\n"),
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
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), rgb)}\n"),
        "a {\
         \n  b: rgb(73.5356983052%, 73.5356983052%, 73.5356983052%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), rgb)}\n"),
        "a {\
         \n  b: rgb(48.4529204482%, 66.5185084631%, 90.6331753344%);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(34.9190212628%, 48.4529204482%, 0%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(34.9190212628%, 0%, 58.383149006%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(0%, 48.4529204482%, 58.383149006%);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), rgb)}\n"),
            "a {\
         \n  b: hsl(0, 100%, -16678.2577069634%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), rgb)}\n"),
            "a {\
         \n  b: hsl(197.5434618594, 666.1615765111%, 17.6628023075%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
