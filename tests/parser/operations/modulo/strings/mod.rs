//! Tests auto-converted from "sass-spec/spec/parser/operations/modulo/strings"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/operations/modulo/strings/pairs"
#[test]
fn pairs() {
    assert_eq!(
        rsass(
            "foo {\n  test-37: #{interpolant}%#{interpolant};\n  test-38: #{interpolant} %#{interpolant};\n  test-39: #{interpolant}% #{interpolant};\n  test-40: #{interpolant} % #{interpolant};\n  test-41: #{interpolant}%lschema_#{ritlp};\n  test-42: #{interpolant} %lschema_#{ritlp};\n  test-43: #{interpolant}% lschema_#{ritlp};\n  test-44: #{interpolant} % lschema_#{ritlp};\n  test-45: #{interpolant}%#{litlp}_rschema;\n  test-46: #{interpolant} %#{litlp}_rschema;\n  test-47: #{interpolant}% #{litlp}_rschema;\n  test-48: #{interpolant} % #{litlp}_rschema;\n  test-49: lschema_#{ritlp}%lschema_#{ritlp};\n  test-50: lschema_#{ritlp} %lschema_#{ritlp};\n  test-51: lschema_#{ritlp}% lschema_#{ritlp};\n  test-52: lschema_#{ritlp} % lschema_#{ritlp};\n  test-53: lschema_#{ritlp}%#{litlp}_rschema;\n  test-54: lschema_#{ritlp} %#{litlp}_rschema;\n  test-55: lschema_#{ritlp}% #{litlp}_rschema;\n  test-56: lschema_#{ritlp} % #{litlp}_rschema;\n  test-57: #{litlp}_rschema%#{litlp}_rschema;\n  test-58: #{litlp}_rschema %#{litlp}_rschema;\n  test-59: #{litlp}_rschema% #{litlp}_rschema;\n  test-60: #{litlp}_rschema % #{litlp}_rschema;\n}\n"
        )
        .unwrap(),
        "foo {\n  test-37: interpolant%interpolant;\n  test-38: interpolant %interpolant;\n  test-39: interpolant% interpolant;\n  test-40: interpolant % interpolant;\n  test-41: interpolant%lschema_ritlp;\n  test-42: interpolant %lschema_ritlp;\n  test-43: interpolant% lschema_ritlp;\n  test-44: interpolant % lschema_ritlp;\n  test-45: interpolant%litlp_rschema;\n  test-46: interpolant %litlp_rschema;\n  test-47: interpolant% litlp_rschema;\n  test-48: interpolant % litlp_rschema;\n  test-49: lschema_ritlp%lschema_ritlp;\n  test-50: lschema_ritlp %lschema_ritlp;\n  test-51: lschema_ritlp% lschema_ritlp;\n  test-52: lschema_ritlp % lschema_ritlp;\n  test-53: lschema_ritlp%litlp_rschema;\n  test-54: lschema_ritlp %litlp_rschema;\n  test-55: lschema_ritlp% litlp_rschema;\n  test-56: lschema_ritlp % litlp_rschema;\n  test-57: litlp_rschema%litlp_rschema;\n  test-58: litlp_rschema %litlp_rschema;\n  test-59: litlp_rschema% litlp_rschema;\n  test-60: litlp_rschema % litlp_rschema;\n}\n"
    );
}
