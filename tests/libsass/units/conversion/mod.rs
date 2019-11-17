//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/units/conversion/angle.hrx"
#[test]
#[ignore] // wrong result
fn angle() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass/units/conversion/frequency.hrx"
#[test]
#[ignore] // wrong result
fn frequency() {
    assert_eq!(
        rsass(
            ".result {\
             \n  output: (0Hz + 1Hz);\
             \n  output: (4.2Hz / 1Hz);\
             \n  output: (4.2Hz * 1Hz / 1Hz);\
             \n  output: (0Hz + 1kHz);\
             \n  output: (4.2Hz / 1kHz);\
             \n  output: (4.2Hz * 1kHz / 1Hz);\
             \n  output: (0kHz + 1Hz);\
             \n  output: (4.2kHz / 1Hz);\
             \n  output: (4.2kHz * 1Hz / 1kHz);\
             \n  output: (0kHz + 1kHz);\
             \n  output: (4.2kHz / 1kHz);\
             \n  output: (4.2kHz * 1kHz / 1kHz);\
             \n}\
             \n"
        )
        .unwrap(),
        ".result {\
         \n  output: 1Hz;\
         \n  output: 4.2;\
         \n  output: 4.2Hz;\
         \n  output: 1000Hz;\
         \n  output: 0.0042;\
         \n  output: 4.2kHz;\
         \n  output: 0.001kHz;\
         \n  output: 4200;\
         \n  output: 4.2Hz;\
         \n  output: 1kHz;\
         \n  output: 4.2;\
         \n  output: 4.2kHz;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/units/conversion/resolution.hrx"
#[test]
#[ignore] // wrong result
fn resolution() {
    assert_eq!(
        rsass(
            ".result {\
             \n  output: (0dpi + 1dpi);\
             \n  output: (4.2dpi / 1dpi);\
             \n  output: (4.2dpi * 1dpi / 1dpi);\
             \n  output: (0dpi + 1dpcm);\
             \n  output: (4.2dpi / 1dpcm);\
             \n  output: (4.2dpi * 1dpcm / 1dpi);\
             \n  output: (0dpi + 1dppx);\
             \n  output: (4.2dpi / 1dppx);\
             \n  output: (4.2dpi * 1dppx / 1dpi);\
             \n  output: (0dpcm + 1dpi);\
             \n  output: (4.2dpcm / 1dpi);\
             \n  output: (4.2dpcm * 1dpi / 1dpcm);\
             \n  output: (0dpcm + 1dpcm);\
             \n  output: (4.2dpcm / 1dpcm);\
             \n  output: (4.2dpcm * 1dpcm / 1dpcm);\
             \n  output: (0dpcm + 1dppx);\
             \n  output: (4.2dpcm / 1dppx);\
             \n  output: (4.2dpcm * 1dppx / 1dpcm);\
             \n  output: (0dppx + 1dpi);\
             \n  output: (4.2dppx / 1dpi);\
             \n  output: (4.2dppx * 1dpi / 1dppx);\
             \n  output: (0dppx + 1dpcm);\
             \n  output: (4.2dppx / 1dpcm);\
             \n  output: (4.2dppx * 1dpcm / 1dppx);\
             \n  output: (0dppx + 1dppx);\
             \n  output: (4.2dppx / 1dppx);\
             \n  output: (4.2dppx * 1dppx / 1dppx);\
             \n}\
             \n"
        )
        .unwrap(),
        ".result {\
         \n  output: 1dpi;\
         \n  output: 4.2;\
         \n  output: 4.2dpi;\
         \n  output: 2.54dpi;\
         \n  output: 1.6535433071;\
         \n  output: 4.2dpcm;\
         \n  output: 96dpi;\
         \n  output: 0.04375;\
         \n  output: 4.2dppx;\
         \n  output: 0.3937007874dpcm;\
         \n  output: 10.668;\
         \n  output: 4.2dpi;\
         \n  output: 1dpcm;\
         \n  output: 4.2;\
         \n  output: 4.2dpcm;\
         \n  output: 37.7952755906dpcm;\
         \n  output: 0.111125;\
         \n  output: 4.2dppx;\
         \n  output: 0.0104166667dppx;\
         \n  output: 403.2;\
         \n  output: 4.2dpi;\
         \n  output: 0.0264583333dppx;\
         \n  output: 158.7401574803;\
         \n  output: 4.2dpcm;\
         \n  output: 1dppx;\
         \n  output: 4.2;\
         \n  output: 4.2dppx;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/units/conversion/size.hrx"
#[test]
#[ignore] // wrong result
fn size() {
    assert_eq!(
        rsass(
            ".result {\
             \n  output: (0px + 1px);\
             \n  output: (4.2px / 1px);\
             \n  output: (4.2px * 1px / 1px);\
             \n  output: (0px + 1pt);\
             \n  output: (4.2px / 1pt);\
             \n  output: (4.2px * 1pt / 1px);\
             \n  output: (0px + 1pc);\
             \n  output: (4.2px / 1pc);\
             \n  output: (4.2px * 1pc / 1px);\
             \n  output: (0px + 1mm);\
             \n  output: (4.2px / 1mm);\
             \n  output: (4.2px * 1mm / 1px);\
             \n  output: (0px + 1cm);\
             \n  output: (4.2px / 1cm);\
             \n  output: (4.2px * 1cm / 1px);\
             \n  output: (0px + 1in);\
             \n  output: (4.2px / 1in);\
             \n  output: (4.2px * 1in / 1px);\
             \n  output: (0pt + 1px);\
             \n  output: (4.2pt / 1px);\
             \n  output: (4.2pt * 1px / 1pt);\
             \n  output: (0pt + 1pt);\
             \n  output: (4.2pt / 1pt);\
             \n  output: (4.2pt * 1pt / 1pt);\
             \n  output: (0pt + 1pc);\
             \n  output: (4.2pt / 1pc);\
             \n  output: (4.2pt * 1pc / 1pt);\
             \n  output: (0pt + 1mm);\
             \n  output: (4.2pt / 1mm);\
             \n  output: (4.2pt * 1mm / 1pt);\
             \n  output: (0pt + 1cm);\
             \n  output: (4.2pt / 1cm);\
             \n  output: (4.2pt * 1cm / 1pt);\
             \n  output: (0pt + 1in);\
             \n  output: (4.2pt / 1in);\
             \n  output: (4.2pt * 1in / 1pt);\
             \n  output: (0pc + 1px);\
             \n  output: (4.2pc / 1px);\
             \n  output: (4.2pc * 1px / 1pc);\
             \n  output: (0pc + 1pt);\
             \n  output: (4.2pc / 1pt);\
             \n  output: (4.2pc * 1pt / 1pc);\
             \n  output: (0pc + 1pc);\
             \n  output: (4.2pc / 1pc);\
             \n  output: (4.2pc * 1pc / 1pc);\
             \n  output: (0pc + 1mm);\
             \n  output: (4.2pc / 1mm);\
             \n  output: (4.2pc * 1mm / 1pc);\
             \n  output: (0pc + 1cm);\
             \n  output: (4.2pc / 1cm);\
             \n  output: (4.2pc * 1cm / 1pc);\
             \n  output: (0pc + 1in);\
             \n  output: (4.2pc / 1in);\
             \n  output: (4.2pc * 1in / 1pc);\
             \n  output: (0mm + 1px);\
             \n  output: (4.2mm / 1px);\
             \n  output: (4.2mm * 1px / 1mm);\
             \n  output: (0mm + 1pt);\
             \n  output: (4.2mm / 1pt);\
             \n  output: (4.2mm * 1pt / 1mm);\
             \n  output: (0mm + 1pc);\
             \n  output: (4.2mm / 1pc);\
             \n  output: (4.2mm * 1pc / 1mm);\
             \n  output: (0mm + 1mm);\
             \n  output: (4.2mm / 1mm);\
             \n  output: (4.2mm * 1mm / 1mm);\
             \n  output: (0mm + 1cm);\
             \n  output: (4.2mm / 1cm);\
             \n  output: (4.2mm * 1cm / 1mm);\
             \n  output: (0mm + 1in);\
             \n  output: (4.2mm / 1in);\
             \n  output: (4.2mm * 1in / 1mm);\
             \n  output: (0cm + 1px);\
             \n  output: (4.2cm / 1px);\
             \n  output: (4.2cm * 1px / 1cm);\
             \n  output: (0cm + 1pt);\
             \n  output: (4.2cm / 1pt);\
             \n  output: (4.2cm * 1pt / 1cm);\
             \n  output: (0cm + 1pc);\
             \n  output: (4.2cm / 1pc);\
             \n  output: (4.2cm * 1pc / 1cm);\
             \n  output: (0cm + 1mm);\
             \n  output: (4.2cm / 1mm);\
             \n  output: (4.2cm * 1mm / 1cm);\
             \n  output: (0cm + 1cm);\
             \n  output: (4.2cm / 1cm);\
             \n  output: (4.2cm * 1cm / 1cm);\
             \n  output: (0cm + 1in);\
             \n  output: (4.2cm / 1in);\
             \n  output: (4.2cm * 1in / 1cm);\
             \n  output: (0in + 1px);\
             \n  output: (4.2in / 1px);\
             \n  output: (4.2in * 1px / 1in);\
             \n  output: (0in + 1pt);\
             \n  output: (4.2in / 1pt);\
             \n  output: (4.2in * 1pt / 1in);\
             \n  output: (0in + 1pc);\
             \n  output: (4.2in / 1pc);\
             \n  output: (4.2in * 1pc / 1in);\
             \n  output: (0in + 1mm);\
             \n  output: (4.2in / 1mm);\
             \n  output: (4.2in * 1mm / 1in);\
             \n  output: (0in + 1cm);\
             \n  output: (4.2in / 1cm);\
             \n  output: (4.2in * 1cm / 1in);\
             \n  output: (0in + 1in);\
             \n  output: (4.2in / 1in);\
             \n  output: (4.2in * 1in / 1in);\
             \n}\
             \n"
        )
        .unwrap(),
        ".result {\
         \n  output: 1px;\
         \n  output: 4.2;\
         \n  output: 4.2px;\
         \n  output: 1.3333333333px;\
         \n  output: 3.15;\
         \n  output: 4.2pt;\
         \n  output: 16px;\
         \n  output: 0.2625;\
         \n  output: 4.2pc;\
         \n  output: 3.7795275591px;\
         \n  output: 1.11125;\
         \n  output: 4.2mm;\
         \n  output: 37.7952755906px;\
         \n  output: 0.111125;\
         \n  output: 4.2cm;\
         \n  output: 96px;\
         \n  output: 0.04375;\
         \n  output: 4.2in;\
         \n  output: 0.75pt;\
         \n  output: 5.6;\
         \n  output: 4.2px;\
         \n  output: 1pt;\
         \n  output: 4.2;\
         \n  output: 4.2pt;\
         \n  output: 12pt;\
         \n  output: 0.35;\
         \n  output: 4.2pc;\
         \n  output: 2.8346456693pt;\
         \n  output: 1.4816666667;\
         \n  output: 4.2mm;\
         \n  output: 28.3464566929pt;\
         \n  output: 0.1481666667;\
         \n  output: 4.2cm;\
         \n  output: 72pt;\
         \n  output: 0.0583333333;\
         \n  output: 4.2in;\
         \n  output: 0.0625pc;\
         \n  output: 67.2;\
         \n  output: 4.2px;\
         \n  output: 0.0833333333pc;\
         \n  output: 50.4;\
         \n  output: 4.2pt;\
         \n  output: 1pc;\
         \n  output: 4.2;\
         \n  output: 4.2pc;\
         \n  output: 0.2362204724pc;\
         \n  output: 17.78;\
         \n  output: 4.2mm;\
         \n  output: 2.3622047244pc;\
         \n  output: 1.778;\
         \n  output: 4.2cm;\
         \n  output: 6pc;\
         \n  output: 0.7;\
         \n  output: 4.2in;\
         \n  output: 0.2645833333mm;\
         \n  output: 15.874015748;\
         \n  output: 4.2px;\
         \n  output: 0.3527777778mm;\
         \n  output: 11.905511811;\
         \n  output: 4.2pt;\
         \n  output: 4.2333333333mm;\
         \n  output: 0.9921259843;\
         \n  output: 4.2pc;\
         \n  output: 1mm;\
         \n  output: 4.2;\
         \n  output: 4.2mm;\
         \n  output: 10mm;\
         \n  output: 0.42;\
         \n  output: 4.2cm;\
         \n  output: 25.4mm;\
         \n  output: 0.1653543307;\
         \n  output: 4.2in;\
         \n  output: 0.0264583333cm;\
         \n  output: 158.7401574803;\
         \n  output: 4.2px;\
         \n  output: 0.0352777778cm;\
         \n  output: 119.0551181102;\
         \n  output: 4.2pt;\
         \n  output: 0.4233333333cm;\
         \n  output: 9.9212598425;\
         \n  output: 4.2pc;\
         \n  output: 0.1cm;\
         \n  output: 42;\
         \n  output: 4.2mm;\
         \n  output: 1cm;\
         \n  output: 4.2;\
         \n  output: 4.2cm;\
         \n  output: 2.54cm;\
         \n  output: 1.6535433071;\
         \n  output: 4.2in;\
         \n  output: 0.0104166667in;\
         \n  output: 403.2;\
         \n  output: 4.2px;\
         \n  output: 0.0138888889in;\
         \n  output: 302.4;\
         \n  output: 4.2pt;\
         \n  output: 0.1666666667in;\
         \n  output: 25.2;\
         \n  output: 4.2pc;\
         \n  output: 0.0393700787in;\
         \n  output: 106.68;\
         \n  output: 4.2mm;\
         \n  output: 0.3937007874in;\
         \n  output: 10.668;\
         \n  output: 4.2cm;\
         \n  output: 1in;\
         \n  output: 4.2;\
         \n  output: 4.2in;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/units/conversion/time.hrx"
#[test]
#[ignore] // wrong result
fn time() {
    assert_eq!(
        rsass(
            ".result {\
             \n  output: (0s + 1s);\
             \n  output: (4.2s / 1s);\
             \n  output: (4.2s * 1s / 1s);\
             \n  output: (0s + 1ms);\
             \n  output: (4.2s / 1ms);\
             \n  output: (4.2s * 1ms / 1s);\
             \n  output: (0ms + 1s);\
             \n  output: (4.2ms / 1s);\
             \n  output: (4.2ms * 1s / 1ms);\
             \n  output: (0ms + 1ms);\
             \n  output: (4.2ms / 1ms);\
             \n  output: (4.2ms * 1ms / 1ms);\
             \n}\
             \n"
        )
        .unwrap(),
        ".result {\
         \n  output: 1s;\
         \n  output: 4.2;\
         \n  output: 4.2s;\
         \n  output: 0.001s;\
         \n  output: 4200;\
         \n  output: 4.2ms;\
         \n  output: 1000ms;\
         \n  output: 0.0042;\
         \n  output: 4.2s;\
         \n  output: 1ms;\
         \n  output: 4.2;\
         \n  output: 4.2ms;\
         \n}\
         \n"
    );
}
