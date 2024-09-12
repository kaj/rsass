//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/xyz_d50.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0125260886 0.0027519422 -0.0349334402 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0125260886 0.0027519422 -0.0349334402 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1205369614 0.1250000018 0.1031380988);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.2495012093 0.0755678711 0.7361680649);\
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
             \na {b: color.to-space(oklab(10% none 0.3), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0004044958 0.0026981824 -0.028716594);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.007277813 0.0004719735 0.0001138228);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 -0.0103407367 0.0059005425 -0.0794063212);\
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
             \na {b: color.to-space(oklab(50% -999999 0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 -80704154717242816 1378317505528979.2 4824363534483794);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 -6.0144158738 2.1214433281 -29.0650672146);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.9642956911 1.0000000145 0.82510479);\
         \n}\n"
    );
}
