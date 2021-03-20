//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/logic_ne/strings/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: literal!=literal;\
            \n  test-2: literal !=literal;\
            \n  test-3: literal!= literal;\
            \n  test-4: literal != literal;\
            \n  test-5: literal!=\"quoted\";\
            \n  test-6: literal !=\"quoted\";\
            \n  test-7: literal!= \"quoted\";\
            \n  test-8: literal != \"quoted\";\
            \n  test-9: literal!=#{interpolant};\
            \n  test-10: literal !=#{interpolant};\
            \n  test-11: literal!= #{interpolant};\
            \n  test-12: literal != #{interpolant};\
            \n  test-13: literal!=lschema_#{ritlp};\
            \n  test-14: literal !=lschema_#{ritlp};\
            \n  test-15: literal!= lschema_#{ritlp};\
            \n  test-16: literal != lschema_#{ritlp};\
            \n  test-17: literal!=#{litlp}_rschema;\
            \n  test-18: literal !=#{litlp}_rschema;\
            \n  test-19: literal!= #{litlp}_rschema;\
            \n  test-20: literal != #{litlp}_rschema;\
            \n  test-21: \"quoted\"!=\"quoted\";\
            \n  test-22: \"quoted\" !=\"quoted\";\
            \n  test-23: \"quoted\"!= \"quoted\";\
            \n  test-24: \"quoted\" != \"quoted\";\
            \n  test-25: \"quoted\"!=#{interpolant};\
            \n  test-26: \"quoted\" !=#{interpolant};\
            \n  test-27: \"quoted\"!= #{interpolant};\
            \n  test-28: \"quoted\" != #{interpolant};\
            \n  test-29: \"quoted\"!=lschema_#{ritlp};\
            \n  test-30: \"quoted\" !=lschema_#{ritlp};\
            \n  test-31: \"quoted\"!= lschema_#{ritlp};\
            \n  test-32: \"quoted\" != lschema_#{ritlp};\
            \n  test-33: \"quoted\"!=#{litlp}_rschema;\
            \n  test-34: \"quoted\" !=#{litlp}_rschema;\
            \n  test-35: \"quoted\"!= #{litlp}_rschema;\
            \n  test-36: \"quoted\" != #{litlp}_rschema;\
            \n  test-37: #{interpolant}!=#{interpolant};\
            \n  test-38: #{interpolant} !=#{interpolant};\
            \n  test-39: #{interpolant}!= #{interpolant};\
            \n  test-40: #{interpolant} != #{interpolant};\
            \n  test-41: #{interpolant}!=lschema_#{ritlp};\
            \n  test-42: #{interpolant} !=lschema_#{ritlp};\
            \n  test-43: #{interpolant}!= lschema_#{ritlp};\
            \n  test-44: #{interpolant} != lschema_#{ritlp};\
            \n  test-45: #{interpolant}!=#{litlp}_rschema;\
            \n  test-46: #{interpolant} !=#{litlp}_rschema;\
            \n  test-47: #{interpolant}!= #{litlp}_rschema;\
            \n  test-48: #{interpolant} != #{litlp}_rschema;\
            \n  test-49: lschema_#{ritlp}!=lschema_#{ritlp};\
            \n  test-50: lschema_#{ritlp} !=lschema_#{ritlp};\
            \n  test-51: lschema_#{ritlp}!= lschema_#{ritlp};\
            \n  test-52: lschema_#{ritlp} != lschema_#{ritlp};\
            \n  test-53: lschema_#{ritlp}!=#{litlp}_rschema;\
            \n  test-54: lschema_#{ritlp} !=#{litlp}_rschema;\
            \n  test-55: lschema_#{ritlp}!= #{litlp}_rschema;\
            \n  test-56: lschema_#{ritlp} != #{litlp}_rschema;\
            \n  test-57: #{litlp}_rschema!=#{litlp}_rschema;\
            \n  test-58: #{litlp}_rschema !=#{litlp}_rschema;\
            \n  test-59: #{litlp}_rschema!= #{litlp}_rschema;\
            \n  test-60: #{litlp}_rschema != #{litlp}_rschema;\
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
