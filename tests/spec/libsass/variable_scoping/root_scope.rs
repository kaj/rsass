//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/root-scope.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: -42;\
            \n$y: -84;\
            \ndiv {\
            \n  x: $x;\
            \n  y: $y;\
            \n  for {\
            \n    x: $x;\
            \n    y: $y;\
            \n    @for $x from 1 through 5 {\
            \n      $y: $y + 5;\
            \n      x: $x;\
            \n      y: $y;\
            \n      $x: 999;\
            \n      $y: -9 !global;\
            \n      $x: -9 !global;\
            \n    }\
            \n    x: $x;\
            \n    y: $y;\
            \n  }\
            \n  x: $x;\
            \n  y: $y;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  x: -42;\
        \n  y: -84;\
        \n  x: -9;\
        \n  y: -9;\
        \n}\
        \ndiv for {\
        \n  x: -42;\
        \n  y: -84;\
        \n  x: 1;\
        \n  y: -79;\
        \n  x: 2;\
        \n  y: -74;\
        \n  x: 3;\
        \n  y: -69;\
        \n  x: 4;\
        \n  y: -64;\
        \n  x: 5;\
        \n  y: -59;\
        \n  x: -9;\
        \n  y: -9;\
        \n}\
        \n"
    );
}
