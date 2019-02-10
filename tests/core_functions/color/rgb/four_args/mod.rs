//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/color/rgb/four_args/alpha_percent"
#[test]
#[ignore] // failing
fn alpha_percent() {
    assert_eq!(
        rsass(
            ".alpha-percent {\n  negative: rgb(0, 0, 0, -10%);\n  min: rgb(0, 0, 0, 0%);\n  positive: rgb(0, 0, 0, 45.6%);\n  max: rgb(0, 0, 0, 100%);\n  above-max: rgb(0, 0, 0, 250%);\n}\n"
        )
        .unwrap(),
        ".alpha-percent {\n  negative: rgba(0, 0, 0, 0);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: black;\n  above-max: black;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgb/four_args/alpha_unitless"
#[test]
#[ignore] // failing
fn alpha_unitless() {
    assert_eq!(
        rsass(
            ".alpha-unitless {\n  negative: rgb(0, 0, 0, -10);\n  min: rgb(0, 0, 0, 0);\n  positive: rgb(0, 0, 0, 0.456);\n  max: rgb(0, 0, 0, 1);\n  above-max: rgb(0, 0, 0, 250);\n}\n"
        )
        .unwrap(),
        ".alpha-unitless {\n  negative: rgba(0, 0, 0, 0);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: black;\n  above-max: black;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgb/four_args/basic"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "basic {\n  transparent: rgb(0, 255, 127, 0);\n  opaque: rgb(190, 173, 237, 1);\n  partial: rgb(18, 52, 86, 0.5);\n  named: rgb($red: 0, $green: 255, $blue: 127, $alpha: 0.3);\n}\n\n// Channels that are out of bounds are clamped within bounds.\nclamped {\n  red: rgb(256, 0, 0, 0.5);\n  green: rgb(0, -1, 0, 0.5);\n  blue: rgb(0, 0, 9999, 0.5);\n  alpha-above: rgb(0, 0, 0, 1.1);\n  alpha-below: rgb(0, 0, 0, -0.1);\n}\n"
        )
        .unwrap(),
        "basic {\n  transparent: rgba(0, 255, 127, 0);\n  opaque: #beaded;\n  partial: rgba(18, 52, 86, 0.5);\n  named: rgba(0, 255, 127, 0.3);\n}\nclamped {\n  red: rgba(255, 0, 0, 0.5);\n  green: rgba(0, 0, 0, 0.5);\n  blue: rgba(0, 0, 255, 0.5);\n  alpha-above: black;\n  alpha-below: rgba(0, 0, 0, 0);\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgb/four_args/special_functions"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "a {\n  calc-1: rgb(calc(1), 2, 3, 0.4);\n  calc-2: rgb(1, calc(2), 3, 0.4);\n  calc-3: rgb(1, 2, calc(3), 0.4);\n  calc-4: rgb(1, 2, 3, calc(0.4));\n\n  var-1: rgb(var(--foo), 2, 3, 0.4);\n  var-2: rgb(1, var(--foo), 3, 0.4);\n  var-3: rgb(1, 2, var(--foo), 0.4);\n  var-4: rgb(1, 2, 3, var(--foo));\n\n  env-1: rgb(env(--foo), 2, 3, 0.4);\n  env-2: rgb(1, env(--foo), 3, 0.4);\n  env-3: rgb(1, 2, env(--foo), 0.4);\n  env-4: rgb(1, 2, 3, env(--foo));\n\n  min-1: rgb(min(1), 2, 3, 0.4);\n  min-2: rgb(1, min(2), 3, 0.4);\n  min-3: rgb(1, 2, min(3), 0.4);\n  min-4: rgb(1, 2, 3, min(0.4));\n\n  max-1: rgb(max(1), 2, 3, 0.4);\n  max-2: rgb(1, max(2), 3, 0.4);\n  max-3: rgb(1, 2, max(3), 0.4);\n  max-4: rgb(1, 2, 3, max(0.4));\n\n  calc-2-args: rgb(blue, calc(0.4));\n  var-2-args-alpha: rgb(blue, var(--foo));\n  var-2-args-color: rgb(var(--foo), 0.4);\n  var-2-args-both: rgb(var(--foo), var(--foo));\n}\n"
        )
        .unwrap(),
        "a {\n  calc-1: rgb(calc(1), 2, 3, 0.4);\n  calc-2: rgb(1, calc(2), 3, 0.4);\n  calc-3: rgb(1, 2, calc(3), 0.4);\n  calc-4: rgb(1, 2, 3, calc(0.4));\n  var-1: rgb(var(--foo), 2, 3, 0.4);\n  var-2: rgb(1, var(--foo), 3, 0.4);\n  var-3: rgb(1, 2, var(--foo), 0.4);\n  var-4: rgb(1, 2, 3, var(--foo));\n  env-1: rgb(env(--foo), 2, 3, 0.4);\n  env-2: rgb(1, env(--foo), 3, 0.4);\n  env-3: rgb(1, 2, env(--foo), 0.4);\n  env-4: rgb(1, 2, 3, env(--foo));\n  min-1: rgb(min(1), 2, 3, 0.4);\n  min-2: rgb(1, min(2), 3, 0.4);\n  min-3: rgb(1, 2, min(3), 0.4);\n  min-4: rgb(1, 2, 3, min(0.4));\n  max-1: rgb(max(1), 2, 3, 0.4);\n  max-2: rgb(1, max(2), 3, 0.4);\n  max-3: rgb(1, 2, max(3), 0.4);\n  max-4: rgb(1, 2, 3, max(0.4));\n  calc-2-args: rgb(0, 0, 255, calc(0.4));\n  var-2-args-alpha: rgb(0, 0, 255, var(--foo));\n  var-2-args-color: rgb(var(--foo), 0.4);\n  var-2-args-both: rgb(var(--foo), var(--foo));\n}\n"
    );
}
