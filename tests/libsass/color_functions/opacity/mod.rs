//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/opacity"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/color-functions/opacity/alpha.hrx"
#[test]
fn alpha() {
    assert_eq!(
        rsass(
            "foo {\n  c0: opacity(rgba(0,0,0,0.0));\n  c1: opacity(rgba(0,0,0,0.1));\n  c2: opacity(rgba(0,0,0,0.2));\n  c3: opacity(rgba(0,0,0,0.3));\n  c4: opacity(rgba(0,0,0,0.4));\n  c5: opacity(rgba(0,0,0,0.5));\n  c6: opacity(rgba(0,0,0,0.6));\n  c7: opacity(rgba(0,0,0,0.7));\n  c8: opacity(rgba(0,0,0,0.8));\n  c9: opacity(rgba(0,0,0,0.9));\n  c10: opacity(rgba(0,0,0,1));\n}\n"
        )
        .unwrap(),
        "foo {\n  c0: 0;\n  c1: 0.1;\n  c2: 0.2;\n  c3: 0.3;\n  c4: 0.4;\n  c5: 0.5;\n  c6: 0.6;\n  c7: 0.7;\n  c8: 0.8;\n  c9: 0.9;\n  c10: 1;\n}\n"
    );
}

// From "sass-spec/spec/libsass/color-functions/opacity/fade-in.hrx"
#[test]
fn fade_in() {
    assert_eq!(
        rsass(
            "foo {\n  c0: fade-in(rgba(0,0,0,0.0), 0.1);\n  c1: fade-in(rgba(0,0,0,0.1), 0.1);\n  c2: fade-in(rgba(0,0,0,0.2), 0.1);\n  c3: fade-in(rgba(0,0,0,0.3), 0.1);\n  c4: fade-in(rgba(0,0,0,0.4), 0.1);\n  c5: fade-in(rgba(0,0,0,0.5), 0.1);\n  c6: fade-in(rgba(0,0,0,0.6), 0.1);\n  c7: fade-in(rgba(0,0,0,0.7), 0.1);\n  c8: fade-in(rgba(0,0,0,0.8), 0.1);\n  c9: fade-in(rgba(0,0,0,0.9), 0.1);\n  c10: fade-in(rgba(0,0,0,1), 0.1);\n}\n"
        )
        .unwrap(),
        "foo {\n  c0: rgba(0, 0, 0, 0.1);\n  c1: rgba(0, 0, 0, 0.2);\n  c2: rgba(0, 0, 0, 0.3);\n  c3: rgba(0, 0, 0, 0.4);\n  c4: rgba(0, 0, 0, 0.5);\n  c5: rgba(0, 0, 0, 0.6);\n  c6: rgba(0, 0, 0, 0.7);\n  c7: rgba(0, 0, 0, 0.8);\n  c8: rgba(0, 0, 0, 0.9);\n  c9: black;\n  c10: black;\n}\n"
    );
}

// From "sass-spec/spec/libsass/color-functions/opacity/fade-out.hrx"
#[test]
fn fade_out() {
    assert_eq!(
        rsass(
            "foo {\n  c0: fade-out(rgba(0,0,0,0.0), 0.1);\n  c1: fade-out(rgba(0,0,0,0.1), 0.1);\n  c2: fade-out(rgba(0,0,0,0.2), 0.1);\n  c3: fade-out(rgba(0,0,0,0.3), 0.1);\n  c4: fade-out(rgba(0,0,0,0.4), 0.1);\n  c5: fade-out(rgba(0,0,0,0.5), 0.1);\n  c6: fade-out(rgba(0,0,0,0.6), 0.1);\n  c7: fade-out(rgba(0,0,0,0.7), 0.1);\n  c8: fade-out(rgba(0,0,0,0.8), 0.1);\n  c9: fade-out(rgba(0,0,0,0.9), 0.1);\n  c10: fade-out(rgba(0,0,0,1), 0.1);\n}\n"
        )
        .unwrap(),
        "foo {\n  c0: rgba(0, 0, 0, 0);\n  c1: rgba(0, 0, 0, 0);\n  c2: rgba(0, 0, 0, 0.1);\n  c3: rgba(0, 0, 0, 0.2);\n  c4: rgba(0, 0, 0, 0.3);\n  c5: rgba(0, 0, 0, 0.4);\n  c6: rgba(0, 0, 0, 0.5);\n  c7: rgba(0, 0, 0, 0.6);\n  c8: rgba(0, 0, 0, 0.7);\n  c9: rgba(0, 0, 0, 0.8);\n  c10: rgba(0, 0, 0, 0.9);\n}\n"
    );
}

// From "sass-spec/spec/libsass/color-functions/opacity/opacify.hrx"
#[test]
fn opacify() {
    assert_eq!(rsass("").unwrap(), "");
}

// From "sass-spec/spec/libsass/color-functions/opacity/opacity.hrx"
#[test]
fn opacity() {
    assert_eq!(
        rsass(
            "foo {\n  c0: opacify(rgba(0,0,0,0.0), 0.1);\n  c1: opacify(rgba(0,0,0,0.1), 0.1);\n  c2: opacify(rgba(0,0,0,0.2), 0.1);\n  c3: opacify(rgba(0,0,0,0.3), 0.1);\n  c4: opacify(rgba(0,0,0,0.4), 0.1);\n  c5: opacify(rgba(0,0,0,0.5), 0.1);\n  c6: opacify(rgba(0,0,0,0.6), 0.1);\n  c7: opacify(rgba(0,0,0,0.7), 0.1);\n  c8: opacify(rgba(0,0,0,0.8), 0.1);\n  c9: opacify(rgba(0,0,0,0.9), 0.1);\n  c10: opacify(rgba(0,0,0,1), 0.1);\n}\n"
        )
        .unwrap(),
        "foo {\n  c0: rgba(0, 0, 0, 0.1);\n  c1: rgba(0, 0, 0, 0.2);\n  c2: rgba(0, 0, 0, 0.3);\n  c3: rgba(0, 0, 0, 0.4);\n  c4: rgba(0, 0, 0, 0.5);\n  c5: rgba(0, 0, 0, 0.6);\n  c6: rgba(0, 0, 0, 0.7);\n  c7: rgba(0, 0, 0, 0.8);\n  c8: rgba(0, 0, 0, 0.9);\n  c9: black;\n  c10: black;\n}\n"
    );
}

// From "sass-spec/spec/libsass/color-functions/opacity/transparentize.hrx"
#[test]
fn transparentize() {
    assert_eq!(
        rsass(
            "foo {\n  c0: transparentize(rgba(0,0,0,0.0), 0.1);\n  c1: transparentize(rgba(0,0,0,0.1), 0.1);\n  c2: transparentize(rgba(0,0,0,0.2), 0.1);\n  c3: transparentize(rgba(0,0,0,0.3), 0.1);\n  c4: transparentize(rgba(0,0,0,0.4), 0.1);\n  c5: transparentize(rgba(0,0,0,0.5), 0.1);\n  c6: transparentize(rgba(0,0,0,0.6), 0.1);\n  c7: transparentize(rgba(0,0,0,0.7), 0.1);\n  c8: transparentize(rgba(0,0,0,0.8), 0.1);\n  c9: transparentize(rgba(0,0,0,0.9), 0.1);\n  c10: transparentize(rgba(0,0,0,1), 0.1);\n}\n"
        )
        .unwrap(),
        "foo {\n  c0: rgba(0, 0, 0, 0);\n  c1: rgba(0, 0, 0, 0);\n  c2: rgba(0, 0, 0, 0.1);\n  c3: rgba(0, 0, 0, 0.2);\n  c4: rgba(0, 0, 0, 0.3);\n  c5: rgba(0, 0, 0, 0.4);\n  c6: rgba(0, 0, 0, 0.5);\n  c7: rgba(0, 0, 0, 0.6);\n  c8: rgba(0, 0, 0, 0.7);\n  c9: rgba(0, 0, 0, 0.8);\n  c10: rgba(0, 0, 0, 0.9);\n}\n"
    );
}
