//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/xyz.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1612558191 0.185031235 0.322205987 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1612558191 0.185031235 0.322205987 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0 0 0), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.4752279635 0.5 0.5445288754);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.3621550952 0.3859198528 0.8532008479);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1017906335 0.1612451608 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.1081222804 none 0.3131833107);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz none 0.1621337786 0.322205987);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz -486570.4620772676 -228974.3350951848 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.0161306991 0.2062946729 2.1059340905);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 1 1 1), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559271 1 1.0890577508);\
         \n}\n"
    );
}
