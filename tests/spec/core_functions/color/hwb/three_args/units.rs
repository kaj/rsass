//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/units.hrx"

mod hue {
    #[test]
    fn deg() {
        assert_eq!(
            crate::rsass(
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
