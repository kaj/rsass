//! Tests auto-converted from "sass-spec/spec/values/maps/errors.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: ( foo: bar );\
             \ntest { baz: $map; }\
             \n"
        )
        .unwrap_err(),
        "Error: (foo: bar) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | test { baz: $map; }\
         \n  |             ^^^^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
    );
}
