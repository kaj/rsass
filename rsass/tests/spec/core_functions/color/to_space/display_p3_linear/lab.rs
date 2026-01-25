//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/lab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), lab)}\n"
        ),
        "a {\
         \n  b: lab(49.8024654585% -10.5740937262 -19.7130066203 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), lab)}\n"
        ),
        "a {\
         \n  b: lab(49.8024654585% -10.5740937262 -19.7130066203 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0 0 0), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), lab)}\n"
        ),
        "a {\
         \n  b: lab(76.0692610142% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), lab)}\n"
        ),
        "a {\
         \n  b: lab(67.9018444833% -6.5044540097 -39.4812389788);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), lab)}\n"
        ),
        "a {\
         \n  b: lab(47.3091665403% -30.4535652285 66.0309604136);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), lab)}\n"
        ),
        "a {\
         \n  b: lab(24.9809266292% 57.2104120541 -60.9644297771);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), lab)}\n"
        ),
        "a {\
         \n  b: lab(46.7662011187% -31.3717136244 -24.9670815486);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz -486570.4620772619 -228974.3350951829 0.0000001214) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), lab)}\n"
        ),
        "a {\
         \n  b: lab(48.1139176418% -546.8109953702 -138.1499509257);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 1 1 1), lab)}\n"),
        "a {\
         \n  b: lab(100% 0 0);\
         \n}\n"
    );
}
