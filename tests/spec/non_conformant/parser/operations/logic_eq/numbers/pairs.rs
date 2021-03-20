//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_eq/numbers/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10==10;\
            \n  test-2: 10 ==10;\
            \n  test-3: 10== 10;\
            \n  test-4: 10 == 10;\
            \n  test-5: 10==#{10};\
            \n  test-6: 10 ==#{10};\
            \n  test-7: 10== #{10};\
            \n  test-8: 10 == #{10};\
            \n  test-9: 10==1#{0};\
            \n  test-10: 10 ==1#{0};\
            \n  test-11: 10== 1#{0};\
            \n  test-12: 10 == 1#{0};\
            \n  test-13: 10==#{1}0;\
            \n  test-14: 10 ==#{1}0;\
            \n  test-15: 10== #{1}0;\
            \n  test-16: 10 == #{1}0;\
            \n  test-17: #{10}==#{10};\
            \n  test-18: #{10} ==#{10};\
            \n  test-19: #{10}== #{10};\
            \n  test-20: #{10} == #{10};\
            \n  test-21: #{10}==1#{0};\
            \n  test-22: #{10} ==1#{0};\
            \n  test-23: #{10}== 1#{0};\
            \n  test-24: #{10} == 1#{0};\
            \n  test-25: #{10}==#{1}0;\
            \n  test-26: #{10} ==#{1}0;\
            \n  test-27: #{10}== #{1}0;\
            \n  test-28: #{10} == #{1}0;\
            \n  test-29: 1#{0}==1#{0};\
            \n  test-30: 1#{0} ==1#{0};\
            \n  test-31: 1#{0}== 1#{0};\
            \n  test-32: 1#{0} == 1#{0};\
            \n  test-33: 1#{0}==#{1}0;\
            \n  test-34: 1#{0} ==#{1}0;\
            \n  test-35: 1#{0}== #{1}0;\
            \n  test-36: 1#{0} == #{1}0;\
            \n  test-37: #{1}0==#{1}0;\
            \n  test-38: #{1}0 ==#{1}0;\
            \n  test-39: #{1}0== #{1}0;\
            \n  test-40: #{1}0 == #{1}0;\
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
        \n  test-9: false 0;\
        \n  test-10: false 0;\
        \n  test-11: false 0;\
        \n  test-12: false 0;\
        \n  test-13: false;\
        \n  test-14: false;\
        \n  test-15: false;\
        \n  test-16: false;\
        \n  test-17: true;\
        \n  test-18: true;\
        \n  test-19: true;\
        \n  test-20: true;\
        \n  test-21: false 0;\
        \n  test-22: false 0;\
        \n  test-23: false 0;\
        \n  test-24: false 0;\
        \n  test-25: true;\
        \n  test-26: true;\
        \n  test-27: true;\
        \n  test-28: true;\
        \n  test-29: 1 false 0;\
        \n  test-30: 1 false 0;\
        \n  test-31: 1 false 0;\
        \n  test-32: 1 false 0;\
        \n  test-33: 1 false;\
        \n  test-34: 1 false;\
        \n  test-35: 1 false;\
        \n  test-36: 1 false;\
        \n  test-37: true;\
        \n  test-38: true;\
        \n  test-39: true;\
        \n  test-40: true;\
        \n}\
        \n"
    );
}
