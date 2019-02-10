//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/color/rgb/one_arg/alpha"
#[test]
#[ignore] // failing
fn alpha() {
    assert_eq!(
        rsass(
            "basic {\n  transparent: rgb(0 255 127 / 0);\n  opaque: rgb(190 173 237 / 1);\n  partial: rgb(18 52 86 / 0.5);\n  percent: rgb(18 52 86 / 50%);\n  named: rgb($channels: 0 255 127 / 0.3);\n\n  // Extra parens shouldn\'t cause the slash to be forced into division.\n  parenthesized: (rgb(0 255 127 / 0.3));\n}\n\n// Channels that are out of bounds are clamped within bounds.\nclamped {\n  red: rgb(256 0 0 / 0.5);\n  green: rgb(0 -1 0 / 0.5);\n  blue: rgb(0 0 9999 / 0.5);\n  alpha-above: rgb(0 0 0 / 1.1);\n  alpha-below: rgb(0 0 0 / -0.1);\n  alpha-above-percent: rgb(0 0 0 / 250%);\n  alpha-below-percent: rgb(0 0 0 / -10%);\n}\n"
        )
        .unwrap(),
        "basic {\n  transparent: rgba(0, 255, 127, 0);\n  opaque: #beaded;\n  partial: rgba(18, 52, 86, 0.5);\n  percent: rgba(18, 52, 86, 0.5);\n  named: rgba(0, 255, 127, 0.3);\n  parenthesized: rgba(0, 255, 127, 0.3);\n}\nclamped {\n  red: rgba(255, 0, 0, 0.5);\n  green: rgba(0, 0, 0, 0.5);\n  blue: rgba(0, 0, 255, 0.5);\n  alpha-above: black;\n  alpha-below: rgba(0, 0, 0, 0);\n  alpha-above-percent: black;\n  alpha-below-percent: rgba(0, 0, 0, 0);\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgb/one_arg/basic"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "// Channels may be specified as unitless numbers between 0 and 255.\nunitless {\n  numbers: rgb(18 52 86);\n  beaded: rgb(190 173 237);\n  springgreen: rgb(0 255 127);\n  named: rgb($channels: 0 255 127);\n\n  // Channels that are out of bounds are clamped within bounds.\n  clamped {\n    red: rgb(256 0 0);\n    green: rgb(0 -1 0);\n    blue: rgb(0 0 9999);\n  }\n}\n\n// Channels may also be specified as percents between 0% and 100%, which may be\n// mixed with unitless numbers.\npercents {\n  all-percent: rgb(7.1% 20.4% 33.9%);\n  unitless-green: rgb(74.7% 173 93%);\n  percent-green: rgb(190 68% 237);\n  boundaries: rgb(0% 100% 50%);\n\n  // Channels that are out of bounds are clamped within bounds.\n  clamped {\n    red: rgb(100.1% 0 0);\n    green: rgb(0 -0.1% 0);\n    blue: rgb(0 0 200%);\n  }\n}\n"
        )
        .unwrap(),
        "unitless {\n  numbers: #123456;\n  beaded: #beaded;\n  springgreen: springgreen;\n  named: springgreen;\n}\nunitless clamped {\n  red: red;\n  green: black;\n  blue: blue;\n}\npercents {\n  all-percent: #123456;\n  unitless-green: #beaded;\n  percent-green: #beaded;\n  boundaries: #00ff80;\n}\npercents clamped {\n  red: red;\n  green: black;\n  blue: blue;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgb/one_arg/special_functions"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "no-alpha {\n  calc-1: rgb(calc(1) 2 3);\n  calc-2: rgb(1 calc(2) 3);\n  calc-3: rgb(1 2 calc(3));\n\n  var-1: rgb(var(--foo) 2 3);\n  var-2: rgb(1 var(--foo) 3);\n  var-3: rgb(1 2 var(--foo));\n\n  env-1: rgb(env(--foo) 2 3);\n  env-2: rgb(1 env(--foo) 3);\n  env-3: rgb(1 2 env(--foo));\n\n  min-1: rgb(min(1) 2 3);\n  min-2: rgb(1 min(2) 3);\n  min-3: rgb(1 2 min(3));\n\n  max-1: rgb(max(1) 2 3);\n  max-2: rgb(1 max(2) 3);\n  max-3: rgb(1 2 max(3));\n\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  multi-argument-var-1-of-2: rgb(var(--foo) 2);\n  multi-argument-var-2-of-2: rgb(1 var(--foo));\n  multi-argument-var-1-of-1: rgb(var(--foo));\n}\n\nalpha {\n  calc-1: rgb(calc(1) 2 3 / 0.4);\n  calc-2: rgb(1 calc(2) 3 / 0.4);\n  calc-3: rgb(1 2 calc(3) / 0.4);\n  calc-4: rgb(1 2 3 / calc(0.4));\n\n  var-1: rgb(var(--foo) 2 3 / 0.4);\n  var-2: rgb(1 var(--foo) 3 / 0.4);\n  var-3: rgb(1 2 var(--foo) / 0.4);\n  var-4: rgb(1 2 3 / var(--foo));\n\n  env-1: rgb(env(--foo) 2 3 / 0.4);\n  env-2: rgb(1 env(--foo) 3 / 0.4);\n  env-3: rgb(1 2 env(--foo) / 0.4);\n  env-4: rgb(1 2 3 / env(--foo));\n\n  min-1: rgb(min(1) 2 3 / 0.4);\n  min-2: rgb(1 min(2) 3 / 0.4);\n  min-3: rgb(1 2 min(3) / 0.4);\n  min-4: rgb(1 2 3 / min(0.4));\n\n  max-1: rgb(max(1) 2 3 / 0.4);\n  max-2: rgb(1 max(2) 3 / 0.4);\n  max-3: rgb(1 2 max(3) / 0.4);\n  max-4: rgb(1 2 3 / max(0.4));\n\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  multi-argument-var-1-of-2: rgb(var(--foo) 2 / 0.4);\n  multi-argument-var-2-of-2: rgb(1 var(--foo) / 0.4);\n  multi-argument-var-1-of-1: rgb(var(--foo) / 0.4);\n}\n"
        )
        .unwrap(),
        "no-alpha {\n  calc-1: rgb(calc(1), 2, 3);\n  calc-2: rgb(1, calc(2), 3);\n  calc-3: rgb(1, 2, calc(3));\n  var-1: rgb(var(--foo), 2, 3);\n  var-2: rgb(1, var(--foo), 3);\n  var-3: rgb(1, 2, var(--foo));\n  env-1: rgb(env(--foo), 2, 3);\n  env-2: rgb(1, env(--foo), 3);\n  env-3: rgb(1, 2, env(--foo));\n  min-1: rgb(min(1), 2, 3);\n  min-2: rgb(1, min(2), 3);\n  min-3: rgb(1, 2, min(3));\n  max-1: rgb(max(1), 2, 3);\n  max-2: rgb(1, max(2), 3);\n  max-3: rgb(1, 2, max(3));\n  multi-argument-var-1-of-2: rgb(var(--foo) 2);\n  multi-argument-var-2-of-2: rgb(1 var(--foo));\n  multi-argument-var-1-of-1: rgb(var(--foo));\n}\nalpha {\n  calc-1: rgb(calc(1), 2, 3, 0.4);\n  calc-2: rgb(1, calc(2), 3, 0.4);\n  calc-3: rgb(1 2 calc(3)/0.4);\n  calc-4: rgb(1 2 3/calc(0.4));\n  var-1: rgb(var(--foo), 2, 3, 0.4);\n  var-2: rgb(1, var(--foo), 3, 0.4);\n  var-3: rgb(1 2 var(--foo)/0.4);\n  var-4: rgb(1 2 3/var(--foo));\n  env-1: rgb(env(--foo), 2, 3, 0.4);\n  env-2: rgb(1, env(--foo), 3, 0.4);\n  env-3: rgb(1 2 env(--foo)/0.4);\n  env-4: rgb(1 2 3/env(--foo));\n  min-1: rgb(min(1), 2, 3, 0.4);\n  min-2: rgb(1, min(2), 3, 0.4);\n  min-3: rgb(1 2 min(3)/0.4);\n  min-4: rgb(1 2 3/min(0.4));\n  max-1: rgb(max(1), 2, 3, 0.4);\n  max-2: rgb(1, max(2), 3, 0.4);\n  max-3: rgb(1 2 max(3)/0.4);\n  max-4: rgb(1 2 3/max(0.4));\n  multi-argument-var-1-of-2: rgb(var(--foo) 2/0.4);\n  multi-argument-var-2-of-2: rgb(1 var(--foo)/0.4);\n  multi-argument-var-1-of-1: rgb(var(--foo)/0.4);\n}\n"
    );
}
