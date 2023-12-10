//! Tests auto-converted from "sass-spec/spec/values/calculation/mod.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mod")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: MoD(7, 3)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn equals() {
    assert_eq!(
        runner().ok("a {b: mod(1, 1)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn dividend_type() {
        assert_eq!(
            runner().err("a {b: mod(\"0\", 0)}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: mod(\"0\", 0)}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn modulus_type() {
        assert_eq!(
            runner().err("a {b: mod(0, \"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: mod(0, \"0\")}\
         \n  |              ^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: mod(7 % 3, 1)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: mod(7 % 3, 1)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: mod(10, $)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: mod(10, $)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: mod(3)}\n"),
            "Error: 2 arguments required, but only 1 was passed.\
         \n  ,\
         \n1 | a {b: mod(3)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: mod(3, 2, 1)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,\
         \n1 | a {b: mod(3, 2, 1)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn complex_and_unknown() {
            assert_eq!(
        runner().err(
            "a {b: mod(1px*2px, 10%)}\n"
        ),
        "Error: Number calc(2px * 1px) isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: mod(1px*2px, 10%)}\
         \n  |           ^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn incompatible() {
            assert_eq!(
                runner().err("a {b: mod(16px, 5deg)}\n"),
                "Error: 16px and 5deg are incompatible.\
         \n  ,\
         \n1 | a {b: mod(16px, 5deg)}\
         \n  |           ^^^^ 16px\
         \n  |                 ==== 5deg\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn real_and_unitless() {
            assert_eq!(
                runner().err("a {b: mod(16px, 5)}\n"),
                "Error: 16px and 5 are incompatible.\
         \n  ,\
         \n1 | a {b: mod(16px, 5)}\
         \n  |           ^^^^ 16px\
         \n  |                 = 5\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
    }
}
mod nan {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn negative_and_positive_infinity() {
        assert_eq!(
            runner().ok("a {b: mod(-5, infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn negative_zero_and_positive_infinity() {
        assert_eq!(
            runner().ok("a {b: mod(-0, infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn positive_and_negative_infinity() {
        assert_eq!(
            runner().ok("a {b: mod(5, -infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn zero_and_negative_infinity() {
        assert_eq!(
            runner().ok("a {b: mod(0, -infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: mod(-2, -5)}\n"),
        "a {\
         \n  b: -2;\
         \n}\n"
    );
}
#[test]
fn negative_and_positive() {
    assert_eq!(
        runner().ok("a {b: mod(-2, 5)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, mod(-7, 7))}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function mod($arg) {@return $arg}\
             \na {b: mod(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn positive_and_negative() {
    assert_eq!(
        runner().ok("a {b: mod(2, -5)}\n"),
        "a {\
         \n  b: -3;\
         \n}\n"
    );
}
#[test]
fn positive_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, mod(7, 7))}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: mod(3px - 1px + var(--c), -7px / 4em * 1em);\
             \n}\n"),
        "a {\
         \n  b: mod(2px + var(--c), -1.75px);\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("a {b: mod(5px, 3px)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: mod(1foo, 2bar);\
             \n}\n"),
            "a {\
         \n  b: mod(1foo, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("a {b: mod(7, 3)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn real_and_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: mod(1px, 2bar);\
             \n}\n"),
            "a {\
         \n  b: mod(1px, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn real_and_unknown() {
        assert_eq!(
            runner().ok("a {b: mod(5px, 3%)}\n"),
            "a {\
         \n  b: mod(5px, 3%);\
         \n}\n"
        );
    }
    #[test]
    fn same_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: mod(1foo, 2foo);\
             \n}\n"),
            "a {\
         \n  b: 1foo;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {\
             \n  b: mod(1%, 2%);\
             \n}\n"),
            "a {\
         \n  b: 1%;\
         \n}\n"
        );
    }
}
mod x_infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: mod(10, -infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: mod(-10, infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
#[test]
fn x_zero() {
    assert_eq!(
        runner().ok("a {b: mod(0, 6)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
mod y_infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: mod(infinity, 10)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
#[test]
fn y_zero() {
    assert_eq!(
        runner().ok("a {b: mod(6, 0)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn zeros() {
    assert_eq!(
        runner().ok("a {b: mod(0, 0)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
