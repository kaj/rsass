//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/semicolon.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("semicolon")
}

mod nested {
    use super::runner;

    mod interleaved {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn before_declaration() {
            assert_eq!(
                runner().ok("a {\
             \n  b {c: d}\
             \n  @e f;\
             \n  g: h\
             \n}\n"),
                "a b {\
         \n  c: d;\
         \n}\
         \na {\
         \n  @e f;\
         \n  g: h;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before_rule() {
            assert_eq!(
                runner().ok("a {\
             \n  b {c: d}\
             \n  @e f;\
             \n  g {h: i}\
             \n}\n"),
                "a b {\
         \n  c: d;\
         \n}\
         \na {\
         \n  @e f;\
         \n}\
         \na g {\
         \n  h: i;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn test_final() {
            assert_eq!(
                runner().ok("a {\
             \n  b {c: d}\
             \n  @e f;\
             \n}\n"),
                "a b {\
         \n  c: d;\
         \n}\
         \na {\
         \n  @e f;\
         \n}\n"
            );
        }
    }
    #[test]
    fn only() {
        assert_eq!(
            runner().ok("a {\
             \n  @b c;\
             \n}\n"),
            "a {\
         \n  @b c;\
         \n}\n"
        );
    }
}
