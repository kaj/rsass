//! Tests auto-converted from "sass-spec/spec/core_functions/string/insert.hrx"

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
             \n// characters even though it\'s rendered as only one and the \"d\" should be\
             \n// injected between the two.\
             \na {b: str-insert(\"c\\0308\", \"d\", 2)}\n"
        ),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: \"cd̈\";\
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
             \na {b: str-insert(\"👭\", \"c\", 2)}\n"
        ),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: \"👭c\";\
         \n}\n"
    );
}
mod empty_destination {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty_source() {
        assert_eq!(
            runner().ok("a {b: str-insert(\"\", \"\", 1)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn index_0() {
        assert_eq!(
            runner().ok("a {b: str-insert(\"\", \"c\", 0)}\n"),
            "a {\
         \n  b: \"c\";\
         \n}\n"
        );
    }
    #[test]
    fn index_1() {
        assert_eq!(
            runner().ok("a {b: str-insert(\"\", \"c\", 1)}\n"),
            "a {\
         \n  b: \"c\";\
         \n}\n"
        );
    }
    #[test]
    fn index_2() {
        assert_eq!(
            runner().ok("a {b: str-insert(\"\", \"c\", 2)}\n"),
            "a {\
         \n  b: \"c\";\
         \n}\n"
        );
    }
    #[test]
    fn index_negative_1() {
        assert_eq!(
            runner().ok("a {b: str-insert(\"\", \"c\", -1)}\n"),
            "a {\
         \n  b: \"c\";\
         \n}\n"
        );
    }
}
#[test]
fn empty_insertion() {
    assert_eq!(
        runner().ok("a {b: str-insert(\"cde\", \"\", 1)}\n"),
        "a {\
         \n  b: \"cde\";\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().err("a {b: str-insert(\"\", \"\", 0.5)}\n"),
            "Error: $index: 0.5 is not an int.\
         \n  ,\
         \n1 | a {b: str-insert(\"\", \"\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: str-insert(\"\", \"\")}\n"),
            "Error: Missing argument $index.\
         \n  ,--> input.scss\
         \n1 | a {b: str-insert(\"\", \"\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function insert($string, $insert, $index) {\
         \n  |           ================================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: str-insert(\"\", \"\", 1, 2)}\n"),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: str-insert(\"\", \"\", 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function insert($string, $insert, $index) {\
         \n  |           ================================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn index() {
            assert_eq!(
                runner().err("a {b: str-insert(\"\", \"\", \"\")}\n"),
                "Error: $index: \"\" is not a number.\
         \n  ,\
         \n1 | a {b: str-insert(\"\", \"\", \"\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn insert() {
            assert_eq!(
                runner().err("a {b: str-insert(\"\", 1, 1)}\n"),
                "Error: $insert: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-insert(\"\", 1, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn string() {
            assert_eq!(
                runner().err("a {b: str-insert(1, \"\", 1)}\n"),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-insert(1, \"\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod index {
    #[allow(unused)]
    use super::runner;

    mod negative {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", -1)}\n"),
                "a {\
         \n  b: \"cdef\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", -2)}\n"),
                "a {\
         \n  b: \"cdfe\";\
         \n}\n"
            );
        }
        mod after_last {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn less_than_double() {
                assert_eq!(
                    runner().ok("// Regression test for sass/dart-sass#1568\
             \na {b: str-insert(\"cdefghijkl\", \"mno\", -15)}\n"),
                    "a {\
         \n  b: \"mnocdefghijkl\";\
         \n}\n"
                );
            }
            #[test]
            fn more_than_double() {
                assert_eq!(
                    runner().ok("a {b: str-insert(\"cde\", \"f\", -100)}\n"),
                    "a {\
         \n  b: \"fcde\";\
         \n}\n"
                );
            }
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", -4)}\n"),
                "a {\
         \n  b: \"fcde\";\
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
                runner().ok("a {b: str-insert(\"cde\", \"f\", 0)}\n"),
                "a {\
         \n  b: \"fcde\";\
         \n}\n"
            );
        }
        #[test]
        fn t1() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", 1)}\n"),
                "a {\
         \n  b: \"fcde\";\
         \n}\n"
            );
        }
        #[test]
        fn t2() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", 2)}\n"),
                "a {\
         \n  b: \"cfde\";\
         \n}\n"
            );
        }
        #[test]
        fn after_last() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", 100)}\n"),
                "a {\
         \n  b: \"cdef\";\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("a {b: str-insert(\"cde\", \"f\", 4)}\n"),
                "a {\
         \n  b: \"cdef\";\
         \n}\n"
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: str-insert($string: \"cde\", $insert: \"f\", $index: 2)}\n"
        ),
        "a {\
         \n  b: \"cfde\";\
         \n}\n"
    );
}
