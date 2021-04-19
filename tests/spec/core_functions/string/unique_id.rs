//! Tests auto-converted from "sass-spec/spec/core_functions/string/unique_id.hrx"

mod error {
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: unique-id(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 0 arguments allowed, but 1 was passed.\
         \n  ,--> input.scss\
         \n1 | a {b: unique-id(c)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function unique-id() {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn is_identifier() {
    assert_eq!(
        crate::rsass(
            "// Every call to unique-id() should return a valid CSS identifier. We can\'t test\
            \n// this directly, so we make sure it can parse as a class selector with\
            \n// selector-parse().\
            \n@for $i from 1 to 1000 {\
            \n  $_: selector-parse(\".#{unique-id()}\");\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
fn is_unique() {
    assert_eq!(
        crate::rsass(
            "// As the name suggests, every call to unique-id() should return a different\
            \n// value.\
            \n$ids: ();\
            \n@for $i from 1 to 1000 {\
            \n  $id: unique-id();\
            \n  @if map-has-key($ids, $id) {\
            \n    @error \"#{$id} generated more than once\";\
            \n  }\
            \n\
            \n  $ids: map-merge($ids, ($id: null));\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
