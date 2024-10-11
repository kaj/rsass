//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), oklch)}\n"),
            "a {\
         \n  b: oklch(56.4079112516% 0.177637331 192.1872561651deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), oklch)}\n"),
            "a {\
         \n  b: oklch(56.4079112516% 0.177637331 192.1872561651deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), oklch)}\n"),
        "a {\
         \n  b: oklch(79.6276380258% 0.0242958246 29.0513452444deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), oklch)}\n"),
        "a {\
         \n  b: oklch(70.8952950666% 0.2657680487 200.914669995deg);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), oklch)}\n"),
            "a {\
         \n  b: oklch(56.6867662885% 0.2192447326 136.5398255702deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(26.3423255262% 0.4568648662 323.7012844561deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(52.6994311398% 0.4939251964 184.7577868469deg);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -999998.9999999988 0 -0.0000000009) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(38.0019903498% 3.2598314857 185.835249167deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), oklch)}\n"),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 1 1 1) 100%, black);\
         \n}\n"
    );
}
