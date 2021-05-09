//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/one_arg/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
                runner().ok("a {b: rgba(0 0 0 / 1.1)}\n"),
                "a {\
         \n  b: black;\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: rgba(0 0 0 / -0.1)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {b: rgba(0 0 9999 / 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 255, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("a {b: rgba(0 -1 0 / 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {b: rgba(256 0 0 / 0.5)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
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
            runner().ok("a {b: rgba($channels: 0 255 127 / 0.3)}\n"),
            "a {\
         \n  b: rgba(0, 255, 127, 0.3);\
         \n}\n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgba(190 173 237 / 1)}\n"),
            "a {\
         \n  b: #beaded;\
         \n}\n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
        runner().ok(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: rgba(0 255 127 / 0.3)}\n"
        ),
        "a {\
         \n  b: rgba(0, 255, 127, 0.3);\
         \n}\n"
    );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: rgba(18 52 86 / 0.5)}\n"),
            "a {\
         \n  b: rgba(18, 52, 86, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("a {b: rgba(18 52 86 / 50%)}\n"),
            "a {\
         \n  b: rgba(18, 52, 86, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgba(0 255 127 / 0)}\n"),
            "a {\
         \n  b: rgba(0, 255, 127, 0);\
         \n}\n"
        );
    }
}
