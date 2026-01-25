//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/srgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.3084757741 0.4892278104 0.5941758414 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.3084757741 0.4892278104 0.5941758414 / 0);\
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
             \na {b: color.to-space(color(display-p3-linear 0 0 0), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.7353569831 0.7353569831 0.7353569831);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.4301819493 0.6714569711 0.9276483709);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.3084757741 0.4892278104 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.384845675 none 0.6076242024);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb none 0.4938703001 0.5958763503);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -362.9953097609 89.0395977724 64.8142232337);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -1.1274787124 0.7075944843 1.4060605884);\
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
             \na {b: color.to-space(color(display-p3-linear 1 1 1), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 1 1 1);\
         \n}\n"
    );
}
