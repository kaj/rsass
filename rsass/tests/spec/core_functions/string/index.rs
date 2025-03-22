//! Tests auto-converted from "sass-spec/spec/core_functions/string/index.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("index")
}

#[test]
fn beginning() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index(\"cde\", \"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn both_empty() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index(\"\", \"\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn combining_character() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Sass does *not* treat strings as sequences of glyphs, so this string which\
             \n// contains \"c\" followed by a combining umlaut should be considered two separate\
             \n// characters even though it\'s rendered as only one.\
             \na {b: string.index(\"c\\0308 a\", \"a\")}\n"
        ),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn double_width_character() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
             \n// character is represented as two UTF-16 code units.\
             \na {b: string.index(\"👭a\", \"a\")}\n"
        ),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn empty_substring() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index(\"cde\", \"\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn end() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index(\"cde\", \"e\")}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.index(\"c\")}\n"
            ),
            "Error: Missing argument $substring.\
         \n  ,--> input.scss\
         \n2 | a {b: string.index(\"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function index($string, $substring) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.index(\"c\", \"d\", \"e\")}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.index(\"c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function index($string, $substring) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        use super::runner;

        #[test]
        fn string() {
            assert_eq!(
                runner().err(
                    "@use \"sass:string\";\
             \na {b: string.index(1, \"c\")}\n"
                ),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.index(1, \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn substring() {
            assert_eq!(
                runner().err(
                    "@use \"sass:string\";\
             \na {b: string.index(\"c\", 1)}\n"
                ),
                "Error: $substring: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.index(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.str-index(\"c\", \"c\")}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-index(\"c\", \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index(\"cde\", \"d\")}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.index($string: \"cde\", $substring: \"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn not_found() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \na {b: meta.inspect(string.index(\"cde\", \"f\"))}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
