//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolation-operators-precedence.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolation-operators-precedence")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ".test {\
             \n  a01: (#{a}+5.0% + 2);\
             \n  a02: (#{a}+ 5.0% + 2);\
             \n  a03: (#{a}  +5.0% + 2);\
             \n  a04: (#{a} +  5.0% + 2);\
             \n  b01: (5 + 2.0%+#{a});\
             \n  b02: (5 + 2.0%+ #{a});\
             \n  b03: (5 + 2.0%  +#{a});\
             \n  b04: (5 + 2.0% +  #{a});\
             \n  c01: (#{a} +5.0% + 2);\
             \n  c02: (#{a} -5.0% + 2);\
             \n  c03: (#{a} /5.0% + 2);\
             \n  c04: (#{a} *5.0% + 2);\
             \n  c05: (#{a} +5.0% - 2);\
             \n  c06: (#{a} -5.0% - 2);\
             \n  c07: (#{a} /5.0% - 2);\
             \n  c08: (#{a} *5.0% - 2);\
             \n  c09: (#{a} +5.0% / 2);\
             \n  c10: (#{a} -5.0% / 2);\
             \n  c11: (#{a} /5.0% / 2);\
             \n  c12: (#{a} *5.0% / 2);\
             \n  c13: (#{a} +5.0% * 2);\
             \n  c14: (#{a} -5.0% * 2);\
             \n  c15: (#{a} /5.0% * 2);\
             \n  c16: (#{a} *5.0% * 2);\
             \n  d01: (5 + 2.0% +#{a});\
             \n  d02: (5 + 2.0% -#{a});\
             \n  d03: (5 + 2.0% /#{a});\
             \n  d04: (5 + 2.0% *#{a});\
             \n  d05: (5 - 2.0% +#{a});\
             \n  d06: (5 - 2.0% -#{a});\
             \n  d07: (5 - 2.0% /#{a});\
             \n  d08: (5 - 2.0% *#{a});\
             \n  d09: (5% / 2.0 +#{a});\
             \n  d10: (5% / 2.0 -#{a});\
             \n  d11: (5% / 2.0 /#{a});\
             \n  d12: (5% / 2.0 *#{a});\
             \n  d13: (5 * 2.0% +#{a});\
             \n  d14: (5 * 2.0% -#{a});\
             \n  d15: (5 * 2.0% /#{a});\
             \n  d16: (5 * 2.0% *#{a});\
             \n  e01: (#{a} ==5.0% == 2);\
             \n  e02: (#{a} >5.0% > 2);\
             \n  e03: (#{a} <5.0% < 2);\
             \n  e04: (#{a} >=5.0% >= 2);\
             \n  e05: (#{a} <=5.0% <= 2);\
             \n  e06: (#{a} !=5.0% != 2);\
             \n}\n"
        ),
        "DEPRECATION WARNING on line 4, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    #{a} + 5%\n\
         \nbut you may have intended it to mean:\n\
         \n    #{a} (+5%)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n  ,\
         \n4 |   a03: (#{a}  +5.0% + 2);\
         \n  |         ^^^^^^^^^^^\
         \n  \'\n\
         \nDEPRECATION WARNING on line 8, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    5 + 2% + #{a}\n\
         \nbut you may have intended it to mean:\n\
         \n    5 + 2% (+#{a})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n  ,\
         \n8 |   b03: (5 + 2.0%  +#{a});\
         \n  |         ^^^^^^^^^^^^^^^\
         \n  \'\n\
         \nDEPRECATION WARNING on line 10, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    #{a} + 5%\n\
         \nbut you may have intended it to mean:\n\
         \n    #{a} (+5%)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n10 |   c01: (#{a} +5.0% + 2);\
         \n   |         ^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 14, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    #{a} + 5%\n\
         \nbut you may have intended it to mean:\n\
         \n    #{a} (+5%)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n14 |   c05: (#{a} +5.0% - 2);\
         \n   |         ^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 18, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    #{a} + 5% / 2\n\
         \nbut you may have intended it to mean:\n\
         \n    #{a} (+5% / 2)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n18 |   c09: (#{a} +5.0% / 2);\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 22, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    #{a} + 5% * 2\n\
         \nbut you may have intended it to mean:\n\
         \n    #{a} (+5% * 2)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n22 |   c13: (#{a} +5.0% * 2);\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 26, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    5 + 2% + #{a}\n\
         \nbut you may have intended it to mean:\n\
         \n    5 + 2% (+#{a})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n26 |   d01: (5 + 2.0% +#{a});\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 30, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    5 - 2% + #{a}\n\
         \nbut you may have intended it to mean:\n\
         \n    5 - 2% (+#{a})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n30 |   d05: (5 - 2.0% +#{a});\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 34, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    5% / 2 + #{a}\n\
         \nbut you may have intended it to mean:\n\
         \n    5% / 2 (+#{a})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n34 |   d09: (5% / 2.0 +#{a});\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nDEPRECATION WARNING on line 38, column 9 of input.scss: \
         \nThis operation is parsed as:\n\
         \n    5 * 2% + #{a}\n\
         \nbut you may have intended it to mean:\n\
         \n    5 * 2% (+#{a})\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\
         \n   ,\
         \n38 |   d13: (5 * 2.0% +#{a});\
         \n   |         ^^^^^^^^^^^^^^\
         \n   \'\n\
         \nError: Undefined operation \"a * 5%\".\
         \n   ,\
         \n13 |   c04: (#{a} *5.0% + 2);\
         \n   |         ^^^^^^^^^^\
         \n   \'\
         \n  input.scss 13:9  root stylesheet",
    );
}
