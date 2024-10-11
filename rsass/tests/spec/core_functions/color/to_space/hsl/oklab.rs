//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.4), oklab)}\n"),
            "a {\
         \n  b: oklab(40.6685298895% 0.0313239056 0.0191932087 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.0), oklab)}\n"),
            "a {\
         \n  b: oklab(40.6685298895% 0.0313239056 0.0191932087 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 0%), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20.123456789deg 30.987654321% 60.192837465%), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(68.4523450864% 0.0396042286 0.0440734549);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 50%), oklab)}\n"),
        "a {\
         \n  b: oklab(59.8180730527% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(80deg 30% 60%), oklab)}\n"),
        "a {\
         \n  b: oklab(75.0578300858% -0.0474046392 0.0717748182);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none 20% 30%), oklab)}\n"),
            "a {\
         \n  b: oklab(39.5815066771% 0.0409008824 0.0142214725);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), oklab)}\n"),
            "a {\
         \n  b: oklab(none 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), oklab)}\n"),
            "a {\
         \n  b: oklab(41.8389440171% 0 0);\
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
             \na {b: color.to-space(hsl(20deg 999999% 50%), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 136956388.39988744 59264689.52803929 -623200798.6169884) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -1.0161268876 -0.540961491 0.1515884565) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 100%), oklab)}\n"),
        "a {\
         \n  b: oklab(100% 0 0);\
         \n}\n"
    );
}
