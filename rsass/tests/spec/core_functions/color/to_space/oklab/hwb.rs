//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), hwb)}\n"),
            "a {\
         \n  b: hsla(19.0047457513, 6337.7067481883%, 0.3924710109%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), hwb)}\n"),
            "a {\
         \n  b: hsla(19.0047457513, 6337.7067481883%, 0.3924710109%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), hwb)}\n"),
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
             \na {b: color.to-space(oklab(50% 0 0), hwb)}\n"),
        "a {\
         \n  b: hsl(223.8135972091, 0.0000078676%, 38.8572876766%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), hwb)}\n"),
        "a {\
         \n  b: hsl(280.3037191595, 185.1123260276%, 35.6118905417%);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), hwb)}\n"),
            "a {\
         \n  b: hsl(229.5442945473, 280.5312633728%, -5.8764819359%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), hwb)}\n"),
            "a {\
         \n  b: hsl(339.4567046558, 263.6331199652%, 4.40110336%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), hwb)}\n"),
            "a {\
         \n  b: hsl(261.4365788529, 230.0242883505%, -11.2533870657%);\
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
             \na {b: color.to-space(oklab(50% -999999 0), hwb)}\n"),
            "a {\
         \n  b: hsl(340.1123890362, 426.4426890198%, -360094010.73043364%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), hwb)}\n"
        ),
        "a {\
         \n  b: hsl(280.1786410944, 318.2272126608%, -118.134272113%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), hwb)}\n"),
        "a {\
         \n  b: hsl(43.8135971652, 172.5242119439%, 100.0000042145%);\
         \n}\n"
    );
}
