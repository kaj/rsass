//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/oklch.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.8480554239% 0.055496412 240.7509124531deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(56.8480554239% 0.055496412 240.7509124531deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(79.3700520804% 0.0000000296 89.8755628773deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(72.8143374355% 0.1028615689 255.885778774deg);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(53.8237207745% 0.138073183 126.5927383995deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(37.7328569492% 0.1982385426 302.8924282818deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(54.0246595647% 0.0975727429 218.8047277574deg);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -412390.3868751598 -212638.7932325044 -19330.7993847733) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(41.5806830952% 2.0279777823 190.8487076701deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), oklch)}\n"),
        "a {\
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
