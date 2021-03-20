//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/23_basic_value_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  a: hello#{world};\
            \n  a: hello #{world};\
            \n  b: 12#{3};\
            \n  b: type-of(12#{3});\
            \n  b: #{12 + 111};\
            \n  b: type-of(#{12 + 111});\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: helloworld;\
        \n  a: hello world;\
        \n  b: 12 3;\
        \n  b: list;\
        \n  b: 123;\
        \n  b: string;\
        \n}\
        \n"
    );
}
