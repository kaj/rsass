//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/prophoto_rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0808023791 0.1159349209 0.1985402672 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0808023791 0.1159349209 0.1985402672 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), prophoto-rgb)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.396850263 0.396850263 0.396850263);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.263788643 0.2994628762 0.7313671022);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0487695227 0.1129520818 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0764391224 none 0.196989411);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb none 0.1152634537 0.1985591483);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -999999 0 0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -90478633.22359292 -19766156.402933683 3456708.9925307552);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb -0.4288132547 0.3340737633 2.4736091068);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
