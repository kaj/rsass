//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/functions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($x, $y, $z) {\
            \n  @while $x < $y {\
            \n    $z: transform($z);\
            \n    @return $z;\
            \n  }\
            \n}\
            \n\
            \n@function bar($x) {\
            \n  @if $x {\
            \n    @return YES;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  answer: bar(true);\
            \n  flanswer: fudge(mux+flux) + mudge(a/b);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  answer: YES;\
        \n  flanswer: fudge(muxflux)mudge(a/b);\
        \n}\
        \n"
    );
}
