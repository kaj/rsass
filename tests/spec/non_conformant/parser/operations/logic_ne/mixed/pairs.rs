//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_ne/mixed/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10!=10;\
            \n  test-2: 10 !=10;\
            \n  test-3: 10!= 10;\
            \n  test-4: 10 != 10;\
            \n  test-5: 10!=10%;\
            \n  test-6: 10 !=10%;\
            \n  test-7: 10!= 10%;\
            \n  test-8: 10 != 10%;\
            \n  test-9: 10!=10px;\
            \n  test-10: 10 !=10px;\
            \n  test-11: 10!= 10px;\
            \n  test-12: 10 != 10px;\
            \n  test-13: 10!=#AAA;\
            \n  test-14: 10 !=#AAA;\
            \n  test-15: 10!= #AAA;\
            \n  test-16: 10 != #AAA;\
            \n  test-17: 10!=#{itpl};\
            \n  test-18: 10 !=#{itpl};\
            \n  test-19: 10!= #{itpl};\
            \n  test-20: 10 != #{itpl};\
            \n  test-21: 10%!=10%;\
            \n  test-22: 10% !=10%;\
            \n  test-23: 10%!= 10%;\
            \n  test-24: 10% != 10%;\
            \n  test-25: 10%!=10px;\
            \n  test-26: 10% !=10px;\
            \n  test-27: 10%!= 10px;\
            \n  test-28: 10% != 10px;\
            \n  test-29: 10%!=#AAA;\
            \n  test-30: 10% !=#AAA;\
            \n  test-31: 10%!= #AAA;\
            \n  test-32: 10% != #AAA;\
            \n  test-33: 10%!=#{itpl};\
            \n  test-34: 10% !=#{itpl};\
            \n  test-35: 10%!= #{itpl};\
            \n  test-36: 10% != #{itpl};\
            \n  test-37: 10px!=10px;\
            \n  test-38: 10px !=10px;\
            \n  test-39: 10px!= 10px;\
            \n  test-40: 10px != 10px;\
            \n  test-41: 10px!=#AAA;\
            \n  test-42: 10px !=#AAA;\
            \n  test-43: 10px!= #AAA;\
            \n  test-44: 10px != #AAA;\
            \n  test-45: 10px!=#{itpl};\
            \n  test-46: 10px !=#{itpl};\
            \n  test-47: 10px!= #{itpl};\
            \n  test-48: 10px != #{itpl};\
            \n  test-49: #AAA!=#AAA;\
            \n  test-50: #AAA !=#AAA;\
            \n  test-51: #AAA!= #AAA;\
            \n  test-52: #AAA != #AAA;\
            \n  test-53: #AAA!=#{itpl};\
            \n  test-54: #AAA !=#{itpl};\
            \n  test-55: #AAA!= #{itpl};\
            \n  test-56: #AAA != #{itpl};\
            \n  test-57: #{itpl}!=#{itpl};\
            \n  test-58: #{itpl} !=#{itpl};\
            \n  test-59: #{itpl}!= #{itpl};\
            \n  test-60: #{itpl} != #{itpl};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: false;\
        \n  test-2: false;\
        \n  test-3: false;\
        \n  test-4: false;\
        \n  test-5: true;\
        \n  test-6: true;\
        \n  test-7: true;\
        \n  test-8: true;\
        \n  test-9: true;\
        \n  test-10: true;\
        \n  test-11: true;\
        \n  test-12: true;\
        \n  test-13: true;\
        \n  test-14: true;\
        \n  test-15: true;\
        \n  test-16: true;\
        \n  test-17: true;\
        \n  test-18: true;\
        \n  test-19: true;\
        \n  test-20: true;\
        \n  test-21: false;\
        \n  test-22: false;\
        \n  test-23: false;\
        \n  test-24: false;\
        \n  test-25: true;\
        \n  test-26: true;\
        \n  test-27: true;\
        \n  test-28: true;\
        \n  test-29: true;\
        \n  test-30: true;\
        \n  test-31: true;\
        \n  test-32: true;\
        \n  test-33: true;\
        \n  test-34: true;\
        \n  test-35: true;\
        \n  test-36: true;\
        \n  test-37: false;\
        \n  test-38: false;\
        \n  test-39: false;\
        \n  test-40: false;\
        \n  test-41: true;\
        \n  test-42: true;\
        \n  test-43: true;\
        \n  test-44: true;\
        \n  test-45: true;\
        \n  test-46: true;\
        \n  test-47: true;\
        \n  test-48: true;\
        \n  test-49: false;\
        \n  test-50: false;\
        \n  test-51: false;\
        \n  test-52: false;\
        \n  test-53: true;\
        \n  test-54: true;\
        \n  test-55: true;\
        \n  test-56: true;\
        \n  test-57: false;\
        \n  test-58: false;\
        \n  test-59: false;\
        \n  test-60: false;\
        \n}\
        \n"
    );
}
