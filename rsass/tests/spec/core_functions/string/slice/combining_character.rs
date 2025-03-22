//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/combining_character.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("combining_character")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// Sass does *not* treat strings as sequences of glyphs, so this string which\
             \n// contains \"c\" followed by a combining umlaut should be considered two separate\
             \n// characters even though it\'s rendered as only one and only the \"d\" should be\
             \n// sliced out.\
             \na {b: string.slice(\"cd\\0308e\", 2, 2)}\n"
        ),
        "a {\
         \n  b: \"d\";\
         \n}\n"
    );
}
