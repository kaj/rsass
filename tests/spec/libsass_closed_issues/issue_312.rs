//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_312.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@for $i from 0 through 10 {\r\
            \n    .foo [index = \"#{$i}\"] {\r\
            \n        transform: translateY($i * 100%);\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        ".foo [index=\"0\"] {\
        \n  transform: translateY(0%);\
        \n}\
        \n.foo [index=\"1\"] {\
        \n  transform: translateY(100%);\
        \n}\
        \n.foo [index=\"2\"] {\
        \n  transform: translateY(200%);\
        \n}\
        \n.foo [index=\"3\"] {\
        \n  transform: translateY(300%);\
        \n}\
        \n.foo [index=\"4\"] {\
        \n  transform: translateY(400%);\
        \n}\
        \n.foo [index=\"5\"] {\
        \n  transform: translateY(500%);\
        \n}\
        \n.foo [index=\"6\"] {\
        \n  transform: translateY(600%);\
        \n}\
        \n.foo [index=\"7\"] {\
        \n  transform: translateY(700%);\
        \n}\
        \n.foo [index=\"8\"] {\
        \n  transform: translateY(800%);\
        \n}\
        \n.foo [index=\"9\"] {\
        \n  transform: translateY(900%);\
        \n}\
        \n.foo [index=\"10\"] {\
        \n  transform: translateY(1000%);\
        \n}\
        \n"
    );
}
