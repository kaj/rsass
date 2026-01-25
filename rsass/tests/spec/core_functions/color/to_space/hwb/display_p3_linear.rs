//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/display_p3_linear.hrx"

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
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.3800388838 0.0779606611 0.0425197501 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.3800388838 0.0779606611 0.0425197501 / 0);\
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
             \na {b: color.to-space(hwb(0deg 0% 100%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2845556788 0.1434294973 0.0865826955);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2140411405 0.2140411405 0.2140411405);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.208277835 0.3140963727 0.0563558394);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.8385898273 0.1210204922 0.0538018878);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.3743307867 0.0468764968 0.0401920707);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.3707271247 0.0272522995 0.0085799998);\
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
             \na {b: color.to-space(hwb(20deg 999999% -999950%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 3501431641.933304 3501206149.469329 3501071484.0647173);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 6.3343931254 5.4854356337 5.0165921388);\
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
             \na {b: color.to-space(hwb(0deg 100% 0%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
