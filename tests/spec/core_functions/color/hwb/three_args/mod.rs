//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/color/hwb/three_args/named.hrx"
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        rsass(
            "@use \'sass:color\';\
            \na {b: color.hwb($hue: 0, $whiteness: 30%, $blackness: 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #994d4d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/core_functions/color/hwb/three_args/units.hrx"
mod units {
    #[allow(unused)]
    use super::rsass;
    mod hue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn deg() {
            assert_eq!(
                rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0deg, 30%, 40%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #994d4d;\
        \n}\
        \n"
            );
        }
    }
}

mod w3c;
