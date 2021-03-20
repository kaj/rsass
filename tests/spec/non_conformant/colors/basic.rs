//! Tests auto-converted from "sass-spec/spec/non_conformant/colors/basic.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "p {\
            \n  color: rgb(255, 128, 0);\
            \n  color: red green blue;\
            \n  color: (red) (green) (blue);\
            \n  color: red + hux;\
            \n  color: unquote(\"red\") + green;\
            \n  foo: rgb(200, 150%, 170%);\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  color: #ff8000;\
        \n  color: red green blue;\
        \n  color: red green blue;\
        \n  color: redhux;\
        \n  color: redgreen;\
        \n  foo: #c8ffff;\
        \n}\
        \n"
    );
}
