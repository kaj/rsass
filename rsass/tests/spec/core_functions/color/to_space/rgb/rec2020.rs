//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/rec2020.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0214656524 0.0305541381 0.055318427 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0214656524 0.0305541381 0.055318427 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, rec2020)}\n"),
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
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2689744026 0.3372277303 0.7270775308);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.630170586 0.630170586 0.630170586);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.327926145 0.4744067013 0.818404902);\
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
             \na {b: color.to-space(rgb(10 20 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0189351452 0.0298903102 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0111000377 none 0.0525478256);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none 0.029610358 0.055094541);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -6394.9114299566 -2369.6504581671 -1240.1892051013);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3600781868 0.3645236247 1.5702758315);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
