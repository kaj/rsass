//! Tests auto-converted from "sass-spec/spec/values/colors/alpha_hex/initial_letter.hrx"

#[test]
fn test() {
    let format = rsass::output::Format {
        style: rsass::output::Style::Expanded,
        precision: 10,
    };
    assert_eq!(
        crate::rsass_fmt(
            format,
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
