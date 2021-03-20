//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_ne/dimensions/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10!=10;\
            \n  test-2: 10 !=10;\
            \n  test-3: 10!= 10;\
            \n  test-4: 10 != 10;\
            \n  test-5: 10!=10px;\
            \n  test-6: 10 !=10px;\
            \n  test-7: 10!= 10px;\
            \n  test-8: 10 != 10px;\
            \n  test-9: 10!=#{10}px;\
            \n  test-10: 10 !=#{10}px;\
            \n  test-11: 10!= #{10}px;\
            \n  test-12: 10 != #{10}px;\
            \n  test-13: 10!=1#{0}px;\
            \n  test-14: 10 !=1#{0}px;\
            \n  test-15: 10!= 1#{0}px;\
            \n  test-16: 10 != 1#{0}px;\
            \n  test-17: 10!=10#{px};\
            \n  test-18: 10 !=10#{px};\
            \n  test-19: 10!= 10#{px};\
            \n  test-20: 10 != 10#{px};\
            \n  test-21: 10!=10#{p}x;\
            \n  test-22: 10 !=10#{p}x;\
            \n  test-23: 10!= 10#{p}x;\
            \n  test-24: 10 != 10#{p}x;\
            \n  test-25: 10px!=10px;\
            \n  test-26: 10px !=10px;\
            \n  test-27: 10px!= 10px;\
            \n  test-28: 10px != 10px;\
            \n  test-29: 10px!=#{10}px;\
            \n  test-30: 10px !=#{10}px;\
            \n  test-31: 10px!= #{10}px;\
            \n  test-32: 10px != #{10}px;\
            \n  test-33: 10px!=1#{0}px;\
            \n  test-34: 10px !=1#{0}px;\
            \n  test-35: 10px!= 1#{0}px;\
            \n  test-36: 10px != 1#{0}px;\
            \n  test-37: 10px!=10#{px};\
            \n  test-38: 10px !=10#{px};\
            \n  test-39: 10px!= 10#{px};\
            \n  test-40: 10px != 10#{px};\
            \n  test-41: 10px!=10#{p}x;\
            \n  test-42: 10px !=10#{p}x;\
            \n  test-43: 10px!= 10#{p}x;\
            \n  test-44: 10px != 10#{p}x;\
            \n  test-45: #{10}px!=#{10}px;\
            \n  test-46: #{10}px !=#{10}px;\
            \n  test-47: #{10}px!= #{10}px;\
            \n  test-48: #{10}px != #{10}px;\
            \n  test-49: #{10}px!=1#{0}px;\
            \n  test-50: #{10}px !=1#{0}px;\
            \n  test-51: #{10}px!= 1#{0}px;\
            \n  test-52: #{10}px != 1#{0}px;\
            \n  test-53: #{10}px!=10#{px};\
            \n  test-54: #{10}px !=10#{px};\
            \n  test-55: #{10}px!= 10#{px};\
            \n  test-56: #{10}px != 10#{px};\
            \n  test-57: #{10}px!=10#{p}x;\
            \n  test-58: #{10}px !=10#{p}x;\
            \n  test-59: #{10}px!= 10#{p}x;\
            \n  test-60: #{10}px != 10#{p}x;\
            \n  test-61: 1#{0}px!=1#{0}px;\
            \n  test-62: 1#{0}px !=1#{0}px;\
            \n  test-63: 1#{0}px!= 1#{0}px;\
            \n  test-64: 1#{0}px != 1#{0}px;\
            \n  test-65: 1#{0}px!=10#{px};\
            \n  test-66: 1#{0}px !=10#{px};\
            \n  test-67: 1#{0}px!= 10#{px};\
            \n  test-68: 1#{0}px != 10#{px};\
            \n  test-69: 1#{0}px!=10#{p}x;\
            \n  test-70: 1#{0}px !=10#{p}x;\
            \n  test-71: 1#{0}px!= 10#{p}x;\
            \n  test-72: 1#{0}px != 10#{p}x;\
            \n  test-73: 10#{px}!=10#{px};\
            \n  test-74: 10#{px} !=10#{px};\
            \n  test-75: 10#{px}!= 10#{px};\
            \n  test-76: 10#{px} != 10#{px};\
            \n  test-77: 10#{px}!=10#{p}x;\
            \n  test-78: 10#{px} !=10#{p}x;\
            \n  test-79: 10#{px}!= 10#{p}x;\
            \n  test-80: 10#{px} != 10#{p}x;\
            \n  test-81: 10#{p}x!=10#{p}x;\
            \n  test-82: 10#{p}x !=10#{p}x;\
            \n  test-83: 10#{p}x!= 10#{p}x;\
            \n  test-84: 10#{p}x != 10#{p}x;\
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
        \n  test-13: true 0px;\
        \n  test-14: true 0px;\
        \n  test-15: true 0px;\
        \n  test-16: true 0px;\
        \n  test-17: false px;\
        \n  test-18: false px;\
        \n  test-19: false px;\
        \n  test-20: false px;\
        \n  test-21: false px;\
        \n  test-22: false px;\
        \n  test-23: false px;\
        \n  test-24: false px;\
        \n  test-25: false;\
        \n  test-26: false;\
        \n  test-27: false;\
        \n  test-28: false;\
        \n  test-29: true;\
        \n  test-30: true;\
        \n  test-31: true;\
        \n  test-32: true;\
        \n  test-33: true 0px;\
        \n  test-34: true 0px;\
        \n  test-35: true 0px;\
        \n  test-36: true 0px;\
        \n  test-37: true px;\
        \n  test-38: true px;\
        \n  test-39: true px;\
        \n  test-40: true px;\
        \n  test-41: true px;\
        \n  test-42: true px;\
        \n  test-43: true px;\
        \n  test-44: true px;\
        \n  test-45: false;\
        \n  test-46: false;\
        \n  test-47: false;\
        \n  test-48: false;\
        \n  test-49: true 0px;\
        \n  test-50: true 0px;\
        \n  test-51: true 0px;\
        \n  test-52: true 0px;\
        \n  test-53: true px;\
        \n  test-54: true px;\
        \n  test-55: true px;\
        \n  test-56: true px;\
        \n  test-57: true px;\
        \n  test-58: true px;\
        \n  test-59: true px;\
        \n  test-60: true px;\
        \n  test-61: 1 true 0px;\
        \n  test-62: 1 true 0px;\
        \n  test-63: 1 true 0px;\
        \n  test-64: 1 true 0px;\
        \n  test-65: 1 true px;\
        \n  test-66: 1 true px;\
        \n  test-67: 1 true px;\
        \n  test-68: 1 true px;\
        \n  test-69: 1 true px;\
        \n  test-70: 1 true px;\
        \n  test-71: 1 true px;\
        \n  test-72: 1 true px;\
        \n  test-73: 10 true px;\
        \n  test-74: 10 true px;\
        \n  test-75: 10 true px;\
        \n  test-76: 10 true px;\
        \n  test-77: 10 true px;\
        \n  test-78: 10 true px;\
        \n  test-79: 10 true px;\
        \n  test-80: 10 true px;\
        \n  test-81: 10 true px;\
        \n  test-82: 10 true px;\
        \n  test-83: 10 true px;\
        \n  test-84: 10 true px;\
        \n}\
        \n"
    );
}
