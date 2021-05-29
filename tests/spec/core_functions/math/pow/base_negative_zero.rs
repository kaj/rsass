//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/base_negative_zero.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod fuzzy {
    #[allow(unused)]
    use super::runner;

    mod with_exponent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn decimal() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, 0.5)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn even_integer() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, 2)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, math.div(1, 0))}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn negative_decimal() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, -0.5)}\n"),
                "a {\
         \n  b: Infinity;\
         \n}\n"
            );
        }
        #[test]
        fn negative_even_integer() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, -2)}\n"),
                "a {\
         \n  b: Infinity;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, math.div(-1, 0))}\n"),
                "a {\
         \n  b: Infinity;\
         \n}\n"
            );
        }
        #[test]
        fn negative_odd_integer() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, -3)}\n"),
                "a {\
         \n  b: -Infinity;\
         \n}\n"
            );
        }
        #[test]
        fn odd_integer() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, 3)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.000000000001, 0)}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
    }
}
mod with_exponent {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 0.5)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn even_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 2)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn even_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 2.000000000001)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, math.div(1, 0))}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn negative_decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, -0.5)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn negative_even_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, -2)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn negative_even_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, -2.000000000001)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, math.div(-1, 0))}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn negative_odd_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, -3)}\n"),
            "a {\
         \n  b: -Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn negative_odd_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, -3.000000000001)}\n"),
            "a {\
         \n  b: -Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn odd_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 3)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn odd_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 3.000000000001)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 0)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn zero_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.0, 0.000000000001)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
