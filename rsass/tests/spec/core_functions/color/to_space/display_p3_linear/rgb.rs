//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/rgb.hrx"

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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(78.6613223953, 124.7530916647, 151.5148395665, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(78.6613223953, 124.7530916647, 151.5148395665, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0 0 0), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(187.5160306784, 187.5160306784, 187.5160306784);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(109.6963970696, 171.2215276397, 236.5503345783);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(78.6613223953, 124.7530916647, 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(98.1356471296, 0, 154.9441716243);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(0, 125.9369265301, 151.9484693152);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(356.7844906486, 165.0029138842%, -13697.7855994258%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(196.5412734015, 909.4415388743%, 13.9290938037%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 1 1 1), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
