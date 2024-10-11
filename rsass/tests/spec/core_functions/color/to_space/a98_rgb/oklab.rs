//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/oklab.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(29.1825436037% -0.0383089975 -0.0634618946 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(29.1825436037% -0.0383089975 -0.0634618946 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), oklab)}\n"),
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
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(60.1621105108% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(52.1495259302% -0.0513578739 -0.1793591975);\
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(26.2341471549% -0.0807607264 0.0595439714);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(20.9471844641% 0.0293694097 -0.1198702912);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(28.2216459147% -0.0592406635 -0.0680291697);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -9041452038524.754 -4661998707364.315 -423818064305.8462) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(38.5831855711% -2.2710958174 -0.6509941571);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(100% 0 0);\
         \n}\n"
    );
}
