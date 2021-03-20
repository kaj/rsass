//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/interpolated.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// You can interpolate into a media type.\
            \n@media bar#{12} {x {y: z}}\
            \n\
            \n// Media queries should be reparsed after interpolation is resolved.\
            \n@media #{\"only screen\"} and\
            \n       #{\"(min-width: 700px)\"} and\
            \n       #{\"(max-width: \"+\"1920px)\"} {\
            \n  x {y: z}\
            \n}\
            \n\
            \n// Queries don\'t have to fully parse before interpolation is resolved.\
            \n@media scr#{\"een, pri\"}nt a#{\"nd (max-width: 300px)\"} {x {y: z}}\
            \n\
            \n\
            \n\
            \n"
        )
        .unwrap(),
        "@media bar12 {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 700px) and (max-width: 1920px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media screen, print and (max-width: 300px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n"
    );
}
