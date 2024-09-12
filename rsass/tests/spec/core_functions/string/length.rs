//! Tests auto-converted from "sass-spec/spec/core_functions/string/length.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("length")
}

#[test]
fn combining_character() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Sass does *not* treat strings as sequences of glyphs, so this string which\
             \n// contains \"c\" followed by a combining umlaut should be considered two separate\
             \n// characters even though it\'s rendered as only one.\
             \na {b: string.length(\"c\\0308\")}\n"
        ),
        "a {\
         \n  b: 2;\
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
             \na {b: string.length(\"👭\")}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.length(\"\")}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.length()}\n"
            ),
            "Error: Missing argument $string.\
         \n  ,--> input.scss\
         \n2 | a {b: string.length()}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function length($string) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.length(c, d)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.length(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function length($string) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.length(1)}\n"
            ),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.length(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.str-length(\"c\")}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-length(\"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn multiple_characters() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.length(\"fblthp abatement\")}\n"),
        "a {\
         \n  b: 16;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.length($string: \"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn one_character() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.length(\"c\")}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn private_use_character() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Dart Sass emits private-use characters as escapes in expanded mode, but it\
             \n// should stil treat them as single characters for the purpose of functions.\
             \na {b: string.length(\"\\E000\")}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: string.length(loofamonster)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
