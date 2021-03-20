//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_eq/mixed/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10==10;\
            \n  test-2: 10 ==10;\
            \n  test-3: 10== 10;\
            \n  test-4: 10 == 10;\
            \n  test-5: 10==10%;\
            \n  test-6: 10 ==10%;\
            \n  test-7: 10== 10%;\
            \n  test-8: 10 == 10%;\
            \n  test-9: 10==10px;\
            \n  test-10: 10 ==10px;\
            \n  test-11: 10== 10px;\
            \n  test-12: 10 == 10px;\
            \n  test-13: 10==#AAA;\
            \n  test-14: 10 ==#AAA;\
            \n  test-15: 10== #AAA;\
            \n  test-16: 10 == #AAA;\
            \n  test-17: 10==#{itpl};\
            \n  test-18: 10 ==#{itpl};\
            \n  test-19: 10== #{itpl};\
            \n  test-20: 10 == #{itpl};\
            \n  test-21: 10%==10%;\
            \n  test-22: 10% ==10%;\
            \n  test-23: 10%== 10%;\
            \n  test-24: 10% == 10%;\
            \n  test-25: 10%==10px;\
            \n  test-26: 10% ==10px;\
            \n  test-27: 10%== 10px;\
            \n  test-28: 10% == 10px;\
            \n  test-29: 10%==#AAA;\
            \n  test-30: 10% ==#AAA;\
            \n  test-31: 10%== #AAA;\
            \n  test-32: 10% == #AAA;\
            \n  test-33: 10%==#{itpl};\
            \n  test-34: 10% ==#{itpl};\
            \n  test-35: 10%== #{itpl};\
            \n  test-36: 10% == #{itpl};\
            \n  test-37: 10px==10px;\
            \n  test-38: 10px ==10px;\
            \n  test-39: 10px== 10px;\
            \n  test-40: 10px == 10px;\
            \n  test-41: 10px==#AAA;\
            \n  test-42: 10px ==#AAA;\
            \n  test-43: 10px== #AAA;\
            \n  test-44: 10px == #AAA;\
            \n  test-45: 10px==#{itpl};\
            \n  test-46: 10px ==#{itpl};\
            \n  test-47: 10px== #{itpl};\
            \n  test-48: 10px == #{itpl};\
            \n  test-49: #AAA==#AAA;\
            \n  test-50: #AAA ==#AAA;\
            \n  test-51: #AAA== #AAA;\
            \n  test-52: #AAA == #AAA;\
            \n  test-53: #AAA==#{itpl};\
            \n  test-54: #AAA ==#{itpl};\
            \n  test-55: #AAA== #{itpl};\
            \n  test-56: #AAA == #{itpl};\
            \n  test-57: #{itpl}==#{itpl};\
            \n  test-58: #{itpl} ==#{itpl};\
            \n  test-59: #{itpl}== #{itpl};\
            \n  test-60: #{itpl} == #{itpl};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: true;\
        \n  test-2: true;\
        \n  test-3: true;\
        \n  test-4: true;\
        \n  test-5: false;\
        \n  test-6: false;\
        \n  test-7: false;\
        \n  test-8: false;\
        \n  test-9: false;\
        \n  test-10: false;\
        \n  test-11: false;\
        \n  test-12: false;\
        \n  test-13: false;\
        \n  test-14: false;\
        \n  test-15: false;\
        \n  test-16: false;\
        \n  test-17: false;\
        \n  test-18: false;\
        \n  test-19: false;\
        \n  test-20: false;\
        \n  test-21: true;\
        \n  test-22: true;\
        \n  test-23: true;\
        \n  test-24: true;\
        \n  test-25: false;\
        \n  test-26: false;\
        \n  test-27: false;\
        \n  test-28: false;\
        \n  test-29: false;\
        \n  test-30: false;\
        \n  test-31: false;\
        \n  test-32: false;\
        \n  test-33: false;\
        \n  test-34: false;\
        \n  test-35: false;\
        \n  test-36: false;\
        \n  test-37: true;\
        \n  test-38: true;\
        \n  test-39: true;\
        \n  test-40: true;\
        \n  test-41: false;\
        \n  test-42: false;\
        \n  test-43: false;\
        \n  test-44: false;\
        \n  test-45: false;\
        \n  test-46: false;\
        \n  test-47: false;\
        \n  test-48: false;\
        \n  test-49: true;\
        \n  test-50: true;\
        \n  test-51: true;\
        \n  test-52: true;\
        \n  test-53: false;\
        \n  test-54: false;\
        \n  test-55: false;\
        \n  test-56: false;\
        \n  test-57: true;\
        \n  test-58: true;\
        \n  test-59: true;\
        \n  test-60: true;\
        \n}\
        \n"
    );
}
