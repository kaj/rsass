//! Tests auto-converted from "sass-spec/spec/values/maps/errors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("errors")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "$map: ( foo: bar );\
             \ntest { baz: $map; }\n"
        ),
        "Error: (foo: bar) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | test { baz: $map; }\
         \n  |             ^^^^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
    );
}
