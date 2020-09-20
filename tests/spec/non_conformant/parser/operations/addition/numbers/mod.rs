//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/addition/numbers"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/operations/addition/numbers/pairs.hrx"
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
            \n  test-5: 10+#{10};\
            \n  test-6: 10 +#{10};\
            \n  test-7: 10+ #{10};\
            \n  test-8: 10 + #{10};\
            \n  test-9: 10+1#{0};\
            \n  test-10: 10 +1#{0};\
            \n  test-11: 10+ 1#{0};\
            \n  test-12: 10 + 1#{0};\
            \n  test-13: 10+#{1}0;\
            \n  test-14: 10 +#{1}0;\
            \n  test-15: 10+ #{1}0;\
            \n  test-16: 10 + #{1}0;\
            \n  test-17: #{10}+#{10};\
            \n  test-18: #{10} +#{10};\
            \n  test-19: #{10}+ #{10};\
            \n  test-20: #{10} + #{10};\
            \n  test-21: #{10}+1#{0};\
            \n  test-22: #{10} +1#{0};\
            \n  test-23: #{10}+ 1#{0};\
            \n  test-24: #{10} + 1#{0};\
            \n  test-25: #{10}+#{1}0;\
            \n  test-26: #{10} +#{1}0;\
            \n  test-27: #{10}+ #{1}0;\
            \n  test-28: #{10} + #{1}0;\
            \n  test-29: 1#{0}+1#{0};\
            \n  test-30: 1#{0} +1#{0};\
            \n  test-31: 1#{0}+ 1#{0};\
            \n  test-32: 1#{0} + 1#{0};\
            \n  test-33: 1#{0}+#{1}0;\
            \n  test-34: 1#{0} +#{1}0;\
            \n  test-35: 1#{0}+ #{1}0;\
            \n  test-36: 1#{0} + #{1}0;\
            \n  test-37: #{1}0+#{1}0;\
            \n  test-38: #{1}0 +#{1}0;\
            \n  test-39: #{1}0+ #{1}0;\
            \n  test-40: #{1}0 + #{1}0;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: 20;\
        \n  test-2: 20;\
        \n  test-3: 20;\
        \n  test-4: 20;\
        \n  test-5: 1010;\
        \n  test-6: 1010;\
        \n  test-7: 1010;\
        \n  test-8: 1010;\
        \n  test-9: 11 0;\
        \n  test-10: 11 0;\
        \n  test-11: 11 0;\
        \n  test-12: 11 0;\
        \n  test-13: 1010;\
        \n  test-14: 1010;\
        \n  test-15: 1010;\
        \n  test-16: 1010;\
        \n  test-17: 1010;\
        \n  test-18: 1010;\
        \n  test-19: 1010;\
        \n  test-20: 1010;\
        \n  test-21: 101 0;\
        \n  test-22: 101 0;\
        \n  test-23: 101 0;\
        \n  test-24: 101 0;\
        \n  test-25: 1010;\
        \n  test-26: 1010;\
        \n  test-27: 1010;\
        \n  test-28: 1010;\
        \n  test-29: 1 01 0;\
        \n  test-30: 1 01 0;\
        \n  test-31: 1 01 0;\
        \n  test-32: 1 01 0;\
        \n  test-33: 1 010;\
        \n  test-34: 1 010;\
        \n  test-35: 1 010;\
        \n  test-36: 1 010;\
        \n  test-37: 1010;\
        \n  test-38: 1010;\
        \n  test-39: 1010;\
        \n  test-40: 1010;\
        \n}\
        \n"
    );
}
