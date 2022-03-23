//! Tests auto-converted from "sass-spec/spec/values/colors/alpha_hex/initial_letter.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().set_precision(10)
}

#[test]
fn test() {
    let runner = runner().set_precision(10);
    assert_eq!(
        runner.ok("a {\
             \n  four-digit: #AbCd;\
             \n  eight-digit: #aBcDeF12;\n\
             \n  // Verify that the color channels are set correctly.\
             \n  four-digit-red: red(#abcd);\
             \n  four-digit-green: green(#abcd);\
             \n  four-digit-blue: blue(#abcd);\
             \n  four-digit-alpha: alpha(#abcd);\n\
             \n  eight-digit-red: red(#ABCDEF12);\
             \n  eight-digit-green: green(#ABCDEF12);\
             \n  eight-digit-blue: blue(#ABCDEF12);\
             \n  eight-digit-alpha: alpha(#ABCDEF12);\
             \n}\n"),
        "a {\
         \n  four-digit: rgba(170, 187, 204, 0.8666666667);\
         \n  eight-digit: rgba(171, 205, 239, 0.0705882353);\
         \n  four-digit-red: 170;\
         \n  four-digit-green: 187;\
         \n  four-digit-blue: 204;\
         \n  four-digit-alpha: 0.8666666667;\
         \n  eight-digit-red: 171;\
         \n  eight-digit-green: 205;\
         \n  eight-digit-blue: 239;\
         \n  eight-digit-alpha: 0.0705882353;\
         \n}\n"
    );
}
