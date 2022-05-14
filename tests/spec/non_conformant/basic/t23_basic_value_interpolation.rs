//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/23_basic_value_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("23_basic_value_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  a: hello#{world};\
             \n  a: hello #{world};\
             \n  b: 12#{3};\
             \n  b: type-of(12#{3});\
             \n  b: #{12 + 111};\
             \n  b: type-of(#{12 + 111});\
             \n}"),
        "div {\
         \n  a: helloworld;\
         \n  a: hello world;\
         \n  b: 12 3;\
         \n  b: list;\
         \n  b: 123;\
         \n  b: string;\
         \n}\n"
    );
}
