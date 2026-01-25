//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/oklch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.6425373683% 0.0652923458 235.0643410454deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.6425373683% 0.0652923458 235.0643410454deg / 0);\
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
             \na {b: color.to-space(color(display-p3-linear 0 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(79.3700525984% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(72.6776426801% 0.1150576748 251.0236635828deg);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(53.2843793949% 0.1642904233 128.5887993605deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(38.9886281594% 0.2117384656 306.0054451285deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(53.4259235831% 0.124581508 213.3257816885deg);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -486570.4620772681 -228974.3350951847 -0.0000000002) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(38.7754419937% 2.2262395216 190.6704180494deg);\
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
             \na {b: color.to-space(color(display-p3-linear 1 1 1), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(100% 0 none);\
         \n}\n"
    );
}
