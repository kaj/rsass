//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.4), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(15.1358007651, 51.7877986787, 78.726254489, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.0), rgb)}\n"
        ),
        "a {\
         \n  b: rgba(15.1358007651, 51.7877986787, 78.726254489, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0 0 0), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.5 0.5 0.5), rgb)}\n"),
        "a {\
         \n  b: rgb(127.5, 127.5, 127.5);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.2 0.4 0.8), rgb)}\n"),
        "a {\
         \n  b: rgb(26.5344066192, 103.5127809071, 211.0954947112);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 none), rgb)}\n"),
            "a {\
         \n  b: rgb(15.1358007651, 51.7877986787, 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 none 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(28.9866805453, 0, 80.0096532626);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 none 0.2 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(0, 52.1258051255, 78.8241584227);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -999999 0 0), rgb)}\n"),
            "a {\
         \n  b: hsl(356.7852726724, 165.043052964%, -41057989.10847678%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -1 0.4 2), rgb)}\n"),
            "a {\
         \n  b: hsl(210.5153182162, 325.7395089334%, 48.9349840904%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 1 1 1), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
