//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn combining_character() {
    assert_eq!(
        runner().ok(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
             \n// contains \"c\" followed by a combining umlaut should be considered two separate\
             \n// characters even though it\'s rendered as only one and only the \"d\" should be\
             \n// sliced out.\
             \na {b: str-slice(\"cd\\0308e\", 2, 2)}\n"
        ),
        "a {\
         \n  b: \"d\";\
         \n}\n"
    );
}
#[test]
fn double_width_character() {
    assert_eq!(
        runner().ok(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
             \n// character is represented as two UTF-16 code units, so inserting a character\
             \n// at index 2 shouldn\'t break this emoji in two.\
             \na {b: str-slice(\"c👭d\", 2, 2)}\n"
        ),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: \"👭\";\
         \n}\n"
    );
}
mod empty {
    #[allow(unused)]
    use super::runner;
    mod end {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t0() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 1, 0)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 1, 1)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 1, 2)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
    }
    mod start {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t0() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 0)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 1)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", 2)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn negative_1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"\", -1)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
    }
}
mod end {
    #[allow(unused)]
    use super::runner;
    mod negative {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, -1)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, -2)}\n"),
                "a {\
         \n  b: \"cd\";\
         \n}\n"
            );
        }
        #[test]
        fn after_last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, -100)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, -4)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
    }
    mod positive {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t0() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, 0)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, 1)}\n"),
                "a {\
         \n  b: \"c\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, 2)}\n"),
                "a {\
         \n  b: \"cd\";\
         \n}\n"
            );
        }
        #[test]
        fn after_last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, 100)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
        #[test]
        fn after_start() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cdef\", 2, 3)}\n"),
                "a {\
         \n  b: \"de\";\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1, 3)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
    }
}
mod error {
    #[allow(unused)]
    use super::runner;
    mod decimal {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
        fn end() {
            assert_eq!(
                runner().err("a {b: str-slice(\"\", 1, 1.5)}\n"),
                "Error: 1.5 is not an int.\
         \n  ,\
         \n1 | a {b: str-slice(\"\", 1, 1.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn start() {
            assert_eq!(
                runner().err("a {b: str-slice(\"\", 0.5)}\n"),
                "Error: 0.5 is not an int.\
         \n  ,\
         \n1 | a {b: str-slice(\"\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: str-slice(\"cde\")}\n"),
            "Error: Missing argument $start-at.\
         \n  ,--> input.scss\
         \n1 | a {b: str-slice(\"cde\")}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: str-slice(\"cde\", 1, 2, 3)}\n"),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: str-slice(\"cde\", 1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn end_at() {
            assert_eq!(
                runner().err("a {b: str-slice(\"cde\", 1, \"f\")}\n"),
                "Error: $end-at: \"f\" is not a number.\
         \n  ,\
         \n1 | a {b: str-slice(\"cde\", 1, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn start_at() {
            assert_eq!(
                runner().err("a {b: str-slice(\"cde\", \"f\")}\n"),
                "Error: $start-at: \"f\" is not a number.\
         \n  ,\
         \n1 | a {b: str-slice(\"cde\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn string() {
            assert_eq!(
                runner().err("a {b: str-slice(1, 2)}\n"),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-slice(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: str-slice($string: \"cde\", $start-at: 2, $end-at: 2)}\n"
        ),
        "a {\
         \n  b: \"d\";\
         \n}\n"
    );
}
mod start {
    #[allow(unused)]
    use super::runner;
    mod negative {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", -1)}\n"),
                "a {\
         \n  b: \"e\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", -2)}\n"),
                "a {\
         \n  b: \"de\";\
         \n}\n"
            );
        }
        #[test]
        fn after_last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", -100)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", -3)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
    }
    mod positive {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn t0() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 0)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 1)}\n"),
                "a {\
         \n  b: \"cde\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 2)}\n"),
                "a {\
         \n  b: \"de\";\
         \n}\n"
            );
        }
        #[test]
        fn after_end() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cdef\", 3, 2)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn after_last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 100)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-slice(\"cde\", 4)}\n"),
                "a {\
         \n  b: \"\";\
         \n}\n"
            );
        }
    }
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("a {b: str-slice(cdefgh, 3, 5)}\n"),
        "a {\
         \n  b: efg;\
         \n}\n"
    );
}
