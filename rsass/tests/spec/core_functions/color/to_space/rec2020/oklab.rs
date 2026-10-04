//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(26.0943666826% -0.0561015249 -0.0661325197 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(26.0943666826% -0.0561015249 -0.0661325197 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), oklab)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(57.4349177499% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(48.8307497788% -0.1013198893 -0.2002702709);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none none none), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(none none none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(23.7844679127% -0.0891920746 0.0607987891);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(17.9852042001% 0.0052091913 -0.1239009702);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(25.3377299665% -0.0795437029 -0.0695312518);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), oklab)}\n"),
            "a {\
         \n  b: color-mix(in oklab, color(xyz -159996243905245.12 -65987151441146.23 -0.0234375) 100%, black);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(37.9216267643% -2.5061681132 -0.7189925745);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(100% 0 0);\
         \n}\n"
    );
}
