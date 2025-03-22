//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.2 0.4 0.8);\
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
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), rec2020)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.1 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), rec2020)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.1 none 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), rec2020)}\n"
            ),
            "a {\
         \n  b: color(rec2020 none 0.2 0.3);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 -1 0.4 2);\
         \n}\n"
    );
}
