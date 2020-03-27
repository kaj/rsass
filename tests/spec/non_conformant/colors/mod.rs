//! Tests auto-converted from "sass-spec/spec/non_conformant/colors"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/colors/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/colors/change-color.hrx"
#[test]
fn change_color() {
    assert_eq!(
        rsass(
            "p {\
            \n  color: change-color(#102030, $blue: 5);\
            \n  color: change-color(#102030, $alpha: .325);\
            \n  color: change-color(#102030, $red: 120, $blue: 5);\
            \n  color: change-color(hsl(25, 100%, 80%), $lightness: 40%, $alpha: 0.8);\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  color: #102005;\
        \n  color: rgba(16, 32, 48, 0.325);\
        \n  color: #782005;\
        \n  color: rgba(204, 85, 0, 0.8);\
        \n}\
        \n"
    );
}
