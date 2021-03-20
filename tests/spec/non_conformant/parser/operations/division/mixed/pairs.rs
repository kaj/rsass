//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/division/mixed/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: 10/10;\
            \n  test-2: 10 /10;\
            \n  test-3: 10/ 10;\
            \n  test-4: 10 / 10;\
            \n  test-5: 10/10%;\
            \n  test-6: 10 /10%;\
            \n  test-7: 10/ 10%;\
            \n  test-8: 10 / 10%;\
            \n  test-9: 10/10px;\
            \n  test-10: 10 /10px;\
            \n  test-11: 10/ 10px;\
            \n  test-12: 10 / 10px;\
            \n  test-13: 10/#AAA;\
            \n  test-14: 10 /#AAA;\
            \n  test-15: 10/ #AAA;\
            \n  test-16: 10 / #AAA;\
            \n  test-17: 10/#{itpl};\
            \n  test-18: 10 /#{itpl};\
            \n  test-19: 10/ #{itpl};\
            \n  test-20: 10 / #{itpl};\
            \n  test-21: 10%/10%;\
            \n  test-22: 10% /10%;\
            \n  test-23: 10%/ 10%;\
            \n  test-24: 10% / 10%;\
            \n  test-25: 10%/10px;\
            \n  test-26: 10% /10px;\
            \n  test-27: 10%/ 10px;\
            \n  test-28: 10% / 10px;\
            \n  test-29: 10%/#AAA;\
            \n  test-30: 10% /#AAA;\
            \n  test-31: 10%/ #AAA;\
            \n  test-32: 10% / #AAA;\
            \n  test-33: 10%/#{itpl};\
            \n  test-34: 10% /#{itpl};\
            \n  test-35: 10%/ #{itpl};\
            \n  test-36: 10% / #{itpl};\
            \n  test-37: 10px/10px;\
            \n  test-38: 10px /10px;\
            \n  test-39: 10px/ 10px;\
            \n  test-40: 10px / 10px;\
            \n  test-41: 10px/#AAA;\
            \n  test-42: 10px /#AAA;\
            \n  test-43: 10px/ #AAA;\
            \n  test-44: 10px / #AAA;\
            \n  test-45: 10px/#{itpl};\
            \n  test-46: 10px /#{itpl};\
            \n  test-47: 10px/ #{itpl};\
            \n  test-48: 10px / #{itpl};\
            \n  test-49: #AAA/#{itpl};\
            \n  test-50: #AAA /#{itpl};\
            \n  test-51: #AAA/ #{itpl};\
            \n  test-52: #AAA / #{itpl};\
            \n  test-53: #{itpl}/#{itpl};\
            \n  test-54: #{itpl} /#{itpl};\
            \n  test-55: #{itpl}/ #{itpl};\
            \n  test-56: #{itpl} / #{itpl};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: 10/10;\
        \n  test-2: 10/10;\
        \n  test-3: 10/10;\
        \n  test-4: 10/10;\
        \n  test-5: 10/10%;\
        \n  test-6: 10/10%;\
        \n  test-7: 10/10%;\
        \n  test-8: 10/10%;\
        \n  test-9: 10/10px;\
        \n  test-10: 10/10px;\
        \n  test-11: 10/10px;\
        \n  test-12: 10/10px;\
        \n  test-13: 10/#AAA;\
        \n  test-14: 10/#AAA;\
        \n  test-15: 10/#AAA;\
        \n  test-16: 10/#AAA;\
        \n  test-17: 10/itpl;\
        \n  test-18: 10/itpl;\
        \n  test-19: 10/itpl;\
        \n  test-20: 10/itpl;\
        \n  test-21: 10%/10%;\
        \n  test-22: 10%/10%;\
        \n  test-23: 10%/10%;\
        \n  test-24: 10%/10%;\
        \n  test-25: 10%/10px;\
        \n  test-26: 10%/10px;\
        \n  test-27: 10%/10px;\
        \n  test-28: 10%/10px;\
        \n  test-29: 10%/#AAA;\
        \n  test-30: 10%/#AAA;\
        \n  test-31: 10%/#AAA;\
        \n  test-32: 10%/#AAA;\
        \n  test-33: 10%/itpl;\
        \n  test-34: 10%/itpl;\
        \n  test-35: 10%/itpl;\
        \n  test-36: 10%/itpl;\
        \n  test-37: 10px/10px;\
        \n  test-38: 10px/10px;\
        \n  test-39: 10px/10px;\
        \n  test-40: 10px/10px;\
        \n  test-41: 10px/#AAA;\
        \n  test-42: 10px/#AAA;\
        \n  test-43: 10px/#AAA;\
        \n  test-44: 10px/#AAA;\
        \n  test-45: 10px/itpl;\
        \n  test-46: 10px/itpl;\
        \n  test-47: 10px/itpl;\
        \n  test-48: 10px/itpl;\
        \n  test-49: #AAA/itpl;\
        \n  test-50: #AAA/itpl;\
        \n  test-51: #AAA/itpl;\
        \n  test-52: #AAA/itpl;\
        \n  test-53: itpl/itpl;\
        \n  test-54: itpl/itpl;\
        \n  test-55: itpl/itpl;\
        \n  test-56: itpl/itpl;\
        \n}\
        \n"
    );
}
