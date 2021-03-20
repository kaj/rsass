//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/subtract/numbers/pairs.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10-10;\
            \n  test-2: 10 -10;\
            \n  test-3: 10- 10;\
            \n  test-4: 10 - 10;\
            \n  test-5: 10-#{10};\
            \n  test-6: 10 -#{10};\
            \n  test-7: 10- #{10};\
            \n  test-8: 10 - #{10};\
            \n  test-9: 10-1#{0};\
            \n  test-10: 10 -1#{0};\
            \n  test-11: 10- 1#{0};\
            \n  test-12: 10 - 1#{0};\
            \n  test-13: 10-#{1}0;\
            \n  test-14: 10 -#{1}0;\
            \n  test-15: 10- #{1}0;\
            \n  test-16: 10 - #{1}0;\
            \n  test-17: #{10}-#{10};\
            \n  test-18: #{10} -#{10};\
            \n  test-19: #{10}- #{10};\
            \n  test-20: #{10} - #{10};\
            \n  test-21: #{10}-1#{0};\
            \n  test-22: #{10} -1#{0};\
            \n  test-23: #{10}- 1#{0};\
            \n  test-24: #{10} - 1#{0};\
            \n  test-25: #{10}-#{1}0;\
            \n  test-26: #{10} -#{1}0;\
            \n  test-27: #{10}- #{1}0;\
            \n  test-28: #{10} - #{1}0;\
            \n  test-29: 1#{0}-1#{0};\
            \n  test-30: 1#{0} -1#{0};\
            \n  test-31: 1#{0}- 1#{0};\
            \n  test-32: 1#{0} - 1#{0};\
            \n  test-33: 1#{0}-#{1}0;\
            \n  test-34: 1#{0} -#{1}0;\
            \n  test-35: 1#{0}- #{1}0;\
            \n  test-36: 1#{0} - #{1}0;\
            \n  test-37: #{1}0-#{1}0;\
            \n  test-38: #{1}0 -#{1}0;\
            \n  test-39: #{1}0- #{1}0;\
            \n  test-40: #{1}0 - #{1}0;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: 0;\
        \n  test-2: 10 -10;\
        \n  test-3: 0;\
        \n  test-4: 0;\
        \n  test-5: 10 -10;\
        \n  test-6: 10 -10;\
        \n  test-7: 10-10;\
        \n  test-8: 10-10;\
        \n  test-9: 9 0;\
        \n  test-10: 10 -1 0;\
        \n  test-11: 9 0;\
        \n  test-12: 9 0;\
        \n  test-13: 10 -10;\
        \n  test-14: 10 -10;\
        \n  test-15: 10-10;\
        \n  test-16: 10-10;\
        \n  test-17: 10-10;\
        \n  test-18: 10 -10;\
        \n  test-19: 10- 10;\
        \n  test-20: 10-10;\
        \n  test-21: 10-10;\
        \n  test-22: 10 -1 0;\
        \n  test-23: 10- 1 0;\
        \n  test-24: 10-1 0;\
        \n  test-25: 10-10;\
        \n  test-26: 10 -10;\
        \n  test-27: 10- 10;\
        \n  test-28: 10-10;\
        \n  test-29: 1 0-10;\
        \n  test-30: 1 0 -1 0;\
        \n  test-31: 1 0- 1 0;\
        \n  test-32: 1 0-1 0;\
        \n  test-33: 1 0-10;\
        \n  test-34: 1 0 -10;\
        \n  test-35: 1 0- 10;\
        \n  test-36: 1 0-10;\
        \n  test-37: 10-10;\
        \n  test-38: 10 -10;\
        \n  test-39: 10- 10;\
        \n  test-40: 10-10;\
        \n}\
        \n"
    );
}
