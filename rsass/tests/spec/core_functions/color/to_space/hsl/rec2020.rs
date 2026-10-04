//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.3614834303 0.3062960515 0.2837490465 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.3614834303 0.3062960515 0.2837490465 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 0%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20.123456789deg 30.987654321% 60.192837465%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.6849521761 0.5960178428 0.5183511499);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 50%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5260663507 0.5260663507 0.5260663507);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(80deg 30% 60%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.6797649811 0.7276359331 0.5356643506);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none 20% 30%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.3568764299 0.2895833506 0.28202887);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none none 50%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.5260663507 0.5260663507 0.5260663507);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.336492891 0.336492891 0.336492891);\
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
             \na {b: color.to-space(hsl(20deg 999999% 50%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 3724.1178064691 -635.2112086133 -4504.7703874825);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -1.2132603476 -0.5056330272 0.4511345945);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 100%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
