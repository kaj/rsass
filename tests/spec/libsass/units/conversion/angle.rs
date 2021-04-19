//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion/angle.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: (0deg + 1deg);\
            \n  output: (4.2deg / 1deg);\
            \n  output: (4.2deg * 1deg / 1deg);\
            \n  output: (0deg + 1grad);\
            \n  output: (4.2deg / 1grad);\
            \n  output: (4.2deg * 1grad / 1deg);\
            \n  output: (0deg + 1rad);\
            \n  output: (4.2deg / 1rad);\
            \n  output: (4.2deg * 1rad / 1deg);\
            \n  output: (0deg + 1turn);\
            \n  output: (4.2deg / 1turn);\
            \n  output: (4.2deg * 1turn / 1deg);\
            \n  output: (0grad + 1deg);\
            \n  output: (4.2grad / 1deg);\
            \n  output: (4.2grad * 1deg / 1grad);\
            \n  output: (0grad + 1grad);\
            \n  output: (4.2grad / 1grad);\
            \n  output: (4.2grad * 1grad / 1grad);\
            \n  output: (0grad + 1rad);\
            \n  output: (4.2grad / 1rad);\
            \n  output: (4.2grad * 1rad / 1grad);\
            \n  output: (0grad + 1turn);\
            \n  output: (4.2grad / 1turn);\
            \n  output: (4.2grad * 1turn / 1grad);\
            \n  output: (0rad + 1deg);\
            \n  output: (4.2rad / 1deg);\
            \n  output: (4.2rad * 1deg / 1rad);\
            \n  output: (0rad + 1grad);\
            \n  output: (4.2rad / 1grad);\
            \n  output: (4.2rad * 1grad / 1rad);\
            \n  output: (0rad + 1rad);\
            \n  output: (4.2rad / 1rad);\
            \n  output: (4.2rad * 1rad / 1rad);\
            \n  output: (0rad + 1turn);\
            \n  output: (4.2rad / 1turn);\
            \n  output: (4.2rad * 1turn / 1rad);\
            \n  output: (0turn + 1deg);\
            \n  output: (4.2turn / 1deg);\
            \n  output: (4.2turn * 1deg / 1turn);\
            \n  output: (0turn + 1grad);\
            \n  output: (4.2turn / 1grad);\
            \n  output: (4.2turn * 1grad / 1turn);\
            \n  output: (0turn + 1rad);\
            \n  output: (4.2turn / 1rad);\
            \n  output: (4.2turn * 1rad / 1turn);\
            \n  output: (0turn + 1turn);\
            \n  output: (4.2turn / 1turn);\
            \n  output: (4.2turn * 1turn / 1turn);\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: 1deg;\
        \n  output: 4.2;\
        \n  output: 4.2deg;\
        \n  output: 0.9deg;\
        \n  output: 4.6666666667;\
        \n  output: 4.2grad;\
        \n  output: 57.2957795131deg;\
        \n  output: 0.0733038286;\
        \n  output: 4.2rad;\
        \n  output: 360deg;\
        \n  output: 0.0116666667;\
        \n  output: 4.2turn;\
        \n  output: 1.1111111111grad;\
        \n  output: 3.78;\
        \n  output: 4.2deg;\
        \n  output: 1grad;\
        \n  output: 4.2;\
        \n  output: 4.2grad;\
        \n  output: 63.6619772368grad;\
        \n  output: 0.0659734457;\
        \n  output: 4.2rad;\
        \n  output: 400grad;\
        \n  output: 0.0105;\
        \n  output: 4.2turn;\
        \n  output: 0.0174532925rad;\
        \n  output: 240.6422739549;\
        \n  output: 4.2deg;\
        \n  output: 0.0157079633rad;\
        \n  output: 267.3803043944;\
        \n  output: 4.2grad;\
        \n  output: 1rad;\
        \n  output: 4.2;\
        \n  output: 4.2rad;\
        \n  output: 6.2831853072rad;\
        \n  output: 0.668450761;\
        \n  output: 4.2turn;\
        \n  output: 0.0027777778turn;\
        \n  output: 1512;\
        \n  output: 4.2deg;\
        \n  output: 0.0025turn;\
        \n  output: 1680;\
        \n  output: 4.2grad;\
        \n  output: 0.1591549431turn;\
        \n  output: 26.3893782902;\
        \n  output: 4.2rad;\
        \n  output: 1turn;\
        \n  output: 4.2;\
        \n  output: 4.2turn;\
        \n}\
        \n"
    );
}
