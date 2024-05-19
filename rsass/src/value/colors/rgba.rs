//! Color names from <https://www.w3.org/TR/css3-color/>
#![allow(clippy::unreadable_literal)]
use super::Rational;
use crate::output::{Format, Formatted};
use crate::value::Number;
use lazy_static::lazy_static;
use num_traits::{one, zero, One, Signed, Zero};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{self, Display};

/// A color defined by red, green, blue, and alpha components.
#[derive(Clone, Debug)]
pub struct Rgba {
    red: Rational,
    green: Rational,
    blue: Rational,
    alpha: Rational,
    source: RgbFormat,
}

/// The source format of an rgb color.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RgbFormat {
    /// Six digit hexadecimal
    LongHex,
    /// Three digit hexadecimal
    ShortHex,
    /// A well-known named color
    Name,
    /// A rgb() or rgba() color specification
    Rgb,
}

impl Rgba {
    /// Create a new rgba color.
    pub fn new(
        r: Rational,
        g: Rational,
        b: Rational,
        a: Rational,
        s: RgbFormat,
    ) -> Self {
        let ff = Rational::new(255, 1);
        let one = Rational::one();
        Self {
            red: cap(r, &ff),
            green: cap(g, &ff),
            blue: cap(b, &ff),
            alpha: cap(a, &one),
            source: s,
        }
    }
    /// Create a color from rgb byte values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: Rational::from_integer(r.into()),
            green: Rational::from_integer(g.into()),
            blue: Rational::from_integer(b.into()),
            alpha: Rational::one(),
            source: RgbFormat::LongHex,
        }
    }
    /// Create a color from rgba byte values.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: Rational::from_integer(r.into()),
            green: Rational::from_integer(g.into()),
            blue: Rational::from_integer(b.into()),
            alpha: Rational::from_integer(a.into()) / 255,
            source: RgbFormat::LongHex,
        }
    }

    /// If this color is equal to a named color, get the name.
    ///
    /// Each component is rounded to its byte value before lookup.
    pub fn name(&self) -> Option<&'static str> {
        if self.alpha >= Rational::one() {
            let (r, g, b, _a) = self.to_bytes();
            let c = u32::from_be_bytes([0, r, g, b]);
            LOOKUP.v2n.get(&c).copied()
        } else {
            None
        }
    }
    /// If `name` is a known color name, get the corresponding rgba value.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        let name: &str = &name;
        if name == "transparent" {
            return Some(Self::new(
                zero(),
                zero(),
                zero(),
                zero(),
                RgbFormat::Name,
            ));
        }
        LOOKUP.n2v.get(name).map(|n| {
            let [_, r, g, b] = n.to_be_bytes();
            Self::new(
                i64::from(r).into(),
                i64::from(g).into(),
                i64::from(b).into(),
                one(),
                RgbFormat::Name,
            )
        })
    }

    /// Return true if all chanels are zero.
    pub fn all_zero(&self) -> bool {
        self.alpha.is_zero()
            && self.red.is_zero()
            && self.green.is_zero()
            && self.blue.is_zero()
    }
    /// Get a (r, g, b, a) byte-value tuple for this color.
    pub fn to_bytes(&self) -> (u8, u8, u8, u8) {
        fn byte(v: Rational) -> u8 {
            v.round().to_integer() as u8
        }
        let a = self.alpha * 255;
        (byte(self.red), byte(self.green), byte(self.blue), byte(a))
    }
    /// Get the red component.
    pub fn red(&self) -> Rational {
        self.red
    }
    /// Get the green component.
    pub fn green(&self) -> Rational {
        self.green
    }
    /// Get the blue component.
    pub fn blue(&self) -> Rational {
        self.blue
    }
    /// Get the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn alpha(&self) -> Rational {
        self.alpha
    }
    /// Set the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn set_alpha(&mut self, alpha: Rational) {
        self.alpha = cap(alpha, &one());
    }
    /// Get the source type of this color.
    pub(crate) fn source(&self) -> RgbFormat {
        self.source
    }
    pub(crate) fn reset_source(&mut self) {
        self.source = RgbFormat::Name;
    }
    /// Get a reference to this `Value` bound to an output format.
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }
}

impl PartialEq for Rgba {
    fn eq(&self, other: &Self) -> bool {
        // ignores source!
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}
impl Eq for Rgba {}
impl Ord for Rgba {
    fn cmp(&self, other: &Self) -> Ordering {
        // ignores source!
        self.red
            .cmp(&other.red)
            .then_with(|| self.green.cmp(&other.green))
            .then_with(|| self.blue.cmp(&other.blue))
            .then_with(|| self.alpha.cmp(&other.alpha))
    }
}
impl PartialOrd for Rgba {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cap(n: Rational, max: &Rational) -> Rational {
    if n > *max {
        *max
    } else if n.is_negative() {
        Rational::zero()
    } else {
        n
    }
}

#[test]
fn get_black() {
    assert_eq!(Rgba::from_rgb(0, 0, 0).name(), Some("black"))
}

#[test]
fn get_none() {
    assert_eq!(Rgba::from_rgb(0, 1, 2).name(), None)
}

#[test]
fn get_red_by_name() {
    assert_eq!(Some(Rgba::from_rgb(255, 0, 0)), Rgba::from_name("red"));
}

#[test]
fn get_none_by_name() {
    assert_eq!(None, Rgba::from_name("xyzzy"));
}

struct Lookup {
    n2v: BTreeMap<&'static str, u32>,
    v2n: BTreeMap<u32, &'static str>,
}

impl Lookup {
    fn from_slice(data: &[(&'static str, u32)]) -> Self {
        let mut n2v: BTreeMap<&'static str, u32> = BTreeMap::new();
        let mut v2n: BTreeMap<u32, &'static str> = BTreeMap::new();
        for &(n, v) in data {
            n2v.insert(n, v);
            v2n.entry(v).or_insert(n);
        }
        Self { n2v, v2n }
    }
}

lazy_static! {
    static ref LOOKUP: Lookup = Lookup::from_slice(&[
        ("aliceblue", 0xf0f8ff_u32),
        ("antiquewhite", 0xfaebd7),
        ("aqua", 0x00ffff),
        ("aquamarine", 0x7fffd4),
        ("azure", 0xf0ffff),
        ("beige", 0xf5f5dc),
        ("bisque", 0xffe4c4),
        ("black", 0x000000),
        ("blanchedalmond", 0xffebcd),
        ("blue", 0x0000ff),
        ("blueviolet", 0x8a2be2),
        ("brown", 0xa52a2a),
        ("burlywood", 0xdeb887),
        ("cadetblue", 0x5f9ea0),
        ("chartreuse", 0x7fff00),
        ("chocolate", 0xd2691e),
        ("coral", 0xff7f50),
        ("cornflowerblue", 0x6495ed),
        ("cornsilk", 0xfff8dc),
        ("crimson", 0xdc143c),
        ("cyan", 0x00ffff),
        ("darkblue", 0x00008b),
        ("darkcyan", 0x008b8b),
        ("darkgoldenrod", 0xb8860b),
        ("darkgray", 0xa9a9a9),
        ("darkgreen", 0x006400),
        ("darkgrey", 0xa9a9a9),
        ("darkkhaki", 0xbdb76b),
        ("darkmagenta", 0x8b008b),
        ("darkolivegreen", 0x556b2f),
        ("darkorange", 0xff8c00),
        ("darkorchid", 0x9932cc),
        ("darkred", 0x8b0000),
        ("darksalmon", 0xe9967a),
        ("darkseagreen", 0x8fbc8f),
        ("darkslateblue", 0x483d8b),
        ("darkslategray", 0x2f4f4f),
        ("darkslategrey", 0x2f4f4f),
        ("darkturquoise", 0x00ced1),
        ("darkviolet", 0x9400d3),
        ("deeppink", 0xff1493),
        ("deepskyblue", 0x00bfff),
        ("dimgrey", 0x696969),
        ("dimgray", 0x696969),
        ("dodgerblue", 0x1e90ff),
        ("firebrick", 0xb22222),
        ("floralwhite", 0xfffaf0),
        ("forestgreen", 0x228b22),
        ("fuchsia", 0xff00ff),
        ("gainsboro", 0xdcdcdc),
        ("ghostwhite", 0xf8f8ff),
        ("gold", 0xffd700),
        ("goldenrod", 0xdaa520),
        ("gray", 0x808080),
        ("green", 0x008000),
        ("greenyellow", 0xadff2f),
        ("grey", 0x808080),
        ("honeydew", 0xf0fff0),
        ("hotpink", 0xff69b4),
        ("indianred", 0xcd5c5c),
        ("indigo", 0x4b0082),
        ("ivory", 0xfffff0),
        ("khaki", 0xf0e68c),
        ("lavender", 0xe6e6fa),
        ("lavenderblush", 0xfff0f5),
        ("lawngreen", 0x7cfc00),
        ("lemonchiffon", 0xfffacd),
        ("lightblue", 0xadd8e6),
        ("lightcoral", 0xf08080),
        ("lightcyan", 0xe0ffff),
        ("lightgoldenrodyellow", 0xfafad2),
        ("lightgray", 0xd3d3d3),
        ("lightgreen", 0x90ee90),
        ("lightgrey", 0xd3d3d3),
        ("lightpink", 0xffb6c1),
        ("lightsalmon", 0xffa07a),
        ("lightseagreen", 0x20b2aa),
        ("lightskyblue", 0x87cefa),
        ("lightslategray", 0x778899),
        ("lightslategrey", 0x778899),
        ("lightsteelblue", 0xb0c4de),
        ("lightyellow", 0xffffe0),
        ("lime", 0x00ff00),
        ("limegreen", 0x32cd32),
        ("linen", 0xfaf0e6),
        ("magenta", 0xff00ff),
        ("maroon", 0x800000),
        ("mediumaquamarine", 0x66cdaa),
        ("mediumblue", 0x0000cd),
        ("mediumorchid", 0xba55d3),
        ("mediumpurple", 0x9370db),
        ("mediumseagreen", 0x3cb371),
        ("mediumslateblue", 0x7b68ee),
        ("mediumspringgreen", 0x00fa9a),
        ("mediumturquoise", 0x48d1cc),
        ("mediumvioletred", 0xc71585),
        ("midnightblue", 0x191970),
        ("mintcream", 0xf5fffa),
        ("mistyrose", 0xffe4e1),
        ("moccasin", 0xffe4b5),
        ("navajowhite", 0xffdead),
        ("navy", 0x000080),
        ("oldlace", 0xfdf5e6),
        ("olive", 0x808000),
        ("olivedrab", 0x6b8e23),
        ("orange", 0xffa500),
        ("orangered", 0xff4500),
        ("orchid", 0xda70d6),
        ("palegoldenrod", 0xeee8aa),
        ("palegreen", 0x98fb98),
        ("paleturquoise", 0xafeeee),
        ("palevioletred", 0xdb7093),
        ("papayawhip", 0xffefd5),
        ("peachpuff", 0xffdab9),
        ("peru", 0xcd853f),
        ("pink", 0xffc0cb),
        ("plum", 0xdda0dd),
        ("powderblue", 0xb0e0e6),
        ("purple", 0x800080),
        ("rebeccapurple", 0x663399),
        ("red", 0xff0000),
        ("rosybrown", 0xbc8f8f),
        ("royalblue", 0x4169e1),
        ("saddlebrown", 0x8b4513),
        ("salmon", 0xfa8072),
        ("sandybrown", 0xf4a460),
        ("seagreen", 0x2e8b57),
        ("seashell", 0xfff5ee),
        ("sienna", 0xa0522d),
        ("silver", 0xc0c0c0),
        ("skyblue", 0x87ceeb),
        ("slateblue", 0x6a5acd),
        ("slategray", 0x708090),
        ("slategrey", 0x708090),
        ("snow", 0xfffafa),
        ("springgreen", 0x00ff7f),
        ("steelblue", 0x4682b4),
        ("tan", 0xd2b48c),
        ("teal", 0x008080),
        ("thistle", 0xd8bfd8),
        ("tomato", 0xff6347),
        ("turquoise", 0x40e0d0),
        ("violet", 0xee82ee),
        ("wheat", 0xf5deb3),
        ("white", 0xffffff),
        ("whitesmoke", 0xf5f5f5),
        ("yellow", 0xffff00),
        ("yellowgreen", 0x9acd32),
    ]);
}

impl<'a> Display for Formatted<'a, Rgba> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // The byte-version of alpha is not used here.
        let rgba = self.value;
        let (r, g, b, _a) = rgba.to_bytes();
        let a = self.value.alpha;
        if a >= Rational::one() {
            // E.g. #ff00cc can be written #f0c in css.
            // 0xff / 0x11 = 0xf.
            let short = r % 0x11 == 0 && g % 0x11 == 0 && b % 0x11 == 0;
            let hex_len = if short { 4 } else { 7 };
            if self.format.is_compressed() {
                if let Some(name) = rgba.name() {
                    if name.len() <= hex_len {
                        return name.fmt(out);
                    }
                }
                if short {
                    write!(out, "#{:x}{:x}{:x}", r / 0x11, g / 0x11, b / 0x11)
                } else {
                    write!(out, "#{r:02x}{g:02x}{b:02x}")
                }
            } else {
                match rgba.source {
                    RgbFormat::LongHex => {
                        write!(out, "#{r:02x}{g:02x}{b:02x}")
                    }
                    RgbFormat::ShortHex => {
                        write!(
                            out,
                            "#{:x}{:x}{:x}",
                            r / 0x11,
                            g / 0x11,
                            b / 0x11
                        )
                    }
                    RgbFormat::Name => {
                        if let Some(name) = rgba.name() {
                            return name.fmt(out);
                        }
                        write!(out, "#{r:02x}{g:02x}{b:02x}")
                    }
                    RgbFormat::Rgb => {
                        write!(out, "rgb({r}, {g}, {b})")
                    }
                }
            }
        } else if self.format.is_compressed() && rgba.all_zero() {
            write!(out, "transparent")
        } else if self.format.is_compressed() {
            // Note: libsass does not use the format for the alpha like this.
            let a = Number::from(a);
            write!(out, "rgba({},{},{},{})", r, g, b, a.format(self.format))
        } else {
            let a = Number::from(a);
            write!(
                out,
                "rgba({}, {}, {}, {})",
                r,
                g,
                b,
                a.format(self.format)
            )
        }
    }
}
