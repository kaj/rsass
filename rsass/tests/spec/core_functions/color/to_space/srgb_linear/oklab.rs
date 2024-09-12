//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/oklab.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.8480554239% -0.0271159546 -0.0484208297 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.8480554239% -0.0271159546 -0.0484208297 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(79.3700520804% 0.0000000001 0.0000000296);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(72.8143374355% -0.0250833833 -0.0997563344);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(53.8237207745% -0.0823086169 0.1108579968);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(37.7328569492% 0.1076561145 -0.1664592467);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(54.0246595647% -0.0760370977 -0.061145727);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -412390.3868751598 -212638.7932325045 -19330.799384773) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(41.5806830952% -1.9917329546 -0.3816984711);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(99.9999993474% 0.0000000001 0.0000000373);\
         \n}\n"
    );
}
