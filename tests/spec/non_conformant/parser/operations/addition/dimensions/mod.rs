//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/addition/dimensions"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/operations/addition/dimensions/pairs.hrx"
#[test]
#[ignore] // wrong result
fn pairs() {
    assert_eq!(
        rsass(
            "foo {\
            \n  test-1: 10+10;\
            \n  test-2: 10 +10;\
            \n  test-3: 10+ 10;\
            \n  test-4: 10 + 10;\
            \n  test-5: 10+10px;\
            \n  test-6: 10 +10px;\
            \n  test-7: 10+ 10px;\
            \n  test-8: 10 + 10px;\
            \n  test-9: 10+#{10}px;\
            \n  test-10: 10 +#{10}px;\
            \n  test-11: 10+ #{10}px;\
            \n  test-12: 10 + #{10}px;\
            \n  test-13: 10+1#{0}px;\
            \n  test-14: 10 +1#{0}px;\
            \n  test-15: 10+ 1#{0}px;\
            \n  test-16: 10 + 1#{0}px;\
            \n  test-17: 10+10#{px};\
            \n  test-18: 10 +10#{px};\
            \n  test-19: 10+ 10#{px};\
            \n  test-20: 10 + 10#{px};\
            \n  test-21: 10+10#{p}x;\
            \n  test-22: 10 +10#{p}x;\
            \n  test-23: 10+ 10#{p}x;\
            \n  test-24: 10 + 10#{p}x;\
            \n  test-25: 10px+10px;\
            \n  test-26: 10px +10px;\
            \n  test-27: 10px+ 10px;\
            \n  test-28: 10px + 10px;\
            \n  test-29: 10px+#{10}px;\
            \n  test-30: 10px +#{10}px;\
            \n  test-31: 10px+ #{10}px;\
            \n  test-32: 10px + #{10}px;\
            \n  test-33: 10px+1#{0}px;\
            \n  test-34: 10px +1#{0}px;\
            \n  test-35: 10px+ 1#{0}px;\
            \n  test-36: 10px + 1#{0}px;\
            \n  test-37: 10px+10#{px};\
            \n  test-38: 10px +10#{px};\
            \n  test-39: 10px+ 10#{px};\
            \n  test-40: 10px + 10#{px};\
            \n  test-41: 10px+10#{p}x;\
            \n  test-42: 10px +10#{p}x;\
            \n  test-43: 10px+ 10#{p}x;\
            \n  test-44: 10px + 10#{p}x;\
            \n  test-45: #{10}px+#{10}px;\
            \n  test-46: #{10}px +#{10}px;\
            \n  test-47: #{10}px+ #{10}px;\
            \n  test-48: #{10}px + #{10}px;\
            \n  test-49: #{10}px+1#{0}px;\
            \n  test-50: #{10}px +1#{0}px;\
            \n  test-51: #{10}px+ 1#{0}px;\
            \n  test-52: #{10}px + 1#{0}px;\
            \n  test-53: #{10}px+10#{px};\
            \n  test-54: #{10}px +10#{px};\
            \n  test-55: #{10}px+ 10#{px};\
            \n  test-56: #{10}px + 10#{px};\
            \n  test-57: #{10}px+10#{p}x;\
            \n  test-58: #{10}px +10#{p}x;\
            \n  test-59: #{10}px+ 10#{p}x;\
            \n  test-60: #{10}px + 10#{p}x;\
            \n  test-61: 1#{0}px+1#{0}px;\
            \n  test-62: 1#{0}px +1#{0}px;\
            \n  test-63: 1#{0}px+ 1#{0}px;\
            \n  test-64: 1#{0}px + 1#{0}px;\
            \n  test-65: 1#{0}px+10#{px};\
            \n  test-66: 1#{0}px +10#{px};\
            \n  test-67: 1#{0}px+ 10#{px};\
            \n  test-68: 1#{0}px + 10#{px};\
            \n  test-69: 1#{0}px+10#{p}x;\
            \n  test-70: 1#{0}px +10#{p}x;\
            \n  test-71: 1#{0}px+ 10#{p}x;\
            \n  test-72: 1#{0}px + 10#{p}x;\
            \n  test-73: 10#{px}+10#{px};\
            \n  test-74: 10#{px} +10#{px};\
            \n  test-75: 10#{px}+ 10#{px};\
            \n  test-76: 10#{px} + 10#{px};\
            \n  test-77: 10#{px}+10#{p}x;\
            \n  test-78: 10#{px} +10#{p}x;\
            \n  test-79: 10#{px}+ 10#{p}x;\
            \n  test-80: 10#{px} + 10#{p}x;\
            \n  test-81: 10#{p}x+10#{p}x;\
            \n  test-82: 10#{p}x +10#{p}x;\
            \n  test-83: 10#{p}x+ 10#{p}x;\
            \n  test-84: 10#{p}x + 10#{p}x;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: 20;\
        \n  test-2: 20;\
        \n  test-3: 20;\
        \n  test-4: 20;\
        \n  test-5: 20px;\
        \n  test-6: 20px;\
        \n  test-7: 20px;\
        \n  test-8: 20px;\
        \n  test-9: 1010px;\
        \n  test-10: 1010px;\
        \n  test-11: 1010px;\
        \n  test-12: 1010px;\
        \n  test-13: 11 0px;\
        \n  test-14: 11 0px;\
        \n  test-15: 11 0px;\
        \n  test-16: 11 0px;\
        \n  test-17: 20 px;\
        \n  test-18: 20 px;\
        \n  test-19: 20 px;\
        \n  test-20: 20 px;\
        \n  test-21: 20 px;\
        \n  test-22: 20 px;\
        \n  test-23: 20 px;\
        \n  test-24: 20 px;\
        \n  test-25: 20px;\
        \n  test-26: 20px;\
        \n  test-27: 20px;\
        \n  test-28: 20px;\
        \n  test-29: 10px10px;\
        \n  test-30: 10px10px;\
        \n  test-31: 10px10px;\
        \n  test-32: 10px10px;\
        \n  test-33: 11px 0px;\
        \n  test-34: 11px 0px;\
        \n  test-35: 11px 0px;\
        \n  test-36: 11px 0px;\
        \n  test-37: 20px px;\
        \n  test-38: 20px px;\
        \n  test-39: 20px px;\
        \n  test-40: 20px px;\
        \n  test-41: 20px px;\
        \n  test-42: 20px px;\
        \n  test-43: 20px px;\
        \n  test-44: 20px px;\
        \n  test-45: 10px10px;\
        \n  test-46: 10px10px;\
        \n  test-47: 10px10px;\
        \n  test-48: 10px10px;\
        \n  test-49: 10px1 0px;\
        \n  test-50: 10px1 0px;\
        \n  test-51: 10px1 0px;\
        \n  test-52: 10px1 0px;\
        \n  test-53: 10px10 px;\
        \n  test-54: 10px10 px;\
        \n  test-55: 10px10 px;\
        \n  test-56: 10px10 px;\
        \n  test-57: 10px10 px;\
        \n  test-58: 10px10 px;\
        \n  test-59: 10px10 px;\
        \n  test-60: 10px10 px;\
        \n  test-61: 1 0px1 0px;\
        \n  test-62: 1 0px1 0px;\
        \n  test-63: 1 0px1 0px;\
        \n  test-64: 1 0px1 0px;\
        \n  test-65: 1 0px10 px;\
        \n  test-66: 1 0px10 px;\
        \n  test-67: 1 0px10 px;\
        \n  test-68: 1 0px10 px;\
        \n  test-69: 1 0px10 px;\
        \n  test-70: 1 0px10 px;\
        \n  test-71: 1 0px10 px;\
        \n  test-72: 1 0px10 px;\
        \n  test-73: 10 px10 px;\
        \n  test-74: 10 px10 px;\
        \n  test-75: 10 px10 px;\
        \n  test-76: 10 px10 px;\
        \n  test-77: 10 px10 px;\
        \n  test-78: 10 px10 px;\
        \n  test-79: 10 px10 px;\
        \n  test-80: 10 px10 px;\
        \n  test-81: 10 px10 px;\
        \n  test-82: 10 px10 px;\
        \n  test-83: 10 px10 px;\
        \n  test-84: 10 px10 px;\
        \n}\
        \n"
    );
}
