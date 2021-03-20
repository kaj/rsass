//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/color_output.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$green: green;\
            \n$green-hex: #00FF00;\
            \n$green-hex-min: #0f0;\
            \n$green-rgb: rgb(0, 255, 0);\
            \n$green-rgba-t: rgba(0, 255, 0, 0.5);\
            \n$green-rgba-s: rgba(0, 255, 0, 1);\
            \n$offgreen: #00ff01;\
            \n$silver: silver;\
            \n$silver-hex: #ddd;\
            \n\
            \na {\
            \n\tq: silver;\
            \n\tr: #ddd;\
            \n\ts: green;\
            \n\tt: #00FF00;\
            \n\tv: #0f0;\
            \n\tw: rgb(0, 255, 0);\
            \n\tx: rgba(0, 255, 0, 0.5);\
            \n\ty: rgba(0, 255, 0, 1);\
            \n\tz: #00ff01; }\
            \n\
            \nb {\
            \n\tq: 1px solid silver;\
            \n\tr: 1px solid #ddd;\
            \n\ts: 1px solid green;\
            \n\tt: 1px solid #00FF00;\
            \n\tv: 1px solid #0f0;\
            \n\tw: 1px solid rgb(0, 255, 0);\
            \n\tx: 1px solid rgba(0, 255, 0, 0.5);\
            \n\ty: 1px solid rgba(0, 255, 0, 1);\
            \n\tz: 1px solid #00ff01; }\
            \n\
            \nc {\
            \n\tq: $silver;\
            \n\tr: $silver-hex;\
            \n\ts: $green;\
            \n\tt: $green-hex;\
            \n\tv: $green-hex-min;\
            \n\tw: $green-rgb;\
            \n\tx: $green-rgba-t;\
            \n\ty: $green-rgba-s;\
            \n\tz: $offgreen; }\
            \n\
            \nd {\
            \n\tq: 1px solid $silver;\
            \n\tr: 1px solid $silver-hex;\
            \n\ts: 1px solid $green;\
            \n\tt: 1px solid $green-hex;\
            \n\tv: 1px solid $green-hex-min;\
            \n\tw: 1px solid $green-rgb;\
            \n\tx: 1px solid $green-rgba-t;\
            \n\ty: 1px solid $green-rgba-s;\
            \n\tz: 1px solid $offgreen; }\
            \n\
            \n"
        )
        .unwrap(),
        "a {\
        \n  q: silver;\
        \n  r: #ddd;\
        \n  s: green;\
        \n  t: #00FF00;\
        \n  v: #0f0;\
        \n  w: lime;\
        \n  x: rgba(0, 255, 0, 0.5);\
        \n  y: lime;\
        \n  z: #00ff01;\
        \n}\
        \nb {\
        \n  q: 1px solid silver;\
        \n  r: 1px solid #ddd;\
        \n  s: 1px solid green;\
        \n  t: 1px solid #00FF00;\
        \n  v: 1px solid #0f0;\
        \n  w: 1px solid lime;\
        \n  x: 1px solid rgba(0, 255, 0, 0.5);\
        \n  y: 1px solid lime;\
        \n  z: 1px solid #00ff01;\
        \n}\
        \nc {\
        \n  q: silver;\
        \n  r: #ddd;\
        \n  s: green;\
        \n  t: #00FF00;\
        \n  v: #0f0;\
        \n  w: lime;\
        \n  x: rgba(0, 255, 0, 0.5);\
        \n  y: lime;\
        \n  z: #00ff01;\
        \n}\
        \nd {\
        \n  q: 1px solid silver;\
        \n  r: 1px solid #ddd;\
        \n  s: 1px solid green;\
        \n  t: 1px solid #00FF00;\
        \n  v: 1px solid #0f0;\
        \n  w: 1px solid lime;\
        \n  x: 1px solid rgba(0, 255, 0, 0.5);\
        \n  y: 1px solid lime;\
        \n  z: 1px solid #00ff01;\
        \n}\
        \n"
    );
}
