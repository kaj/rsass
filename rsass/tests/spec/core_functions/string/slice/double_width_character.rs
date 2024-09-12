//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/double_width_character.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("double_width_character")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
             \n// character is represented as two UTF-16 code units, so inserting a character\
             \n// at index 2 shouldn\'t break this emoji in two.\
             \na {b: string.slice(\"c👭d\", 2, 2)}\n"
        ),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: \"👭\";\
         \n}\n"
    );
}
