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
         \n  b: color(xyz 0.0096949161 0.0016890376 -0.046376448 / 0.4);\
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
         \n  b: color(xyz 0.0096949161 0.0016890376 -0.046376448 / 0);\
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
         \n  b: color(xyz 0.1188069941 0.1250000023 0.13613225);\
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
         \n  b: color(xyz 0.2832157077 0.0847349623 0.9808955517);\
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
         \n  b: color(xyz -0.0014924289 0.0021094378 -0.0382539306);\
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
         \n  b: color(xyz 0.0069500554 0.0002726166 0.000231366);\
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
         \n  b: color(xyz -0.0150397763 0.004582061 -0.1058878063);\
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
         \n  b: color(xyz -76837326254677680 3783159310641777.5 5396110649242756);\
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
         \n  b: color(xyz -7.6342507319 1.7017043263 -38.7847424885);\
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
         \n  b: color(xyz 0.9504559526 1.0000000182 1.0890580001);\
         \n}\n"
    );
}
