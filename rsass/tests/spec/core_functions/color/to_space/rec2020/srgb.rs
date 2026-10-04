//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/srgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -0.0984745484 0.1634805153 0.2717406985 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -0.0984745484 0.1634805153 0.2717406985 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.4725 0.4725 0.4725);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), srgb)}\n"),
        "a {\
         \n  b: color(srgb -0.299361812 0.3782591938 0.8228664179);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), srgb)}\n"),
            "a {\
         \n  b: color(srgb -0.067849869 0.1653196347 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.0330742965 none 0.2764862586);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb none 0.1654440356 0.2719045508);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), srgb)}\n"),
            "a {\
         \n  b: color(srgb -1303218.4699719346 442907.0049856773 198514.1389888964);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), srgb)}\n"),
            "a {\
         \n  b: color(srgb -1.3850726368 0.4913523661 2.1570700889);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), srgb)}\n"),
        "a {\
         \n  b: color(srgb 1 1 1);\
         \n}\n"
    );
}
