//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/four_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "alpha_percent", start_version is 3.7.

/// From "sass-spec/spec/core_functions/color/rgba/four_args/alpha_unitless"
#[test]
#[ignore] // failing
fn alpha_unitless() {
    assert_eq!(
        rsass(
            ".alpha-unitless {\n  negative: rgba(0, 0, 0, -10);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: rgba(0, 0, 0, 1);\n  above-max: rgba(0, 0, 0, 250);\n}\n"
        )
        .unwrap(),
        ".alpha-unitless {\n  negative: rgba(0, 0, 0, 0);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: black;\n  above-max: black;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgba/four_args/basic"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "basic {\n  transparent: rgba(0, 255, 127, 0);\n  opaque: rgba(190, 173, 237, 1);\n  partial: rgba(18, 52, 86, 0.5);\n  named: rgba($red: 0, $green: 255, $blue: 127, $alpha: 0.3);\n}\n\n// Channels that are out of bounds are clamped within bounds.\nclamped {\n  red: rgba(256, 0, 0, 0.5);\n  green: rgba(0, -1, 0, 0.5);\n  blue: rgba(0, 0, 9999, 0.5);\n  alpha-above: rgba(0, 0, 0, 1.1);\n  alpha-below: rgba(0, 0, 0, -0.1);\n}\n"
        )
        .unwrap(),
        "basic {\n  transparent: rgba(0, 255, 127, 0);\n  opaque: #beaded;\n  partial: rgba(18, 52, 86, 0.5);\n  named: rgba(0, 255, 127, 0.3);\n}\nclamped {\n  red: rgba(255, 0, 0, 0.5);\n  green: rgba(0, 0, 0, 0.5);\n  blue: rgba(0, 0, 255, 0.5);\n  alpha-above: black;\n  alpha-below: rgba(0, 0, 0, 0);\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgba/four_args/special_functions"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "a {\n  calc-1: rgba(calc(1), 2, 3, 0.4);\n  calc-2: rgba(1, calc(2), 3, 0.4);\n  calc-3: rgba(1, 2, calc(3), 0.4);\n  calc-4: rgba(1, 2, 3, calc(0.4));\n\n  var-1: rgba(var(--foo), 2, 3, 0.4);\n  var-2: rgba(1, var(--foo), 3, 0.4);\n  var-3: rgba(1, 2, var(--foo), 0.4);\n  var-4: rgba(1, 2, 3, var(--foo));\n\n  env-1: rgba(env(--foo), 2, 3, 0.4);\n  env-2: rgba(1, env(--foo), 3, 0.4);\n  env-3: rgba(1, 2, env(--foo), 0.4);\n  env-4: rgba(1, 2, 3, env(--foo));\n\n  min-1: rgba(min(1), 2, 3, 0.4);\n  min-2: rgba(1, min(2), 3, 0.4);\n  min-3: rgba(1, 2, min(3), 0.4);\n  min-4: rgba(1, 2, 3, min(0.4));\n\n  max-1: rgba(max(1), 2, 3, 0.4);\n  max-2: rgba(1, max(2), 3, 0.4);\n  max-3: rgba(1, 2, max(3), 0.4);\n  max-4: rgba(1, 2, 3, max(0.4));\n\n  calc-2-args: rgba(blue, calc(0.4));\n  var-2-args-alpha: rgba(blue, var(--foo));\n  var-2-args-color: rgba(var(--foo), 0.4);\n  var-2-args-both: rgba(var(--foo), var(--foo));\n}\n"
        )
        .unwrap(),
        "a {\n  calc-1: rgba(calc(1), 2, 3, 0.4);\n  calc-2: rgba(1, calc(2), 3, 0.4);\n  calc-3: rgba(1, 2, calc(3), 0.4);\n  calc-4: rgba(1, 2, 3, calc(0.4));\n  var-1: rgba(var(--foo), 2, 3, 0.4);\n  var-2: rgba(1, var(--foo), 3, 0.4);\n  var-3: rgba(1, 2, var(--foo), 0.4);\n  var-4: rgba(1, 2, 3, var(--foo));\n  env-1: rgba(env(--foo), 2, 3, 0.4);\n  env-2: rgba(1, env(--foo), 3, 0.4);\n  env-3: rgba(1, 2, env(--foo), 0.4);\n  env-4: rgba(1, 2, 3, env(--foo));\n  min-1: rgba(min(1), 2, 3, 0.4);\n  min-2: rgba(1, min(2), 3, 0.4);\n  min-3: rgba(1, 2, min(3), 0.4);\n  min-4: rgba(1, 2, 3, min(0.4));\n  max-1: rgba(max(1), 2, 3, 0.4);\n  max-2: rgba(1, max(2), 3, 0.4);\n  max-3: rgba(1, 2, max(3), 0.4);\n  max-4: rgba(1, 2, 3, max(0.4));\n  calc-2-args: rgba(0, 0, 255, calc(0.4));\n  var-2-args-alpha: rgba(0, 0, 255, var(--foo));\n  var-2-args-color: rgba(var(--foo), 0.4);\n  var-2-args-both: rgba(var(--foo), var(--foo));\n}\n"
    );
}
