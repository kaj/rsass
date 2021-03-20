//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/variable/number.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a1: 10.0001;\
            \n$b1: 10.01;\
            \n$c1: -10.0001;\
            \n$d1: -10.01;\
            \n\
            \n#foo {\
            \n  a: $a1;\
            \n  b: $b1;\
            \n  c: $c1;\
            \n  d: $d1; }\
            \n\
            \n$a2: 0.0001;\
            \n$b2: 0.01;\
            \n$c2: -0.0001;\
            \n$d2: -0.01;\
            \n\
            \n#foo {\
            \n  a: $a2;\
            \n  b: $b2;\
            \n  c: $c2;\
            \n  d: $d2; }\
            \n\
            \n$a3: .0001;\
            \n$b3: .01;\
            \n$c3: -.0001;\
            \n$d3: -.01;\
            \n\
            \n#foo {\
            \n  a: $a3;\
            \n  b: $b3;\
            \n  c: $c3;\
            \n  d: $d3; }\
            \n\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  a: 10.0001;\
        \n  b: 10.01;\
        \n  c: -10.0001;\
        \n  d: -10.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n"
    );
}
