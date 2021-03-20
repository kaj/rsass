//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_eq/dimensions/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10==10;\
            \n  test-2: 10 ==10;\
            \n  test-3: 10== 10;\
            \n  test-4: 10 == 10;\
            \n  test-5: 10==10px;\
            \n  test-6: 10 ==10px;\
            \n  test-7: 10== 10px;\
            \n  test-8: 10 == 10px;\
            \n  test-9: 10==#{10}px;\
            \n  test-10: 10 ==#{10}px;\
            \n  test-11: 10== #{10}px;\
            \n  test-12: 10 == #{10}px;\
            \n  test-13: 10==1#{0}px;\
            \n  test-14: 10 ==1#{0}px;\
            \n  test-15: 10== 1#{0}px;\
            \n  test-16: 10 == 1#{0}px;\
            \n  test-17: 10==10#{px};\
            \n  test-18: 10 ==10#{px};\
            \n  test-19: 10== 10#{px};\
            \n  test-20: 10 == 10#{px};\
            \n  test-21: 10==10#{p}x;\
            \n  test-22: 10 ==10#{p}x;\
            \n  test-23: 10== 10#{p}x;\
            \n  test-24: 10 == 10#{p}x;\
            \n  test-25: 10px==10px;\
            \n  test-26: 10px ==10px;\
            \n  test-27: 10px== 10px;\
            \n  test-28: 10px == 10px;\
            \n  test-29: 10px==#{10}px;\
            \n  test-30: 10px ==#{10}px;\
            \n  test-31: 10px== #{10}px;\
            \n  test-32: 10px == #{10}px;\
            \n  test-33: 10px==1#{0}px;\
            \n  test-34: 10px ==1#{0}px;\
            \n  test-35: 10px== 1#{0}px;\
            \n  test-36: 10px == 1#{0}px;\
            \n  test-37: 10px==10#{px};\
            \n  test-38: 10px ==10#{px};\
            \n  test-39: 10px== 10#{px};\
            \n  test-40: 10px == 10#{px};\
            \n  test-41: 10px==10#{p}x;\
            \n  test-42: 10px ==10#{p}x;\
            \n  test-43: 10px== 10#{p}x;\
            \n  test-44: 10px == 10#{p}x;\
            \n  test-45: #{10}px==#{10}px;\
            \n  test-46: #{10}px ==#{10}px;\
            \n  test-47: #{10}px== #{10}px;\
            \n  test-48: #{10}px == #{10}px;\
            \n  test-49: #{10}px==1#{0}px;\
            \n  test-50: #{10}px ==1#{0}px;\
            \n  test-51: #{10}px== 1#{0}px;\
            \n  test-52: #{10}px == 1#{0}px;\
            \n  test-53: #{10}px==10#{px};\
            \n  test-54: #{10}px ==10#{px};\
            \n  test-55: #{10}px== 10#{px};\
            \n  test-56: #{10}px == 10#{px};\
            \n  test-57: #{10}px==10#{p}x;\
            \n  test-58: #{10}px ==10#{p}x;\
            \n  test-59: #{10}px== 10#{p}x;\
            \n  test-60: #{10}px == 10#{p}x;\
            \n  test-61: 1#{0}px==1#{0}px;\
            \n  test-62: 1#{0}px ==1#{0}px;\
            \n  test-63: 1#{0}px== 1#{0}px;\
            \n  test-64: 1#{0}px == 1#{0}px;\
            \n  test-65: 1#{0}px==10#{px};\
            \n  test-66: 1#{0}px ==10#{px};\
            \n  test-67: 1#{0}px== 10#{px};\
            \n  test-68: 1#{0}px == 10#{px};\
            \n  test-69: 1#{0}px==10#{p}x;\
            \n  test-70: 1#{0}px ==10#{p}x;\
            \n  test-71: 1#{0}px== 10#{p}x;\
            \n  test-72: 1#{0}px == 10#{p}x;\
            \n  test-73: 10#{px}==10#{px};\
            \n  test-74: 10#{px} ==10#{px};\
            \n  test-75: 10#{px}== 10#{px};\
            \n  test-76: 10#{px} == 10#{px};\
            \n  test-77: 10#{px}==10#{p}x;\
            \n  test-78: 10#{px} ==10#{p}x;\
            \n  test-79: 10#{px}== 10#{p}x;\
            \n  test-80: 10#{px} == 10#{p}x;\
            \n  test-81: 10#{p}x==10#{p}x;\
            \n  test-82: 10#{p}x ==10#{p}x;\
            \n  test-83: 10#{p}x== 10#{p}x;\
            \n  test-84: 10#{p}x == 10#{p}x;\
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
        \n  test-13: false 0px;\
        \n  test-14: false 0px;\
        \n  test-15: false 0px;\
        \n  test-16: false 0px;\
        \n  test-17: true px;\
        \n  test-18: true px;\
        \n  test-19: true px;\
        \n  test-20: true px;\
        \n  test-21: true px;\
        \n  test-22: true px;\
        \n  test-23: true px;\
        \n  test-24: true px;\
        \n  test-25: true;\
        \n  test-26: true;\
        \n  test-27: true;\
        \n  test-28: true;\
        \n  test-29: false;\
        \n  test-30: false;\
        \n  test-31: false;\
        \n  test-32: false;\
        \n  test-33: false 0px;\
        \n  test-34: false 0px;\
        \n  test-35: false 0px;\
        \n  test-36: false 0px;\
        \n  test-37: false px;\
        \n  test-38: false px;\
        \n  test-39: false px;\
        \n  test-40: false px;\
        \n  test-41: false px;\
        \n  test-42: false px;\
        \n  test-43: false px;\
        \n  test-44: false px;\
        \n  test-45: true;\
        \n  test-46: true;\
        \n  test-47: true;\
        \n  test-48: true;\
        \n  test-49: false 0px;\
        \n  test-50: false 0px;\
        \n  test-51: false 0px;\
        \n  test-52: false 0px;\
        \n  test-53: false px;\
        \n  test-54: false px;\
        \n  test-55: false px;\
        \n  test-56: false px;\
        \n  test-57: false px;\
        \n  test-58: false px;\
        \n  test-59: false px;\
        \n  test-60: false px;\
        \n  test-61: 1 false 0px;\
        \n  test-62: 1 false 0px;\
        \n  test-63: 1 false 0px;\
        \n  test-64: 1 false 0px;\
        \n  test-65: 1 false px;\
        \n  test-66: 1 false px;\
        \n  test-67: 1 false px;\
        \n  test-68: 1 false px;\
        \n  test-69: 1 false px;\
        \n  test-70: 1 false px;\
        \n  test-71: 1 false px;\
        \n  test-72: 1 false px;\
        \n  test-73: 10 false px;\
        \n  test-74: 10 false px;\
        \n  test-75: 10 false px;\
        \n  test-76: 10 false px;\
        \n  test-77: 10 false px;\
        \n  test-78: 10 false px;\
        \n  test-79: 10 false px;\
        \n  test-80: 10 false px;\
        \n  test-81: 10 false px;\
        \n  test-82: 10 false px;\
        \n  test-83: 10 false px;\
        \n  test-84: 10 false px;\
        \n}\
        \n"
    );
}
