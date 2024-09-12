//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/rec2020.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.0023957954 0.2043088925 0.318295884 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.0023957954 0.2043088925 0.318295884 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.5277221397 0.5277221397 0.5277221397);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.1605692199 0.4295080587 0.8280670897);\
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.0713341527 0.2072064033 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.0119013078 none 0.3219111049);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 none 0.2069376666 0.3181540374);\
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
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -86467.5261196395 24057.1232097426 -7686.7078341848);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -1.3001703326 0.4795529651 1.8550030977);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
