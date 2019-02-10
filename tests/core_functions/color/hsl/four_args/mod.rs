//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/color/hsl/four_args/alpha_percent"
#[test]
#[ignore] // failing
fn alpha_percent() {
    assert_eq!(
        rsass(
            ".alpha-percent {\n  negative: hsl(0, 0, 0, -10%);\n  min: hsl(0, 0, 0, 0%);\n  positive: hsl(0, 0, 0, 45.6%);\n  max: hsl(0, 0, 0, 100%);\n  above-max: hsl(0, 0, 0, 250%);\n}\n"
        )
        .unwrap(),
        ".alpha-percent {\n  negative: rgba(0, 0, 0, 0);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: black;\n  above-max: black;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/hsl/four_args/alpha_unitless"
#[test]
#[ignore] // failing
fn alpha_unitless() {
    assert_eq!(
        rsass(
            ".alpha-unitless {\n  negative: hsl(0, 0, 0, -10);\n  min: hsl(0, 0, 0, 0);\n  positive: hsl(0, 0, 0, 0.456);\n  max: hsl(0, 0, 0, 1);\n  above-max: hsl(0, 0, 0, 250);\n}\n"
        )
        .unwrap(),
        ".alpha-unitless {\n  negative: rgba(0, 0, 0, 0);\n  min: rgba(0, 0, 0, 0);\n  positive: rgba(0, 0, 0, 0.456);\n  max: black;\n  above-max: black;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/hsl/four_args/basic"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "basic {\n  transparent: hsl(180, 60%, 50%, 0);\n  opaque: hsl(180, 60%, 50%, 1);\n  partial: hsl(180, 60%, 50%, 0.5);\n  named: hsl($hue: 180, $saturation: 60%, $lightness: 50%, $alpha: 0.4);\n}\n\n// Channels that are out of bounds are clamped within bounds.\nclamped {\n  saturation: hsl(0, -0.1%, 50%, 0.5);\n  blue: hsl(0, 100%, 9999%, 0.5);\n  alpha-above: hsl(0, 100%, 50%, 1.1);\n  alpha-below: rgba(0, 100%, 50%, -0.1);\n}\n"
        )
        .unwrap(),
        "basic {\n  transparent: rgba(51, 204, 204, 0);\n  opaque: #33cccc;\n  partial: rgba(51, 204, 204, 0.5);\n  named: rgba(51, 204, 204, 0.4);\n}\nclamped {\n  saturation: rgba(128, 128, 128, 0.5);\n  blue: rgba(255, 255, 255, 0.5);\n  alpha-above: red;\n  alpha-below: rgba(0, 255, 128, 0);\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/hsl/four_args/special_functions"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "a {\n  calc-1: hsl(calc(1), 2%, 3%, 0.4);\n  calc-2: hsl(1, calc(2%), 3%, 0.4);\n  calc-3: hsl(1, 2%, calc(3%), 0.4);\n  calc-4: hsl(1, 2%, 3%, calc(0.4));\n\n  var-1: hsl(var(--foo), 2%, 3%, 0.4);\n  var-2: hsl(1, var(--foo), 3%, 0.4);\n  var-3: hsl(1, 2%, var(--foo), 0.4);\n  var-4: hsl(1, 2%, 3%, var(--foo));\n\n  env-1: hsl(env(--foo), 2%, 3%, 0.4);\n  env-2: hsl(1, env(--foo), 3%, 0.4);\n  env-3: hsl(1, 2%, env(--foo), 0.4);\n  env-4: hsl(1, 2%, 3%, env(--foo));\n\n  min-1: hsl(min(1), 2%, 3%, 0.4);\n  min-2: hsl(1, min(2%), 3%, 0.4);\n  min-3: hsl(1, 2%, min(3%), 0.4);\n  min-4: hsl(1, 2%, 3%, min(0.4));\n\n  max-1: hsl(max(1), 2%, 3%, 0.4);\n  max-2: hsl(1, max(2%), 3%, 0.4);\n  max-3: hsl(1, 2%, max(3%), 0.4);\n  max-4: hsl(1, 2%, 3%, max(0.4));\n}\n"
        )
        .unwrap(),
        "a {\n  calc-1: hsl(calc(1), 2%, 3%, 0.4);\n  calc-2: hsl(1, calc(2%), 3%, 0.4);\n  calc-3: hsl(1, 2%, calc(3%), 0.4);\n  calc-4: hsl(1, 2%, 3%, calc(0.4));\n  var-1: hsl(var(--foo), 2%, 3%, 0.4);\n  var-2: hsl(1, var(--foo), 3%, 0.4);\n  var-3: hsl(1, 2%, var(--foo), 0.4);\n  var-4: hsl(1, 2%, 3%, var(--foo));\n  env-1: hsl(env(--foo), 2%, 3%, 0.4);\n  env-2: hsl(1, env(--foo), 3%, 0.4);\n  env-3: hsl(1, 2%, env(--foo), 0.4);\n  env-4: hsl(1, 2%, 3%, env(--foo));\n  min-1: hsl(min(1), 2%, 3%, 0.4);\n  min-2: hsl(1, min(2%), 3%, 0.4);\n  min-3: hsl(1, 2%, min(3%), 0.4);\n  min-4: hsl(1, 2%, 3%, min(0.4));\n  max-1: hsl(max(1), 2%, 3%, 0.4);\n  max-2: hsl(1, max(2%), 3%, 0.4);\n  max-3: hsl(1, 2%, max(3%), 0.4);\n  max-4: hsl(1, 2%, 3%, max(0.4));\n}\n"
    );
}
