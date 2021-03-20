//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color.hrx"

#[test]
fn adjust() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.adjust(#abcdef, $red: 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b5cdef;\
        \n}\
        \n"
    );
}
#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.alpha(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn blue() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.blue(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 239;\
        \n}\
        \n"
    );
}
#[test]
fn change() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.change(#abcdef, $red: 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0acdef;\
        \n}\
        \n"
    );
}
#[test]
fn complement() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.complement(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #efcdab;\
        \n}\
        \n"
    );
}
mod css_overloads {
    mod alpha {
        #[test]
        fn multi_arg() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\";\
            \na {b: color.alpha(c=d, e=f, g=h)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
            );
        }
        #[test]
        fn one_arg() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:color\";\
            \na {b: color.alpha(c=d)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn grayscale() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.grayscale(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: grayscale(1);\
        \n}\
        \n"
        );
    }
    #[test]
    fn invert() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.invert(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: invert(1);\
        \n}\
        \n"
        );
    }
    #[test]
    fn opacity() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:color\";\
            \na {b: color.opacity(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: opacity(1);\
        \n}\
        \n"
        );
    }
}
mod error {

    // Ignoring "adjust_color", error tests are not supported yet.

    // Ignoring "adjust_hue", error tests are not supported yet.

    // Ignoring "change_color", error tests are not supported yet.

    // Ignoring "darken", error tests are not supported yet.

    // Ignoring "desaturate", error tests are not supported yet.

    // Ignoring "fade_in", error tests are not supported yet.

    // Ignoring "fade_out", error tests are not supported yet.

    // Ignoring "lighten", error tests are not supported yet.

    // Ignoring "opacify", error tests are not supported yet.

    // Ignoring "saturate", error tests are not supported yet.

    // Ignoring "scale_color", error tests are not supported yet.

    // Ignoring "transparentize", error tests are not supported yet.
}
#[test]
fn green() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.green(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 205;\
        \n}\
        \n"
    );
}
#[test]
fn hue() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.hue(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 210deg;\
        \n}\
        \n"
    );
}
#[test]
fn ie_hex_str() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.ie-hex-str(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #FFABCDEF;\
        \n}\
        \n"
    );
}
#[test]
fn invert() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.invert(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #543210;\
        \n}\
        \n"
    );
}
#[test]
fn lightness() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.lightness(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 80.3921568627%;\
        \n}\
        \n"
    );
}
#[test]
fn mix() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.mix(#abcdef, #daddee)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #c3d5ef;\
        \n}\
        \n"
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.red(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 171;\
        \n}\
        \n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.saturation(#abcdef)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 68%;\
        \n}\
        \n"
    );
}
#[test]
fn scale() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:color\";\
            \na {b: color.scale(#abcdef, $red: 10%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b3cdef;\
        \n}\
        \n"
    );
}
