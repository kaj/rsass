//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/xyz.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.009694915 0.0016890377 -0.046376451 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.009694915 0.0016890377 -0.046376451 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.1188069909 0.125 0.1361322188);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.2832156909 0.0847349662 0.9808954672);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz -0.0014924294 0.0021094378 -0.0382539338);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.006950055 0.0002726167 0.000231366);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz -0.0150397765 0.0045820606 -0.1058878076);\
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
             \na {b: color.to-space(oklab(50% -999999 0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz -76837317949857248 3783158056963297.5 5396109066377534);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz -7.6342505681 1.7017041167 -38.7847424763);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559271 1 1.0890577508);\
         \n}\n"
    );
}
