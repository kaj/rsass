//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod clamped {
    #[allow(unused)]
    use super::runner;

    mod alpha {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hsl(0 100% 50% / 1.1)}\n"),
                "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsl(0 100% 50% / -0.1)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn lightness() {
        assert_eq!(
            runner().ok("a {b: hsl(0 100% 9999% / 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 100%, 9999%, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            runner().ok("a {b: hsl(0 -0.1% 50% / 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 0%, 50%, 0.5);\
         \n}\n"
        );
    }
}
mod in_gamut {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: hsl($channels: 180 60% 50% / 0.4)}\n"),
            "a {\
         \n  b: hsla(180, 60%, 50%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: hsl(180 60% 50% / 1)}\n"),
            "a {\
         \n  b: hsl(180, 60%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
        runner().ok(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: (hsl(180 60% 50% / 0.4))}\n"
        ),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: hsl(180 60% 50% / 0.5)}\n"),
            "a {\
         \n  b: hsla(180, 60%, 50%, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: hsl(180 60% 50% / 0)}\n"),
            "a {\
         \n  b: hsla(180, 60%, 50%, 0);\
         \n}\n"
        );
    }
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn slash() {
        assert_eq!(
            runner().ok("a {b: hsl(180 60% 50% / none)}\n"),
            "a {\
         \n  b: hsl(180deg 60% 50% / none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn slash_list() {
        assert_eq!(
            runner().ok("@use \'sass:list\';\
             \na {b: hsl(list.slash(180 60% 50%, none))}\n"),
            "a {\
         \n  b: hsl(180deg 60% 50% / none);\
         \n}\n"
        );
    }
}
#[test]
fn slash_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: hsl(list.slash(180 60% 50%, 0))}\n"),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0);\
         \n}\n"
    );
}
