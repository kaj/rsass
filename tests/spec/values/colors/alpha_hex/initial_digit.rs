//! Tests auto-converted from "sass-spec/spec/values/colors/alpha_hex/initial_digit.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().set_precision(10)
}

#[test]
fn test() {
    let runner = runner().set_precision(10);
    assert_eq!(
        runner.ok("a {\
             \n  four-digit: #0123;\
             \n  eight-digit: #98765432;\n\
             \n  // Verify that the color channels are set correctly.\
             \n  four-digit-red: red(#0123);\
             \n  four-digit-green: green(#0123);\
             \n  four-digit-blue: blue(#0123);\
             \n  four-digit-alpha: alpha(#0123);\n\
             \n  eight-digit-red: red(#98765432);\
             \n  eight-digit-green: green(#98765432);\
             \n  eight-digit-blue: blue(#98765432);\
             \n  eight-digit-alpha: alpha(#98765432);\
             \n}\n"),
        "a {\
         \n  four-digit: rgba(0, 17, 34, 0.2);\
         \n  eight-digit: rgba(152, 118, 84, 0.1960784314);\
         \n  four-digit-red: 0;\
         \n  four-digit-green: 17;\
         \n  four-digit-blue: 34;\
         \n  four-digit-alpha: 0.2;\
         \n  eight-digit-red: 152;\
         \n  eight-digit-green: 118;\
         \n  eight-digit-blue: 84;\
         \n  eight-digit-alpha: 0.1960784314;\
         \n}\n"
    );
}
