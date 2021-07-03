//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  b: {\
             \n    @error test;\
             \n    c: d;\
             \n  }\
             \n}\n"
        ),
        "Error: test\
         \n  ,\
         \n3 |     @error test;\
         \n  |     ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
    );
}
