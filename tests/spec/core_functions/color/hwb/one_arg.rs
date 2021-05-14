//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    mod clamped {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40% / 1.1)}\n"),
                "a {\
         \n  b: #994d4d;\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0 30% 40% / -0.1)}\n"),
                "a {\
         \n  b: rgba(153, 77, 77, 0);\
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
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\n"),
                "a {\
         \n  b: rgba(77, 153, 153, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn opaque() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(180 30% 40% / 1)}\n"),
                "a {\
         \n  b: #4d9999;\
         \n}\n"
            );
        }
        #[test]
        fn parenthesized() {
            assert_eq!(
        runner().ok(
            "@use \'sass:color\';\n\
             \n// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: (color.hwb(180 30% 40% / 0.4))}\n"
        ),
        "a {\
         \n  b: rgba(77, 153, 153, 0.4);\
         \n}\n"
    );
        }
        #[test]
        fn partial() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(180 30% 40% / 0.5)}\n"),
                "a {\
         \n  b: rgba(77, 153, 153, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(180 30% 40% / 0)}\n"),
                "a {\
         \n  b: rgba(77, 153, 153, 0);\
         \n}\n"
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\n"),
        "a {\
         \n  b: rgba(77, 153, 153, 0.4);\
         \n}\n"
    );
}
#[test]
fn no_alpha() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(180 30% 40%)}\n"),
        "a {\
         \n  b: #4d9999;\
         \n}\n"
    );
}
