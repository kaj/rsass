//! Tests auto-converted from "sass-spec/spec/css/media/logic/nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

mod interpolated {
    #[allow(unused)]
    use super::runner;

    mod and {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lowercase() {
            assert_eq!(
                runner().ok("@media (#{\"(a) and (b)\"}) {x {y: z}}\n"),
                "@media ((a) and (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media (#{\"(a) AnD (b)\"}) {x {y: z}}\n"),
                "@media ((a) AnD (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod not {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn lowercase() {
            assert_eq!(
                runner().ok("@media (#{\"not (a)\"}) {x {y: z}}\n"),
                "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media (#{\"NoT (a)\"}) {x {y: z}}\n"),
                "@media (NoT (a)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod or {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lowercase() {
            assert_eq!(
                runner().ok("@media (#{\"(a) or (b)\"}) {x {y: z}}\n"),
                "@media ((a) or (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media (#{\"(a) oR (b)\"}) {x {y: z}}\n"),
                "@media ((a) oR (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
}
mod raw {
    #[allow(unused)]
    use super::runner;

    mod and {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lowercase() {
            assert_eq!(
                runner().ok("@media ((a) and (b)) {x {y: z}}\n"),
                "@media ((a) and (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media ((a) AnD (b)) {x {y: z}}\n"),
                "@media ((a) and (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    #[test]
    fn different_than_top_level() {
        assert_eq!(
            runner().ok("@media (a) and ((b) or (c)) {x {y: z}}\n"),
            "@media (a) and ((b) or (c)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
        );
    }
    mod not {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn lowercase() {
            assert_eq!(
                runner().ok("@media (not (a)) {x {y: z}}\n"),
                "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media (NoT (a)) {x {y: z}}\n"),
                "@media not (a) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod or {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lowercase() {
            assert_eq!(
                runner().ok("@media ((a) or (b)) {x {y: z}}\n"),
                "@media ((a) or (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn mixed_case() {
            assert_eq!(
                runner().ok("@media ((a) Or (b)) {x {y: z}}\n"),
                "@media ((a) or (b)) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
            );
        }
    }
}
