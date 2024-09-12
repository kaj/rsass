//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), lab)}\n"),
            "a {\
         \n  b: lab(45.4228859316% 43.1656753022 35.3539928676 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), lab)}\n"),
            "a {\
         \n  b: lab(45.4228859316% 43.1656753022 35.3539928676 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 0% 100%), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), lab)}\n"
        ),
        "a {\
         \n  b: lab(48.7209664558% 18.0179342003 22.2207931535);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), lab)}\n"),
        "a {\
         \n  b: lab(53.3889647411% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), lab)}\n"),
        "a {\
         \n  b: lab(59.1049141497% -23.929516442 47.3632808755);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% none), lab)}\n"),
            "a {\
         \n  b: lab(60.7483623827% 64.2863186591 55.7098309346);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), lab)}\n"),
            "a {\
         \n  b: lab(42.0638915398% 51.8940588399 31.6507145358);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), lab)}\n"),
            "a {\
         \n  b: lab(39.3870160342% 57.3703400949 53.7905111747);\
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
             \na {b: color.to-space(hwb(20deg 999999% -999950%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 3327825161.664072 3501247104.3035965 3812875110.896886) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 5.5338099778 5.6426521513 5.4845096668) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 100% 0%), lab)}\n"),
        "a {\
         \n  b: lab(100% 0 0);\
         \n}\n"
    );
}
