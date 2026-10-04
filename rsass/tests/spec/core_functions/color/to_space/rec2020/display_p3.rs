//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/display_p3.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.0506451441 0.159102819 0.2627588541 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.0506451441 0.159102819 0.2627588541 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.4725 0.4725 0.4725);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.2180290019 0.368411688 0.7936119604);\
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
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.007498001 0.1614755472 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.0250001058 none 0.2637280821);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 none 0.1601647915 0.2627323564);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -1193149.6211313969 338422.1042057179 -91405.0539395979);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 -1.2607082796 0.3947763762 2.0688537186);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
