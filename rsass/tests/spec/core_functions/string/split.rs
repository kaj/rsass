//! Tests auto-converted from "sass-spec/spec/core_functions/string/split.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("split")
}

#[test]
fn both_empty() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(\"\", \"\")}\n"),
        "a {\
         \n  b: [];\
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
             \na {b: string.split(\"👭a\", \"\")}\n"
        ),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: [\"👭\", \"a\"];\
         \n}\n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$result: string.split(\"\", \"/\");\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: list.separator($result); \
             \n}\n"),
        "a {\
         \n  value: [];\
         \n  separator: comma;\
         \n}\n"
    );
}
#[test]
fn empty_separator() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \na {b: string.split(\"Helvetica\", \"\")}\n"
        ),
        "a {\
         \n  b: [\"H\", \"e\", \"l\", \"v\", \"e\", \"t\", \"i\", \"c\", \"a\"];\
         \n}\n"
    );
}
#[test]
fn empty_string() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(\"\", \"/\")}\n"),
        "a {\
         \n  b: [];\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.split(\"1/2/3\", \"/\", 0.5)}\n"
            ),
            "Error: $limit: 0.5 is not an int.\
         \n  ,\
         \n2 | a {b: string.split(\"1/2/3\", \"/\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn limit_zero() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.split(\"a, b, c\", \", \", 0)}\n"
            ),
            "Error: $limit: Must be 1 or greater, was 0.\
         \n  ,\
         \n2 | a {b: string.split(\"a, b, c\", \", \", 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn negative_limit() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.split(\"1/2/3\", \"/\", -1)}\n"
            ),
            "Error: $limit: Must be 1 or greater, was -1.\
         \n  ,\
         \n2 | a {b: string.split(\"1/2/3\", \"/\", -1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.split(\"a/b/c\")}\n"
            ),
            "Error: Missing argument $separator.\
         \n  ,--> input.scss\
         \n2 | a {b: string.split(\"a/b/c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function split($string, $separator, $limit: null) {\
         \n  |           ======================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.split(\"a/b/c\", \"/\", 1, 3)}\n"
            ),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.split(\"a/b/c\", \"/\", 1, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function split($string, $separator, $limit: null) {\
         \n  |           ======================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        use super::runner;

        #[test]
        fn limit() {
            assert_eq!(
                runner().err(
                    "@use \"sass:string\";\
             \na {b: string.split(\"1/2/3\", \"/\", \"1\")}\n"
                ),
                "Error: $limit: \"1\" is not a number.\
         \n  ,\
         \n2 | a {b: string.split(\"1/2/3\", \"/\", \"1\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn separator() {
            assert_eq!(
                runner().err(
                    "@use \"sass:string\";\
             \na {b: string.split(\"1/2/3\", 1)}\n"
                ),
                "Error: $separator: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.split(\"1/2/3\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn string() {
            assert_eq!(
                runner().err(
                    "@use \"sass:string\";\
             \na {b: string.split(1, \"%\")}\n"
                ),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.split(1, \"%\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn limit() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(\"a, b, c, d\", \", \", 2)}\n"),
        "a {\
         \n  b: [\"a\", \"b\", \"c, d\"];\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \na {b: string.split($string: \"a/b/c\", $separator: \"/\", $limit: 1)}\n"
        ),
        "a {\
         \n  b: [\"a\", \"b/c\"];\
         \n}\n"
    );
}
#[test]
fn private_use_character() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Dart Sass emits private-use characters as escapes in expanded mode, but it\
             \n// should still treat them as single characters for the purpose of functions.\
             \na {b: string.split(\"\\E000\", \"\")}\n"
        ),
        "a {\
         \n  b: [\"\\e000\"];\
         \n}\n"
    );
}
#[test]
fn separator() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(\"a, b, c\", \", \")}\n"),
        "a {\
         \n  b: [\"a\", \"b\", \"c\"];\
         \n}\n"
    );
}
#[test]
fn separator_not_found() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(\"a, b, c\", \"&\")}\n"),
        "a {\
         \n  b: [\"a, b, c\"];\
         \n}\n"
    );
}
#[test]
fn single() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"sass:string\";\
             \n$result: string.split(\"a\", \"\");\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: list.separator($result); \
             \n}\n"),
        "a {\
         \n  value: [\"a\",];\
         \n  separator: comma;\
         \n}\n"
    );
}
#[test]
fn unquoted_string() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.split(abc, \"\")}\n"),
        "a {\
         \n  b: [a, b, c];\
         \n}\n"
    );
}
