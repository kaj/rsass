//! Tests auto-converted from "sass-spec/spec/values/colors/alpha_hex/initial_digit.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("initial_digit").set_precision(10)
}

#[test]
fn test() {
    let runner = runner().set_precision(10);
    assert_eq!(
        runner.ok("@use \"sass:color\";\
             \na {\
             \n  four-digit: #0123;\
             \n  eight-digit: #98765432;\n\
             \n  // Verify that the color channels are set correctly.\
             \n  four-digit-red: color.red(#0123);\
             \n  four-digit-green: color.green(#0123);\
             \n  four-digit-blue: color.blue(#0123);\
             \n  four-digit-alpha: color.alpha(#0123);\n\
             \n  eight-digit-red: color.red(#98765432);\
             \n  eight-digit-green: color.green(#98765432);\
             \n  eight-digit-blue: color.blue(#98765432);\
             \n  eight-digit-alpha: color.alpha(#98765432);\
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
