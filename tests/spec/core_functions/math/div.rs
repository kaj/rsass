//! Tests auto-converted from "sass-spec/spec/core_functions/math/div.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.div(6)}\n"
            ),
            "Error: Missing argument $number2.\
         \n  ,--> input.scss\
         \n2 | a {b: math.div(6)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function div($number1, $number2) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.div(6, 3, 1)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.div(6, 3, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function div($number1, $number2) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod non_numeric {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn denominator() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {\
             \n  $result: math.div(6, b);\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: 6/b;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn numerator() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {\
             \n  $result: math.div(b, 3);\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: b/3;\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod unit {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn compatible() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6in, 3px)}\n"),
            "a {\
         \n  b: 192;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn denominator() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6, 3px) * 1px}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn incompatible() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6in, 3s) * 1s}\n"),
            "a {\
         \n  b: 2in;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn numerator() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6px, 3)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn same() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6px, 3px)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6c, 3d) * 1d}\n"),
            "a {\
         \n  b: 2c;\
         \n}\n"
        );
    }
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6, 5)}\n"),
            "a {\
         \n  b: 1.2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn named() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div($number2: 3, $number1: 6)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whole() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(6, 3)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
}
