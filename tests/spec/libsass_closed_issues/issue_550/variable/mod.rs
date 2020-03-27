//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/variable"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_550/variable/dimension.hrx"
#[test]
fn dimension() {
    assert_eq!(
        rsass(
            "$i1: 10.0001px;\
            \n$j1: 10.01px;\
            \n$k1: -10.0001px;\
            \n$l1: -10.01px;\
            \n\
            \n#foo {\
            \n  i: $i1;\
            \n  j: $j1;\
            \n  k: $k1;\
            \n  l: $l1; }\
            \n\
            \n$i2: 0.0001px;\
            \n$j2: 0.01px;\
            \n$k2: -0.0001px;\
            \n$l2: -0.01px;\
            \n\
            \n#foo {\
            \n  i: $i2;\
            \n  j: $j2;\
            \n  k: $k2;\
            \n  l: $l2; }\
            \n\
            \n$i3: .0001px;\
            \n$j3: .01px;\
            \n$k3: -.0001px;\
            \n$l3: -.01px;\
            \n\
            \n#foo {\
            \n  i: $i3;\
            \n  j: $j3;\
            \n  k: $k3;\
            \n  l: $l3; }\
            \n\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  i: 10.0001px;\
        \n  j: 10.01px;\
        \n  k: -10.0001px;\
        \n  l: -10.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_550/variable/number.hrx"
#[test]
fn number() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass-closed-issues/issue_550/variable/percent.hrx"
#[test]
fn percent() {
    assert_eq!(
        rsass(
            "$e1: 10.0001%;\
            \n$f1: 10.01%;\
            \n$g1: -10.0001%;\
            \n$h1: -10.01%;\
            \n\
            \n#foo {\
            \n  e: $e1;\
            \n  f: $f1;\
            \n  g: $g1;\
            \n  h: $h1; }\
            \n$e2: 0.0001%;\
            \n$f2: 0.01%;\
            \n$g2: -0.0001%;\
            \n$h2: -0.01%;\
            \n\
            \n#foo {\
            \n  e: $e2;\
            \n  f: $f2;\
            \n  g: $g2;\
            \n  h: $h2; }\
            \n\
            \n$e3: .0001%;\
            \n$f3: .01%;\
            \n$g3: -.0001%;\
            \n$h3: -.01%;\
            \n\
            \n#foo {\
            \n  e: $e3;\
            \n  f: $f3;\
            \n  g: $g3;\
            \n  h: $h3; }\
            \n\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  e: 10.0001%;\
        \n  f: 10.01%;\
        \n  g: -10.0001%;\
        \n  h: -10.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n"
    );
}
