//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/three_args"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/color/rgba/three_args/basic"
#[test]
#[ignore] // failing
fn basic() {
    assert_eq!(
        rsass(
            "// Channels may be specified as unitless numbers between 0 and 255.\nunitless {\n  numbers: rgba(18, 52, 86);\n  beaded: rgba(190, 173, 237);\n  springgreen: rgba(0, 255, 127);\n  named: rgba($red: 0, $green: 255, $blue: 127);\n\n  // Channels that are out of bounds are clamped within bounds.\n  clamped {\n    red: rgba(256, 0, 0);\n    green: rgba(0, -1, 0);\n    blue: rgba(0, 0, 9999);\n  }\n}\n\n// Channels may also be specified as percents between 0% and 100%, which may be\n// mixed with unitless numbers.\npercents {\n  all-percent: rgba(7.1%, 20.4%, 33.9%);\n  unitless-green: rgba(74.7%, 173, 93%);\n  percent-green: rgba(190, 68%, 237);\n  boundaries: rgba(0%, 100%, 50%);\n\n  // Channels that are out of bounds are clamped within bounds.\n  clamped {\n    red: rgba(100.1%, 0, 0);\n    green: rgba(0, -0.1%, 0);\n    blue: rgba(0, 0, 200%);\n  }\n}\n"
        )
        .unwrap(),
        "unitless {\n  numbers: #123456;\n  beaded: #beaded;\n  springgreen: springgreen;\n  named: springgreen;\n}\nunitless clamped {\n  red: red;\n  green: black;\n  blue: blue;\n}\npercents {\n  all-percent: #123456;\n  unitless-green: #beaded;\n  percent-green: #beaded;\n  boundaries: #00ff80;\n}\npercents clamped {\n  red: red;\n  green: black;\n  blue: blue;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/color/rgba/three_args/special_functions"
#[test]
#[ignore] // failing
fn special_functions() {
    assert_eq!(
        rsass(
            "a {\n  calc-1: rgba(calc(1), 2, 3);\n  calc-2: rgba(1, calc(2), 3);\n  calc-3: rgba(1, 2, calc(3));\n\n  var-1: rgba(var(--foo), 2, 3);\n  var-2: rgba(1, var(--foo), 3);\n  var-3: rgba(1, 2, var(--foo));\n\n  env-1: rgba(env(--foo), 2, 3);\n  env-2: rgba(1, env(--foo), 3);\n  env-3: rgba(1, 2, env(--foo));\n\n  min-1: rgba(min(1), 2, 3);\n  min-2: rgba(1, min(2), 3);\n  min-3: rgba(1, 2, min(3));\n\n  max-1: rgba(max(1), 2, 3);\n  max-2: rgba(1, max(2), 3);\n  max-3: rgba(1, 2, max(3));\n}\n"
        )
        .unwrap(),
        "a {\n  calc-1: rgba(calc(1), 2, 3);\n  calc-2: rgba(1, calc(2), 3);\n  calc-3: rgba(1, 2, calc(3));\n  var-1: rgba(var(--foo), 2, 3);\n  var-2: rgba(1, var(--foo), 3);\n  var-3: rgba(1, 2, var(--foo));\n  env-1: rgba(env(--foo), 2, 3);\n  env-2: rgba(1, env(--foo), 3);\n  env-3: rgba(1, 2, env(--foo));\n  min-1: rgba(min(1), 2, 3);\n  min-2: rgba(1, min(2), 3);\n  min-3: rgba(1, 2, min(3));\n  max-1: rgba(max(1), 2, 3);\n  max-2: rgba(1, max(2), 3);\n  max-3: rgba(1, 2, max(3));\n}\n"
    );
}
