//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/oklch.hrx"

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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(34.6066204299% 0.1371016621 212.8665983723deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(34.6066204299% 0.1371016621 212.8665983723deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), oklch)}\n"),
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
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(65.9753951081% 0.0000000246 89.875562502deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(52.9739863521% 0.3041341331 219.4853767353deg);\
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(33.9153744238% 0.1484719929 141.0866609821deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(20.6577507967% 0.1790896327 256.2784427934deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(32.493750959% 0.2189851728 202.1257651897deg);\
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
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -47674467013.18753 -16929933315.11391 -247080732.777771) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(9.4597839538% 2.3640864612 199.9392877927deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), oklch)}\n"),
        "a {\
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
