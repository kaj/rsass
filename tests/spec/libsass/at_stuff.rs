//! Tests auto-converted from "sass-spec/spec/libsass/at-stuff.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@fudge hux bloo;\
            \n\
            \ndiv {\
            \n\tcolor: red;\
            \n\t@fudge {\
            \n\t\tspan {\
            \n\t\t\twidth: 10px;\
            \n\t\t\ta {\
            \n\t\t\t\tfont: whatever;\
            \n\t\t\t}\
            \n\t\t}\
            \n\t}\
            \n\theight: 20px;\
            \n\t@-webkit-keyframes SOMETHING {\
            \n\t\t0%   { opacity: 0; }\
            \n\t\t50%  { opacity: 0.5; }\
            \n\t\t100% { opacity: 1.0; }\
            \n\t}\
            \n\t@-webkit-keyframes BOUNCE {\
            \n\t\tfrom {\
            \n\t\t\tleft: 0px;\
            \n\t\t}\
            \n\t\tto {\
            \n\t\t\tleft: 200px;\
            \n\t\t}\
            \n\t}\
            \n}\
            \n\
            \ndiv {\
            \n\tspan {\
            \n\t\tfont: whatever;\
            \n\t}\
            \n\tborder: {\
            \n\t\tupper: {\
            \n\t\t\tleft: 10px;\
            \n\t\t\tright: 9px;\
            \n\t\t}\
            \n\t\tlower: {\
            \n\t\t\tleft: 8px;\
            \n\t\t\tright: 7px;\
            \n\t\t}\
            \n\t}\
            \n\tbackground: gray;\
            \n}\
            \n\
            \n@fudge HEY, HOO, HA:first-child {\
            \n\tcolor: blue;\
            \n}\
            \n\
            \n@mudge div span, a:visited;\
            \n\
            \n@fu#{dge} foo {\
            \n\tcolor: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "@fudge hux bloo;\
        \ndiv {\
        \n  color: red;\
        \n  height: 20px;\
        \n}\
        \n@fudge {\
        \n  div span {\
        \n    width: 10px;\
        \n  }\
        \n  div span a {\
        \n    font: whatever;\
        \n  }\
        \n}\
        \n@-webkit-keyframes SOMETHING {\
        \n  0% {\
        \n    opacity: 0;\
        \n  }\
        \n  50% {\
        \n    opacity: 0.5;\
        \n  }\
        \n  100% {\
        \n    opacity: 1;\
        \n  }\
        \n}\
        \n@-webkit-keyframes BOUNCE {\
        \n  from {\
        \n    left: 0px;\
        \n  }\
        \n  to {\
        \n    left: 200px;\
        \n  }\
        \n}\
        \ndiv {\
        \n  border-upper-left: 10px;\
        \n  border-upper-right: 9px;\
        \n  border-lower-left: 8px;\
        \n  border-lower-right: 7px;\
        \n  background: gray;\
        \n}\
        \ndiv span {\
        \n  font: whatever;\
        \n}\
        \n@fudge HEY, HOO, HA:first-child {\
        \n  color: blue;\
        \n}\
        \n@mudge div span, a:visited;\
        \n@fudge foo {\
        \n  color: red;\
        \n}\
        \n"
    );
}
