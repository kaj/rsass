//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/21_one_builtin_function.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  color: rgb(255, $blue: 0, $green: 255);\
            \n  background: rgb(123, 45, 6);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: yellow;\
        \n  background: #7b2d06;\
        \n}\
        \n"
    );
}
