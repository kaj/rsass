//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@mixin c() {\
             \n  @error test;\
             \n  c: d;\
             \n}\n\
             \na {\
             \n  b: {\
             \n    @include c();\
             \n  }\
             \n}\n"
        ),
        "Error: test\
         \n  ,\
         \n8 |     @include c();\
         \n  |     ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 8:5  root stylesheet",
    );
}
