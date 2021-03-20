//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_eq/strings/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: literal==literal;\
            \n  test-2: literal ==literal;\
            \n  test-3: literal== literal;\
            \n  test-4: literal == literal;\
            \n  test-5: literal==\"quoted\";\
            \n  test-6: literal ==\"quoted\";\
            \n  test-7: literal== \"quoted\";\
            \n  test-8: literal == \"quoted\";\
            \n  test-9: literal==#{interpolant};\
            \n  test-10: literal ==#{interpolant};\
            \n  test-11: literal== #{interpolant};\
            \n  test-12: literal == #{interpolant};\
            \n  test-13: literal==lschema_#{ritlp};\
            \n  test-14: literal ==lschema_#{ritlp};\
            \n  test-15: literal== lschema_#{ritlp};\
            \n  test-16: literal == lschema_#{ritlp};\
            \n  test-17: literal==#{litlp}_rschema;\
            \n  test-18: literal ==#{litlp}_rschema;\
            \n  test-19: literal== #{litlp}_rschema;\
            \n  test-20: literal == #{litlp}_rschema;\
            \n  test-21: \"quoted\"==\"quoted\";\
            \n  test-22: \"quoted\" ==\"quoted\";\
            \n  test-23: \"quoted\"== \"quoted\";\
            \n  test-24: \"quoted\" == \"quoted\";\
            \n  test-25: \"quoted\"==#{interpolant};\
            \n  test-26: \"quoted\" ==#{interpolant};\
            \n  test-27: \"quoted\"== #{interpolant};\
            \n  test-28: \"quoted\" == #{interpolant};\
            \n  test-29: \"quoted\"==lschema_#{ritlp};\
            \n  test-30: \"quoted\" ==lschema_#{ritlp};\
            \n  test-31: \"quoted\"== lschema_#{ritlp};\
            \n  test-32: \"quoted\" == lschema_#{ritlp};\
            \n  test-33: \"quoted\"==#{litlp}_rschema;\
            \n  test-34: \"quoted\" ==#{litlp}_rschema;\
            \n  test-35: \"quoted\"== #{litlp}_rschema;\
            \n  test-36: \"quoted\" == #{litlp}_rschema;\
            \n  test-37: #{interpolant}==#{interpolant};\
            \n  test-38: #{interpolant} ==#{interpolant};\
            \n  test-39: #{interpolant}== #{interpolant};\
            \n  test-40: #{interpolant} == #{interpolant};\
            \n  test-41: #{interpolant}==lschema_#{ritlp};\
            \n  test-42: #{interpolant} ==lschema_#{ritlp};\
            \n  test-43: #{interpolant}== lschema_#{ritlp};\
            \n  test-44: #{interpolant} == lschema_#{ritlp};\
            \n  test-45: #{interpolant}==#{litlp}_rschema;\
            \n  test-46: #{interpolant} ==#{litlp}_rschema;\
            \n  test-47: #{interpolant}== #{litlp}_rschema;\
            \n  test-48: #{interpolant} == #{litlp}_rschema;\
            \n  test-49: lschema_#{ritlp}==lschema_#{ritlp};\
            \n  test-50: lschema_#{ritlp} ==lschema_#{ritlp};\
            \n  test-51: lschema_#{ritlp}== lschema_#{ritlp};\
            \n  test-52: lschema_#{ritlp} == lschema_#{ritlp};\
            \n  test-53: lschema_#{ritlp}==#{litlp}_rschema;\
            \n  test-54: lschema_#{ritlp} ==#{litlp}_rschema;\
            \n  test-55: lschema_#{ritlp}== #{litlp}_rschema;\
            \n  test-56: lschema_#{ritlp} == #{litlp}_rschema;\
            \n  test-57: #{litlp}_rschema==#{litlp}_rschema;\
            \n  test-58: #{litlp}_rschema ==#{litlp}_rschema;\
            \n  test-59: #{litlp}_rschema== #{litlp}_rschema;\
            \n  test-60: #{litlp}_rschema == #{litlp}_rschema;\
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
