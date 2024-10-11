//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), oklab)}\n"),
            "a {\
         \n  b: oklab(56.4079112516% -0.173633896 -0.0375005538 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), oklab)}\n"),
            "a {\
         \n  b: oklab(56.4079112516% -0.173633896 -0.0375005538 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(79.6276380258% 0.0212390429 0.0117978875);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(70.8952950666% -0.2482574171 -0.0948731289);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(56.6867662885% -0.1591393729 0.1508075356);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(26.3423255262% 0.3682063802 -0.2704617674);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(52.6994311398% -0.4922232483 -0.0409679567);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -999998.9999999993 0 -0.0000000001) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(38.0019903498% -3.2429401776 -0.3314216643);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), oklab)}\n"),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 1 1 1) 100%, black);\
         \n}\n"
    );
}
