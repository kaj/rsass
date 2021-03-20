//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/subtract/strings/pairs.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: literal-literal;\
            \n  test-2: literal -literal;\
            \n  test-3: literal- literal;\
            \n  test-4: literal - literal;\
            \n  test-5: literal-\"quoted\";\
            \n  test-6: literal -\"quoted\";\
            \n  test-7: literal- \"quoted\";\
            \n  test-8: literal - \"quoted\";\
            \n  test-9: literal-#{interpolant};\
            \n  test-10: literal -#{interpolant};\
            \n  test-11: literal- #{interpolant};\
            \n  test-12: literal - #{interpolant};\
            \n  test-13: literal-lschema_#{ritlp};\
            \n  test-14: literal -lschema_#{ritlp};\
            \n  test-15: literal- lschema_#{ritlp};\
            \n  test-16: literal - lschema_#{ritlp};\
            \n  test-17: literal-#{litlp}_rschema;\
            \n  test-18: literal -#{litlp}_rschema;\
            \n  test-19: literal- #{litlp}_rschema;\
            \n  test-20: literal - #{litlp}_rschema;\
            \n  test-21: \"quoted\"-\"quoted\";\
            \n  test-22: \"quoted\" -\"quoted\";\
            \n  test-23: \"quoted\"- \"quoted\";\
            \n  test-24: \"quoted\" - \"quoted\";\
            \n  test-25: \"quoted\"-#{interpolant};\
            \n  test-26: \"quoted\" -#{interpolant};\
            \n  test-27: \"quoted\"- #{interpolant};\
            \n  test-28: \"quoted\" - #{interpolant};\
            \n  test-29: \"quoted\"-lschema_#{ritlp};\
            \n  test-30: \"quoted\" -lschema_#{ritlp};\
            \n  test-31: \"quoted\"- lschema_#{ritlp};\
            \n  test-32: \"quoted\" - lschema_#{ritlp};\
            \n  test-33: \"quoted\"-#{litlp}_rschema;\
            \n  test-34: \"quoted\" -#{litlp}_rschema;\
            \n  test-35: \"quoted\"- #{litlp}_rschema;\
            \n  test-36: \"quoted\" - #{litlp}_rschema;\
            \n  test-37: #{interpolant}-#{interpolant};\
            \n  test-38: #{interpolant} -#{interpolant};\
            \n  test-39: #{interpolant}- #{interpolant};\
            \n  test-40: #{interpolant} - #{interpolant};\
            \n  test-41: #{interpolant}-lschema_#{ritlp};\
            \n  test-42: #{interpolant} -lschema_#{ritlp};\
            \n  test-43: #{interpolant}- lschema_#{ritlp};\
            \n  test-44: #{interpolant} - lschema_#{ritlp};\
            \n  test-45: #{interpolant}-#{litlp}_rschema;\
            \n  test-46: #{interpolant} -#{litlp}_rschema;\
            \n  test-47: #{interpolant}- #{litlp}_rschema;\
            \n  test-48: #{interpolant} - #{litlp}_rschema;\
            \n  test-49: lschema_#{ritlp}-lschema_#{ritlp};\
            \n  test-50: lschema_#{ritlp} -lschema_#{ritlp};\
            \n  test-51: lschema_#{ritlp}- lschema_#{ritlp};\
            \n  test-52: lschema_#{ritlp} - lschema_#{ritlp};\
            \n  test-53: lschema_#{ritlp}-#{litlp}_rschema;\
            \n  test-54: lschema_#{ritlp} -#{litlp}_rschema;\
            \n  test-55: lschema_#{ritlp}- #{litlp}_rschema;\
            \n  test-56: lschema_#{ritlp} - #{litlp}_rschema;\
            \n  test-57: #{litlp}_rschema-#{litlp}_rschema;\
            \n  test-58: #{litlp}_rschema -#{litlp}_rschema;\
            \n  test-59: #{litlp}_rschema- #{litlp}_rschema;\
            \n  test-60: #{litlp}_rschema - #{litlp}_rschema;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: literal-literal;\
        \n  test-2: literal -literal;\
        \n  test-3: literal- literal;\
        \n  test-4: literal-literal;\
        \n  test-5: literal- \"quoted\";\
        \n  test-6: literal-\"quoted\";\
        \n  test-7: literal- \"quoted\";\
        \n  test-8: literal-\"quoted\";\
        \n  test-9: literal-interpolant;\
        \n  test-10: literal -interpolant;\
        \n  test-11: literal- interpolant;\
        \n  test-12: literal-interpolant;\
        \n  test-13: literal-lschema_ritlp;\
        \n  test-14: literal -lschema_ritlp;\
        \n  test-15: literal- lschema_ritlp;\
        \n  test-16: literal-lschema_ritlp;\
        \n  test-17: literal-litlp_rschema;\
        \n  test-18: literal -litlp_rschema;\
        \n  test-19: literal- litlp_rschema;\
        \n  test-20: literal-litlp_rschema;\
        \n  test-21: \"quoted\"-\"quoted\";\
        \n  test-22: \"quoted\"-\"quoted\";\
        \n  test-23: \"quoted\"-\"quoted\";\
        \n  test-24: \"quoted\"-\"quoted\";\
        \n  test-25: \"quoted\" -interpolant;\
        \n  test-26: \"quoted\" -interpolant;\
        \n  test-27: \"quoted\"-interpolant;\
        \n  test-28: \"quoted\"-interpolant;\
        \n  test-29: \"quoted\" -lschema_ritlp;\
        \n  test-30: \"quoted\" -lschema_ritlp;\
        \n  test-31: \"quoted\"-lschema_ritlp;\
        \n  test-32: \"quoted\"-lschema_ritlp;\
        \n  test-33: \"quoted\" -litlp_rschema;\
        \n  test-34: \"quoted\" -litlp_rschema;\
        \n  test-35: \"quoted\"-litlp_rschema;\
        \n  test-36: \"quoted\"-litlp_rschema;\
        \n  test-37: interpolant-interpolant;\
        \n  test-38: interpolant -interpolant;\
        \n  test-39: interpolant- interpolant;\
        \n  test-40: interpolant-interpolant;\
        \n  test-41: interpolant-lschema_ritlp;\
        \n  test-42: interpolant -lschema_ritlp;\
        \n  test-43: interpolant- lschema_ritlp;\
        \n  test-44: interpolant-lschema_ritlp;\
        \n  test-45: interpolant-litlp_rschema;\
        \n  test-46: interpolant -litlp_rschema;\
        \n  test-47: interpolant- litlp_rschema;\
        \n  test-48: interpolant-litlp_rschema;\
        \n  test-49: lschema_ritlp-lschema_ritlp;\
        \n  test-50: lschema_ritlp -lschema_ritlp;\
        \n  test-51: lschema_ritlp- lschema_ritlp;\
        \n  test-52: lschema_ritlp-lschema_ritlp;\
        \n  test-53: lschema_ritlp-litlp_rschema;\
        \n  test-54: lschema_ritlp -litlp_rschema;\
        \n  test-55: lschema_ritlp- litlp_rschema;\
        \n  test-56: lschema_ritlp-litlp_rschema;\
        \n  test-57: litlp_rschema-litlp_rschema;\
        \n  test-58: litlp_rschema -litlp_rschema;\
        \n  test-59: litlp_rschema- litlp_rschema;\
        \n  test-60: litlp_rschema-litlp_rschema;\
        \n}\
        \n"
    );
}
