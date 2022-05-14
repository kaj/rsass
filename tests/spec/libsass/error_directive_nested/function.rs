//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function c() {\
             \n  @error test;\
             \n  @return d;\
             \n}\n\
             \na {\
             \n  b: {\
             \n    c: c();\
             \n  }\
             \n}\n"
        ),
        "Error: test\
         \n  ,\
         \n8 |     c: c();\
         \n  |        ^^^\
         \n  \'\
         \n  input.scss 8:8  root stylesheet",
    );
}
