//! Tests auto-converted from "sass-spec/spec/values/numbers/bounds.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod int {
    use super::runner;

    mod above_max {
        use super::runner;

        #[test]
        fn slightly() {
            assert_eq!(
        runner().ok(
            "a {b: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001}\n"
        ),
        "a {\
         \n  b: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;\
         \n}\n"
    );
        }
        #[test]
        fn very() {
            assert_eq!(
        runner().ok(
            "a {\
             \n  b: 999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999;\
             \n}\n"
        ),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
        }
    }
    mod below_min {
        use super::runner;

        #[test]
        fn slightly() {
            assert_eq!(
        runner().ok(
            "a {b: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001}\n"
        ),
        "a {\
         \n  b: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;\
         \n}\n"
    );
        }
        #[test]
        fn very() {
            assert_eq!(
        runner().ok(
            "a {\
             \n  b: -999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999;\
             \n}\n"
        ),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
        }
    }
    #[test]
    fn max_value() {
        assert_eq!(
        runner().ok(
            "a {b: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000}\n"
        ),
        "a {\
         \n  b: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;\
         \n}\n"
    );
    }
    #[test]
    fn min_value() {
        assert_eq!(
        runner().ok(
            "a {b: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000}\n"
        ),
        "a {\
         \n  b: -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;\
         \n}\n"
    );
    }
    mod safe {
        use super::runner;

        #[test]
        fn above_max() {
            assert_eq!(
                runner().ok("a {\
             \n  b: 9007199254740993;\
             \n}\n"),
                "a {\
         \n  b: 9007199254740992;\
         \n}\n"
            );
        }
        #[test]
        fn below_min() {
            assert_eq!(
                runner().ok("a {b: -9007199254740993}\n"),
                "a {\
         \n  b: -9007199254740992;\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("a {b: 9007199254740991}\n"),
                "a {\
         \n  b: 9007199254740991;\
         \n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                runner().ok("a {\
             \n  b: -9007199254740991;\
             \n}\n"),
                "a {\
         \n  b: -9007199254740991;\
         \n}\n"
            );
        }
    }
}
mod precision_limit {
    use super::runner;

    mod at {
        use super::runner;

        #[test]
        fn after_decimal() {
            assert_eq!(
        runner().ok(
            "// binary 0.10000000000000000000000000000000000000000000000000001\
             \n// 0.5 + 2^-53\
             \n// Multiplied by 2^53 so Sass\'s serialization doesn\'t remove the precision\
             \n@use \'sass:math\';\
             \na {b: 0.5000000000000001 * math.pow(2, 53)}\n"
        ),
        "a {\
         \n  b: 4503599627370497;\
         \n}\n"
    );
        }
        #[test]
        fn balanced() {
            assert_eq!(
        runner().ok(
            "// binary 10000000000000000000000000.000000000000000000000000001\
             \n// 2^26 + 2^-26\
             \na {b: 67108864.00000001}\n"
        ),
        "a {\
         \n  b: 67108864.00000001;\
         \n}\n"
    );
        }
        #[test]
        fn no_decimal() {
            assert_eq!(
        runner().ok(
            "// binary 10000000000000000000000000000000000000000000000000001\
             \n// 2^52 + 1\
             \na {b: 4503599627370497}\n"
        ),
        "a {\
         \n  b: 4503599627370497;\
         \n}\n"
    );
        }
    }
    mod over {
        use super::runner;

        #[test]
        fn after_decimal() {
            assert_eq!(
        runner().ok(
            "// 0b0.100000000000000000000000000000000000000000000000000001\
             \n// 0.5 + 2^-54\
             \n// Multiplied by 2^53 so Sass\'s serialization doesn\'t remove the precision\
             \n@use \'sass:math\';\
             \na {b: 0.50000000000000005 * math.pow(2, 54)}\n"
        ),
        "a {\
         \n  b: 9007199254740992;\
         \n}\n"
    );
        }
        #[test]
        fn balanced() {
            assert_eq!(
        runner().ok(
            "// 0b10000000000000000000000000.0000000000000000000000000001\
             \n// 2^26 + 2^-27\
             \na {b: 67108864.000000007}\n"
        ),
        "a {\
         \n  b: 67108864;\
         \n}\n"
    );
        }
        #[test]
        fn no_decimal() {
            assert_eq!(
        runner().ok(
            "// binary 100000000000000000000000000000000000000000000000000001\
             \n// 2^53 + 1\
             \na {b: 9007199254740993}\n"
        ),
        "a {\
         \n  b: 9007199254740992;\
         \n}\n"
    );
        }
    }
}
