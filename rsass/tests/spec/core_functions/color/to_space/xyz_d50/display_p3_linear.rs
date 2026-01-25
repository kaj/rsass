//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -0.0768967659 0.280378289 0.3674437529 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -0.0768967659 0.280378289 0.3674437529 / 0);\
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
             \na {b: color.to-space(color(xyz-d50 0 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.5081450238 0.4863668124 0.612239994);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -0.2333162581 0.563965703 0.9896217796);\
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0423873237 0.2755646016 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.1211093226 none 0.3869207925);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear none 0.3646052907 0.3626243712);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -2403931.7179213753 842269.1738754527 -48193.7686703162);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -3.5951735625 1.5939444721 2.4601948425);\
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
             \na {b: color.to-space(color(xyz-d50 1 1 1), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1.0162900475 0.9727336247 1.224479988);\
         \n}\n"
    );
}
