//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1747294041 0.5751337079 0.6581713933 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1747294041 0.5751337079 0.6581713933 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.7546018898 0.7421998405 0.8139518386);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 -0.1822145748 0.7692556516 0.993137764);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 none 0.6327456634 0.6559458093);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3732526973 none 0.6674970467);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3597521318 0.5716433477 none);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -389.3221679147 269.7239004363 -73.0228219197);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -1.4088593501 1.1392552927 1.4531839464);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 1 1 1), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1.0072726765 0.9907179269 1.0864953536);\
         \n}\n"
    );
}
