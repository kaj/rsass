//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/prophoto_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1084709661 0.1348055136 0.2212739372 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1084709661 0.1348055136 0.2212739372 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.4287509597 0.4287509597 0.4287509597);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3168388393 0.3260606848 0.726820787);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0682396395 0.1289630742 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0971986374 none 0.2172679015);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb none 0.1323881901 0.2209968271);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -18118318.905084856 -7113714.776888951 -2671576.7208059593);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -0.1930208925 0.2909921381 2.199240452);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 1 1 1), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
