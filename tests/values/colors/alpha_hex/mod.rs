//! Tests auto-converted from "sass-spec/spec/values/colors/alpha_hex"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/values/colors/alpha_hex/initial_digit.hrx"
#[test]
#[ignore] // failing
fn initial_digit() {
    set_precision(10);
    assert_eq!(
        rsass(
            "a {\
             \n  four-digit: #0123;\
             \n  eight-digit: #98765432;\
             \n\
             \n  // Verify that the color channels are set correctly.\
             \n  four-digit-red: red(#0123);\
             \n  four-digit-green: green(#0123);\
             \n  four-digit-blue: blue(#0123);\
             \n  four-digit-alpha: alpha(#0123);\
             \n\
             \n  eight-digit-red: red(#98765432);\
             \n  eight-digit-green: green(#98765432);\
             \n  eight-digit-blue: blue(#98765432);\
             \n  eight-digit-alpha: alpha(#98765432);\
             \n}\
             \n"
        )
        .unwrap(),
        "a {\
         \n  four-digit: #0123;\
         \n  eight-digit: #98765432;\
         \n  four-digit-red: 0;\
         \n  four-digit-green: 17;\
         \n  four-digit-blue: 34;\
         \n  four-digit-alpha: 0.2;\
         \n  eight-digit-red: 152;\
         \n  eight-digit-green: 118;\
         \n  eight-digit-blue: 84;\
         \n  eight-digit-alpha: 0.1960784314;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/values/colors/alpha_hex/initial_letter.hrx"
#[test]
#[ignore] // failing
fn initial_letter() {
    set_precision(10);
    assert_eq!(
        rsass(
            "a {\
             \n  four-digit: #AbCd;\
             \n  eight-digit: #aBcDeF12;\
             \n\
             \n  // Verify that the color channels are set correctly.\
             \n  four-digit-red: red(#abcd);\
             \n  four-digit-green: green(#abcd);\
             \n  four-digit-blue: blue(#abcd);\
             \n  four-digit-alpha: alpha(#abcd);\
             \n\
             \n  eight-digit-red: red(#ABCDEF12);\
             \n  eight-digit-green: green(#ABCDEF12);\
             \n  eight-digit-blue: blue(#ABCDEF12);\
             \n  eight-digit-alpha: alpha(#ABCDEF12);\
             \n}\
             \n"
        )
        .unwrap(),
        "a {\
         \n  four-digit: #AbCd;\
         \n  eight-digit: #aBcDeF12;\
         \n  four-digit-red: 170;\
         \n  four-digit-green: 187;\
         \n  four-digit-blue: 204;\
         \n  four-digit-alpha: 0.8666666667;\
         \n  eight-digit-red: 171;\
         \n  eight-digit-green: 205;\
         \n  eight-digit-blue: 239;\
         \n  eight-digit-alpha: 0.0705882353;\
         \n}\
         \n"
    );
}
