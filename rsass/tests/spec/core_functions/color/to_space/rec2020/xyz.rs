//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/xyz.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.0149646311 0.0185893732 0.0595828421 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.0149646311 0.0185893732 0.0595828421 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), xyz)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.1800777243 0.1894645708 0.2063378593);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.1282769355 0.1154243414 0.6241614492);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0055744982 0.0152920747 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0119259086 none 0.0589929724);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz none 0.0175435449 0.0595828421);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -999999 0 0), xyz)}\n"),
            "a {\
         \n  b: color(xyz -159996243905245.38 -65987151441146.4 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.2704395563 0.1254882627 5.6030260582);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559271 1 1.0890577508);\
         \n}\n"
    );
}
