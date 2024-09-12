//! Tests auto-converted from "sass-spec/spec/directives/for.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod after_from {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for $i from /**/ 1 through 10 {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for $i from //\
             \n  1 through 10 {}\n"),
                ""
            );
        }
    }
    mod after_through {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for $i from 1 through /**/ 10 {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for $i from 1 through //\
             \n  10 {}\n"),
                ""
            );
        }
    }
    mod before_block {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for $i from 1 through 10 /**/ {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for $i from 1 through 10 //\
             \n  {}\n"),
                ""
            );
        }
    }
    mod before_from {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for $i /**/ from 1 through 10 {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for $i //\
             \n  from 1 through 10 {}\n"),
                ""
            );
        }
    }
    mod before_through {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for $i from 1 /**/ through 10 {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for $i from 1 //\
             \n  through 10 {}\n"),
                ""
            );
        }
    }
    mod before_var {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn loud() {
            assert_eq!(
                runner().ok("@for /**/ $i from 1 through 10 {}\n"),
                ""
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@for //\
             \n  $i from 1 through 10 {}\n"),
                ""
            );
        }
    }
}
#[test]
fn empty() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 1 to 1 {b: $i;}\
             \n}\n"),
        ""
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn from_float() {
        assert_eq!(
            runner().err("@for $i from 1.5 through 4 {}"),
            "Error: 1.5 is not an int.\
         \n  ,\
         \n1 | @for $i from 1.5 through 4 {}\
         \n  |              ^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn from_type() {
        assert_eq!(
            runner().err("@for $i from \"foo\" through 4 {}"),
            "Error: \"foo\" is not a number.\
         \n  ,\
         \n1 | @for $i from \"foo\" through 4 {}\
         \n  |              ^^^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn incompatible_units() {
        assert_eq!(
            runner().err("@for $i from 100% through 42px {}"),
            "Error: Expected 42px to have unit %.\
         \n  ,\
         \n1 | @for $i from 100% through 42px {}\
         \n  |                           ^^^^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn to_float() {
        assert_eq!(
            runner().err("@for $i from 1 through 1.5 {}"),
            "Error: 1.5 is not an int.\
         \n  ,\
         \n1 | @for $i from 1 through 1.5 {}\
         \n  |                        ^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn to_type() {
        assert_eq!(
            runner().err("@for $i from 1 through \"foo\" {}"),
            "Error: \"foo\" is not a number.\
         \n  ,\
         \n1 | @for $i from 1 through \"foo\" {}\
         \n  |                        ^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn unit_coersion_to_float() {
        assert_eq!(
            runner().err("@for $i from 1cm through 5mm {}"),
            "Error: 0.5cm is not an int.\
         \n  ,\
         \n1 | @for $i from 1cm through 5mm {}\
         \n  |                          ^^^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
        );
    }
}
#[test]
fn exclusive_backward() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 5 to 1 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: 5;\
         \n  b: 4;\
         \n  b: 3;\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn exclusive_forward() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 1 to 5 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: 1;\
         \n  b: 2;\
         \n  b: 3;\
         \n  b: 4;\
         \n}\n"
    );
}
#[test]
fn in_declaration() {
    assert_eq!(
        runner().ok("a {\
             \n  b: {\
             \n    @for $i from 1 through 5 {c: $i}\
             \n  }\
             \n}\n"),
        "a {\
         \n  b-c: 1;\
         \n  b-c: 2;\
         \n  b-c: 3;\
         \n  b-c: 4;\
         \n  b-c: 5;\
         \n}\n"
    );
}
#[test]
fn inclusive_backward() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 5 through 1 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: 5;\
         \n  b: 4;\
         \n  b: 3;\
         \n  b: 2;\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn inclusive_forward() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 1 through 5 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: 1;\
         \n  b: 2;\
         \n  b: 3;\
         \n  b: 4;\
         \n  b: 5;\
         \n}\n"
    );
}
#[test]
fn negative_to_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from -5 through -1 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: -5;\
         \n  b: -4;\
         \n  b: -3;\
         \n  b: -2;\
         \n  b: -1;\
         \n}\n"
    );
}
#[test]
fn negative_to_positive() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from -1 through 3 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: -1;\
         \n  b: 0;\
         \n  b: 1;\
         \n  b: 2;\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn positive_to_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  @for $i from 2 through -1 {b: $i;}\
             \n}\n"),
        "a {\
         \n  b: 2;\
         \n  b: 1;\
         \n  b: 0;\
         \n  b: -1;\
         \n}\n"
    );
}
#[test]
fn to_scope() {
    assert_eq!(
        runner().ok(
            "// Overriding the variable inside the loop will not impact the end of the loop.\
             \n$limit: 4;\n\
             \n@for $i from 1 through $limit {\
             \n  $limit: 2;\
             \n  a {\
             \n    limit: $limit;\
             \n    i: $i;\
             \n  }\
             \n}\n"
        ),
        "a {\
         \n  limit: 2;\
         \n  i: 1;\
         \n}\
         \na {\
         \n  limit: 2;\
         \n  i: 2;\
         \n}\
         \na {\
         \n  limit: 2;\
         \n  i: 3;\
         \n}\
         \na {\
         \n  limit: 2;\
         \n  i: 4;\
         \n}\n"
    );
}
mod unit {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("a {\
             \n  @for $i from 5mm through 1cm {b: $i;}\
             \n}\n"),
            "a {\
         \n  b: 5mm;\
         \n  b: 6mm;\
         \n  b: 7mm;\
         \n  b: 8mm;\
         \n  b: 9mm;\
         \n  b: 10mm;\
         \n}\n"
        );
    }
    #[test]
    fn from_unitless() {
        assert_eq!(
            runner().ok("a {\
             \n  @for $i from 1 through 5px {b: $i;}\
             \n}\n"),
            "a {\
         \n  b: 1;\
         \n  b: 2;\
         \n  b: 3;\
         \n  b: 4;\
         \n  b: 5;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok("a {\
             \n  @for $i from 1px through 5px {b: $i;}\
             \n}\n"),
            "a {\
         \n  b: 1px;\
         \n  b: 2px;\
         \n  b: 3px;\
         \n  b: 4px;\
         \n  b: 5px;\
         \n}\n"
        );
    }
    #[test]
    fn to_unitless() {
        assert_eq!(
            runner().ok("a {\
             \n  @for $i from 1px through 5 {b: $i;}\
             \n}\n"),
            "a {\
         \n  b: 1px;\
         \n  b: 2px;\
         \n  b: 3px;\
         \n  b: 4px;\
         \n  b: 5px;\
         \n}\n"
        );
    }
}
