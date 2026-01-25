//! Tests auto-converted from "sass-spec/spec/expressions/if/short_circuit.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("short_circuit")
}

mod clause {
    use super::runner;

    mod and {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn css() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(false) and css(#{$undefined}): c)}\n"),
                ""
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp() {
            assert_eq!(
                runner().ok("a {b: if(sass(false) and #{$undefined}: c)}\n"),
                ""
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(false) and sass($undefined): c)}\n"),
                ""
            );
        }
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn css() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) or css(#{$undefined}): c)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn interp() {
            assert_eq!(
                runner().ok("a {b: if(sass(true) or #{$undefined}: c)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sass() {
            assert_eq!(
                runner().ok("a {b: if(sass(true) or sass($undefined): c)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod root {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn css() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true): c; css(#{$undefined}): d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp() {
            assert_eq!(
                runner().ok("a {b: if(sass(true): c; #{$undefined}: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true): c; sass($undefined): d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
}
mod value {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn after_matched() {
        assert_eq!(
            runner().ok("a {b: if(sass(true): c; else: $undefined)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unmatched() {
        assert_eq!(
            runner().ok("a {b: if(sass(false): $undefined; else: c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
