//! Tests auto-converted from "sass-spec/spec/css/custom_properties/exclamation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("exclamation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".exclamation {\
             \n  // `!` is technically not allowed at the top-level of a custom property, but\
             \n  // that\'s only because `!important` is filtered out before the custom property\
             \n  // is parsed by the CSS parser. As far as Sass is concerned, it\'s fine.\
             \n  --important: value !important;\n\
             \n  // We even allow constructions like these for forwards-compatibility with\
             \n  // additional flags or syntax CSS might add.\
             \n  --multiple: !important !important;\
             \n  --other-word: !something;\
             \n  --in-identifier: foo!bar;\
             \n  --just-exclam: !;\
             \n  --just-exclams: !!!!!!!;\
             \n}\n"
        ),
        ".exclamation {\
         \n  --important: value !important;\
         \n  --multiple: !important !important;\
         \n  --other-word: !something;\
         \n  --in-identifier: foo!bar;\
         \n  --just-exclam: !;\
         \n  --just-exclams: !!!!!!!;\
         \n}\n"
    );
}
