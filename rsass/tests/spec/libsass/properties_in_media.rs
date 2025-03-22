//! Tests auto-converted from "sass-spec/spec/libsass/properties-in-media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("properties-in-media")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@media only screen {\r\
             \n    /* asd */\r\
             \n  color: red;\r\
             \n  color: gray;\r\
             \n    /* asd */\r\
             \n  color: green;\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n    qwe: baz;\r\
             \n  }\r\
             \n  color: blue;\r\
             \n  color: yellow;\r\
             \n}"
        ),
        "Error: expected \"{\".\
         \n  ,\
         \n3 |   color: red;\
         \n  |             ^\
         \n  \'\
         \n  input.scss 3:13  root stylesheet",
    );
}
