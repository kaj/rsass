//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/prophoto_rgb.hrx"

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
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2194696613 0.4617099795 0.489000517 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2194696613 0.4617099795 0.489000517 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), prophoto-rgb)}\n"),
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
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.7095939166 0.6699021515 0.6489253277);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2936039607 0.6811658686 0.8421802859);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2715690946 0.4564221473 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2931451274 none 0.4853966387);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb none 0.5136422936 0.49010255);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -2600.5460588084 1508.1151466205 177.670007881);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb -1.340921335 1.0822677288 1.4036262654);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 1.0429146548 0.9845782985 0.953747937);\
         \n}\n"
    );
}
