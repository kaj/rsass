//! Color names from <https://www.w3.org/TR/css3-color/>
#![allow(clippy::unreadable_literal)]
use crate::output::{Format, Formatted};
use crate::value::Number;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{self, Display};

/// A color defined by red, green, blue, and alpha components.
#[derive(Clone, Debug)]
pub struct Rgba {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
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
    pub fn new(r: f64, g: f64, b: f64, a: f64, s: RgbFormat) -> Self {
        let ff = 255.;
        Self {
            red: cap(r, ff),
            green: cap(g, ff),
            blue: cap(b, ff),
            alpha: cap(a, 1.),
            source: s,
        }
    }
    /// Create a color from rgb byte values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
            alpha: 1.0,
            source: RgbFormat::LongHex,
        }
    }
    /// Create a color from rgba byte values.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
            alpha: f64::from(a) / 255.,
            source: RgbFormat::LongHex,
        }
    }

    fn is_opaque(&self) -> bool {
        self.alpha >= 1.
    }
    /// If this color is equal to a named color, get the name.
    pub fn name(&self) -> Option<&'static str> {
        self.try_bytes()
            .map(|(r, g, b)| u32::from_be_bytes([0, r, g, b]))
            .and_then(|c| LOOKUP.v2n.get(&c).copied())
    }
    /// If `name` is a known color name, get the corresponding rgba value.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.to_lowercase();
        let name: &str = &name;
        if name == "transparent" {
            return Some(Self::new(0., 0., 0., 0., RgbFormat::Name));
        }
        LOOKUP.n2v.get(name).map(|n| {
            let [_, r, g, b] = n.to_be_bytes();
            Self::new(r.into(), g.into(), b.into(), 1., RgbFormat::Name)
        })
    }

    /// Return true if all chanels are zero.
    pub fn all_zero(&self) -> bool {
        self.alpha == 0.
            && self.red == 0.
            && self.green == 0.
            && self.blue == 0.
    }
    pub(crate) fn is_integer(&self) -> bool {
        near_integer(self.red)
            && near_integer(self.green)
            && near_integer(self.blue)
            && self.is_opaque()
    }
    /// Get a (r, g, b, a) byte-value tuple for this color.
    pub fn to_bytes(&self) -> (u8, u8, u8, u8) {
        fn byte(v: f64) -> u8 {
            v.round() as u8
        }
        fn fb(v: f64) -> u8 {
            v.round() as u8
        }
        let a = self.alpha * 255.;
        (byte(self.red), byte(self.green), byte(self.blue), fb(a))
    }
    /// Get a (r, g, b) byte-value tuple for this color.
    ///
    /// If the color is not opaque or not exactly equal to a byte
    /// value, return None.
    pub fn try_bytes(&self) -> Option<(u8, u8, u8)> {
        if !self.is_opaque() {
            return None;
        }
        fn byte(v: f64) -> Option<u8> {
            if (v.round() - v).abs() < 1e-7 {
                Some(v.round() as u8)
            } else {
                None
            }
        }
        if let (Some(r), Some(g), Some(b)) =
            (byte(self.red), byte(self.green), byte(self.blue))
        {
            Some((r, g, b))
        } else {
            None
        }
    }
    /// Get the red component.
    pub fn red(&self) -> f64 {
        self.red
    }
    /// Get the green component.
    pub fn green(&self) -> f64 {
        self.green
    }
    /// Get the blue component.
    pub fn blue(&self) -> f64 {
        self.blue
    }
    /// Get the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    /// Set the alpha value of this color.
    ///
    /// Zero is fully transparent, one is fully opaque.
    pub fn set_alpha(&mut self, alpha: f64) {
        self.alpha = alpha.clamp(0., 1.);
    }

    pub(crate) fn invert(&self, weight: f64) -> Self {
        let inv = |v: f64| -(v - 255.) * weight + v * (1. - weight);
        Rgba::new(
            inv(self.red()),
            inv(self.green()),
            inv(self.blue()),
            self.alpha(),
            self.source(),
        )
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
        cmp_chan(self.red, other.red)
            .then_with(|| cmp_chan(self.green, other.green))
            .then_with(|| cmp_chan(self.blue, other.blue))
            .then_with(|| cmp_chan(self.alpha, other.alpha))
    }
}
impl PartialOrd for Rgba {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_chan(a: f64, b: f64) -> Ordering {
    if (a - b).abs() < 1e-7 {
        Ordering::Equal
    } else {
        match (a.is_nan(), b.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => a.partial_cmp(&b).unwrap(),
        }
    }
}

fn cap(n: f64, max: f64) -> f64 {
    f64::min(f64::max(0., n), max)
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

impl Display for Formatted<'_, Rgba> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // The byte-version of alpha is not used here.
        let rgba = self.value;
        if let Some((r, g, b)) = rgba.try_bytes() {
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
        } else {
            write_rgba(rgba, out, self.format)
        }
    }
}

fn write_rgba(
    rgba: &Rgba,
    out: &mut fmt::Formatter,
    format: Format,
) -> fmt::Result {
    let r = Number::from(rgba.red);
    let g = Number::from(rgba.green);
    let b = Number::from(rgba.blue);
    let r = r.format(format);
    let g = g.format(format);
    let b = b.format(format);
    if rgba.is_opaque() {
        if format.is_compressed() {
            write!(out, "rgb({r},{g},{b})")
        } else {
            write!(out, "rgb({r}, {g}, {b})")
        }
    } else {
        let a = Number::from(rgba.alpha);
        let a = a.format(format);
        if format.is_compressed() {
            write!(out, "rgba({r},{g},{b},{a})")
        } else {
            write!(out, "rgba({r}, {g}, {b}, {a})")
        }
    }
}

fn near_integer(v: f64) -> bool {
    (v - v.round()).abs() < 1e-7
}
