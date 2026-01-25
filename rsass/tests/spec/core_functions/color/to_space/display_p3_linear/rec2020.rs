//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/rec2020.hrx"

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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3386744281 0.4295143351 0.5387140347 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3386744281 0.4295143351 0.5387140347 / 0);\
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
             \na {b: color.to-space(color(display-p3-linear 0 0 0), rec2020)}\n"
        ),
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
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.7054355531 0.7054355531 0.7054355531);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.5087906169 0.6251394888 0.8914365162);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3162343228 0.4249607248 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2720431653 none 0.5353168234);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 none 0.423943727 0.5388304424);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -485.0647425281 -137.3886086146 26.7185039398);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.7605208006 0.5913026458 1.394070004);\
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
             \na {b: color.to-space(color(display-p3-linear 1 1 1), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
