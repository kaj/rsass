//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3/prophoto_rgb.hrx"

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
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1280122112 0.1488387497 0.2305514968 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1280122112 0.1488387497 0.2305514968 / 0);\
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
             \na {b: color.to-space(color(display-p3 0 0 0), prophoto-rgb)}\n"
        ),
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
             \na {b: color.to-space(color(display-p3 0.5 0.5 0.5), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.4246723949 0.4246723949 0.4246723949);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.2 0.4 0.8), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.3388390747 0.3342960169 0.7393352444);\
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
             \na {b: color.to-space(color(display-p3 0.1 0.2 none), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0911473149 0.1429690366 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 none 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1061223268 none 0.2275163591);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 none 0.2 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb none 0.1466998342 0.2305744173);\
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
             \na {b: color.to-space(color(display-p3 -999999 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -72137964.95638128 -23392436.47544621 2293597.437985952);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -1 0.4 2), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.363190311 0.3948251642 2.3672559908);\
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
             \na {b: color.to-space(color(display-p3 1 1 1), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
