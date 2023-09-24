//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/map.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("map")
        .mock_file("plain.css", "a {\n  x: (y: z);\n}\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@import \'plain\'"),
        "Error: expected \")\".\
         \n  ,\
         \n2 |   x: (y: z);\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
