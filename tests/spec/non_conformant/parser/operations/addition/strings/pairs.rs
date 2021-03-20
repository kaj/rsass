//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/operations/addition/strings/pairs.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  test-1: literal+literal;\
            \n  test-2: literal +literal;\
            \n  test-3: literal+ literal;\
            \n  test-4: literal + literal;\
            \n  test-5: literal+\"quoted\";\
            \n  test-6: literal +\"quoted\";\
            \n  test-7: literal+ \"quoted\";\
            \n  test-8: literal + \"quoted\";\
            \n  test-9: literal+#{interpolant};\
            \n  test-10: literal +#{interpolant};\
            \n  test-11: literal+ #{interpolant};\
            \n  test-12: literal + #{interpolant};\
            \n  test-13: literal+lschema_#{ritlp};\
            \n  test-14: literal +lschema_#{ritlp};\
            \n  test-15: literal+ lschema_#{ritlp};\
            \n  test-16: literal + lschema_#{ritlp};\
            \n  test-17: literal+#{litlp}_rschema;\
            \n  test-18: literal +#{litlp}_rschema;\
            \n  test-19: literal+ #{litlp}_rschema;\
            \n  test-20: literal + #{litlp}_rschema;\
            \n  test-21: \"quoted\"+\"quoted\";\
            \n  test-22: \"quoted\" +\"quoted\";\
            \n  test-23: \"quoted\"+ \"quoted\";\
            \n  test-24: \"quoted\" + \"quoted\";\
            \n  test-25: \"quoted\"+#{interpolant};\
            \n  test-26: \"quoted\" +#{interpolant};\
            \n  test-27: \"quoted\"+ #{interpolant};\
            \n  test-28: \"quoted\" + #{interpolant};\
            \n  test-29: \"quoted\"+lschema_#{ritlp};\
            \n  test-30: \"quoted\" +lschema_#{ritlp};\
            \n  test-31: \"quoted\"+ lschema_#{ritlp};\
            \n  test-32: \"quoted\" + lschema_#{ritlp};\
            \n  test-33: \"quoted\"+#{litlp}_rschema;\
            \n  test-34: \"quoted\" +#{litlp}_rschema;\
            \n  test-35: \"quoted\"+ #{litlp}_rschema;\
            \n  test-36: \"quoted\" + #{litlp}_rschema;\
            \n  test-37: #{interpolant}+#{interpolant};\
            \n  test-38: #{interpolant} +#{interpolant};\
            \n  test-39: #{interpolant}+ #{interpolant};\
            \n  test-40: #{interpolant} + #{interpolant};\
            \n  test-41: #{interpolant}+lschema_#{ritlp};\
            \n  test-42: #{interpolant} +lschema_#{ritlp};\
            \n  test-43: #{interpolant}+ lschema_#{ritlp};\
            \n  test-44: #{interpolant} + lschema_#{ritlp};\
            \n  test-45: #{interpolant}+#{litlp}_rschema;\
            \n  test-46: #{interpolant} +#{litlp}_rschema;\
            \n  test-47: #{interpolant}+ #{litlp}_rschema;\
            \n  test-48: #{interpolant} + #{litlp}_rschema;\
            \n  test-49: lschema_#{ritlp}+lschema_#{ritlp};\
            \n  test-50: lschema_#{ritlp} +lschema_#{ritlp};\
            \n  test-51: lschema_#{ritlp}+ lschema_#{ritlp};\
            \n  test-52: lschema_#{ritlp} + lschema_#{ritlp};\
            \n  test-53: lschema_#{ritlp}+#{litlp}_rschema;\
            \n  test-54: lschema_#{ritlp} +#{litlp}_rschema;\
            \n  test-55: lschema_#{ritlp}+ #{litlp}_rschema;\
            \n  test-56: lschema_#{ritlp} + #{litlp}_rschema;\
            \n  test-57: #{litlp}_rschema+#{litlp}_rschema;\
            \n  test-58: #{litlp}_rschema +#{litlp}_rschema;\
            \n  test-59: #{litlp}_rschema+ #{litlp}_rschema;\
            \n  test-60: #{litlp}_rschema + #{litlp}_rschema;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test-1: literalliteral;\
        \n  test-2: literalliteral;\
        \n  test-3: literalliteral;\
        \n  test-4: literalliteral;\
        \n  test-5: literalquoted;\
        \n  test-6: literalquoted;\
        \n  test-7: literalquoted;\
        \n  test-8: literalquoted;\
        \n  test-9: literalinterpolant;\
        \n  test-10: literalinterpolant;\
        \n  test-11: literalinterpolant;\
        \n  test-12: literalinterpolant;\
        \n  test-13: literallschema_ritlp;\
        \n  test-14: literallschema_ritlp;\
        \n  test-15: literallschema_ritlp;\
        \n  test-16: literallschema_ritlp;\
        \n  test-17: literallitlp_rschema;\
        \n  test-18: literallitlp_rschema;\
        \n  test-19: literallitlp_rschema;\
        \n  test-20: literallitlp_rschema;\
        \n  test-21: \"quotedquoted\";\
        \n  test-22: \"quotedquoted\";\
        \n  test-23: \"quotedquoted\";\
        \n  test-24: \"quotedquoted\";\
        \n  test-25: \"quotedinterpolant\";\
        \n  test-26: \"quotedinterpolant\";\
        \n  test-27: \"quotedinterpolant\";\
        \n  test-28: \"quotedinterpolant\";\
        \n  test-29: \"quotedlschema_ritlp\";\
        \n  test-30: \"quotedlschema_ritlp\";\
        \n  test-31: \"quotedlschema_ritlp\";\
        \n  test-32: \"quotedlschema_ritlp\";\
        \n  test-33: \"quotedlitlp_rschema\";\
        \n  test-34: \"quotedlitlp_rschema\";\
        \n  test-35: \"quotedlitlp_rschema\";\
        \n  test-36: \"quotedlitlp_rschema\";\
        \n  test-37: interpolantinterpolant;\
        \n  test-38: interpolantinterpolant;\
        \n  test-39: interpolantinterpolant;\
        \n  test-40: interpolantinterpolant;\
        \n  test-41: interpolantlschema_ritlp;\
        \n  test-42: interpolantlschema_ritlp;\
        \n  test-43: interpolantlschema_ritlp;\
        \n  test-44: interpolantlschema_ritlp;\
        \n  test-45: interpolantlitlp_rschema;\
        \n  test-46: interpolantlitlp_rschema;\
        \n  test-47: interpolantlitlp_rschema;\
        \n  test-48: interpolantlitlp_rschema;\
        \n  test-49: lschema_ritlplschema_ritlp;\
        \n  test-50: lschema_ritlplschema_ritlp;\
        \n  test-51: lschema_ritlplschema_ritlp;\
        \n  test-52: lschema_ritlplschema_ritlp;\
        \n  test-53: lschema_ritlplitlp_rschema;\
        \n  test-54: lschema_ritlplitlp_rschema;\
        \n  test-55: lschema_ritlplitlp_rschema;\
        \n  test-56: lschema_ritlplitlp_rschema;\
        \n  test-57: litlp_rschemalitlp_rschema;\
        \n  test-58: litlp_rschemalitlp_rschema;\
        \n  test-59: litlp_rschemalitlp_rschema;\
        \n  test-60: litlp_rschemalitlp_rschema;\
        \n}\
        \n"
    );
}
