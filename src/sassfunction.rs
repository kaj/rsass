use formalargs::{CallArgs, FormalArgs};
use num_rational::Rational;
use num_traits::Zero;
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn get_function(name: &str) -> Option<&SassFunction> {
    let name = name.replace("-", "_");
    let name: &str = &name;
    FUNCTIONS.get(name)
}

pub struct SassFunction {
    args: FormalArgs,
    body: Box<Fn(&Scope) -> Value + Send + Sync>,
}

impl SassFunction {
    pub fn call(&self, scope: &mut Scope, args: &CallArgs) -> Value {
        let s = self.args.eval(scope, args);
        (self.body)(&s)
    }
}

macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), Value::Null)
    };
    ($name:ident = $value:expr) => {{
        use valueexpression::value_expression;
        (stringify!($name).into(), value_expression($value).unwrap().1)
    }};
}

macro_rules! func {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {
        SassFunction {
            args: FormalArgs::new(vec![ $( one_arg!($arg $( = $value)* ) ),* ]),
            body: Box::new($body),
        }
    };
}

lazy_static! {
    static ref FUNCTIONS: BTreeMap<&'static str, SassFunction> = {
        let mut f = BTreeMap::new();
        f.insert("rgb", func!((red, green, blue), |s| {
            Value::Color(to_int(s.get("red")),
                         to_int(s.get("green")),
                         to_int(s.get("blue")),
                         Rational::from_integer(1),
                         None)
        }));
        f.insert("rgba", func!((red, green, blue, alpha), |s: &Scope| {
            let red = s.get("red");
            let alpha = s.get("alpha");
            if let Value::Color(r, g, b, _, _) = red {
                let a = to_rational(if alpha == Value::Null { s.get("green") }
                                    else { alpha });
                Value::Color(r, g, b, a, None)
            } else {
                Value::Color(to_int(red),
                             to_int(s.get("green")),
                             to_int(s.get("blue")),
                             to_rational(alpha),
                             None)
            }
        }));
        f.insert("hsl", func!((hue, saturation, lightness), |s: &Scope| {
            let hue = to_rational(s.get("hue")) * Rational::new(1, 360);
            let sat = to_rational_percent(s.get("saturation"));
            let light = to_rational_percent(s.get("lightness"));
            let (r, g, b) = hsl_to_rgb(hue, sat, light);
            let one = Rational::from_integer(1);
            Value::Color(frac_to_int(r),
                         frac_to_int(g),
                         frac_to_int(b),
                         one,
                         None)
        }));
        f.insert("hsla", func!((hue, saturation, lightness, alpha),
                               |s: &Scope| {
            let hue = to_rational(s.get("hue")) * Rational::new(1, 360);
            let sat = to_rational_percent(s.get("saturation"));
            let light = to_rational_percent(s.get("lightness"));
            let alpha = s.get("alpha");
            let (r, g, b) = hsl_to_rgb(hue, sat, light);
            Value::Color(frac_to_int(r),
                         frac_to_int(g),
                         frac_to_int(b),
                         to_rational(alpha),
                         None)
        }));
        f.insert("red", func!((color), |s: &Scope| {
            match s.get("color") {
                Value::Color(red, _, _, _, _) => {
                    Value::Numeric(b2rat(red), Unit::None, true)
                }
                value => value,
            }
        }));
        f.insert("green", func!((color), |s: &Scope| {
            match s.get("color") {
                Value::Color(_, green, _, _, _) => {
                    Value::Numeric(b2rat(green), Unit::None, true)
                }
                value => value,
            }
        }));
        f.insert("blue", func!((color), |s: &Scope| {
            match s.get("color") {
                Value::Color(_, _, blue, _, _) => {
                    Value::Numeric(b2rat(blue), Unit::None, true)
                }
                value => value,
            }
        }));
        f.insert("invert", func!((color), |s: &Scope| {
            let color = s.get("color");
            if let &Value::Color(ref r, ref g, ref b, ref a, _) = &color {
                Value::Color(0xff - r, 0xff - g, 0xff - b, *a, None)
            } else {
                panic!(format!("Unexpected arguments to invert: ({:?})", color))
            }
        }));
        f.insert("mix", func!((color1, color2, weight=b"50%"), |s| {
            let color1 = s.get("color1");
            let color2 = s.get("color2");
            let weight = s.get("weight");
            if let (&Value::Color(ref r1, ref g1, ref b1, ref a1, _),
                    &Value::Color(ref r2, ref g2, ref b2, ref a2, _),
                    &Value::Numeric(ref w, ref wu, _)) =
                (&color1, &color2, &weight) {
                let w = if wu == &Unit::Percent {
                    w / Rational::from_integer(100)
                } else {
                    w.clone()
                };
                let one = Rational::from_integer(1);
                let w2 = one - (one - w * a1) * a2;
                fn mv(v1: u8, v2: u8, w: Rational) -> Rational {
                    mr(Rational::from_integer(v1 as isize),
                       Rational::from_integer(v2 as isize),
                       w)
                }
                fn mr(v1: Rational, v2: Rational, w: Rational) -> Rational {
                    v1 * w + v2 * (Rational::from_integer(1) - w)
                }
                fn rb(r: Rational) -> u8 {
                    cap_u8(r.round().to_integer())
                }
                Value::Color(rb(mv(*r1, *r2, w2)),
                             rb(mv(*g1, *g2, w2)),
                             rb(mv(*b1, *b2, w2)),
                             mr(*a1, *a2, w),
                             None)
            } else {
                panic!(format!("Unexpected arguments to mix: \
                                ({:?}, {:?}, {:?})",
                               color1,
                               color2,
                               weight))
            }
        }));
        f.insert("adjust_color",
                 func!((color, red, green, blue,
                        hue, saturation, lightness, alpha),
                       |s: &Scope| {
            fn c_comb(orig: u8, x: Value) -> u8 {
                match x {
                    Value::Null => orig,
                    Value::Numeric(..) => {
                        cap_u8(orig as isize +
                               to_rational(x).round().to_integer())
                    }
                    _ => orig, // Or error?
                }
            }
            fn a_comb(orig: Rational, x: Value) -> Rational {
                match x {
                    Value::Null => orig,
                    Value::Numeric(..) => orig + to_rational(x),
                    _ => orig, // Or error?
                }
            }
            fn sl_comb(orig: Rational, x: Value) -> Rational {
                match x {
                    Value::Null => orig,
                    Value::Numeric(..) => orig + to_rational_percent(x),
                    _ => orig, // Or error?
                }
            }
            match s.get("color") {
                Value::Color(red, green, blue, alpha, _) => {
                    let h_adj = s.get("hue");
                    let s_adj = s.get("saturation");
                    let l_adj = s.get("lightness");
                    let a_adj = s.get("alpha");
                    if h_adj != Value::Null
                        || s_adj != Value::Null
                        || l_adj != Value::Null {
                        let (h, s, l) = rgb_to_hsl(u8_to_frac(red),
                                                   u8_to_frac(green),
                                                   u8_to_frac(blue));
                        let h = a_comb(h, h_adj);
                        let s = sl_comb(s, s_adj);
                        let l = sl_comb(l, l_adj);
                        println!("Adjusted to hsl({}, {}, {})", h, s, l);
                        let (r, g, b) =
                            hsl_to_rgb(h * Rational::new(1, 360), s, l);
                        Value::Color(frac_to_int(r),
                                     frac_to_int(g),
                                     frac_to_int(b),
                                     a_comb(alpha, a_adj),
                                     None)
                    } else {
                        Value::Color(c_comb(red, s.get("red")),
                                     c_comb(green, s.get("green")),
                                     c_comb(blue, s.get("blue")),
                                     a_comb(alpha, s.get("alpha")),
                                     None)
                    }
                }
                v => v,
            }
        }));
        f.insert("type_of", func!((value), |s: &Scope| {
            let value = s.get("value");
            Value::Literal(
                match value {
                    Value::Color(..) => "color",
                    Value::Literal(..) => "string",
                    Value::Numeric(..) => "number",
                    _ => "unknown",
                }.into(),
                false)
        }));
        f.insert("if", func!((condition, if_true, if_false), |s| {
            if s.get("condition").is_true() {
                s.get("if_true")
            } else {
                s.get("if_false")
            }
        }));

        f.insert("quote", func!((contents), |s| {
            match s.get("contents") {
                Value::Literal(v, _) => Value::Literal(v, true),
                v => Value::Literal(format!("{}", v), true),
            }
        }));
        f.insert("unquote", func!((contents), |s| {
            match s.get("contents") {
                Value::Literal(v, _) => Value::Literal(v, false),
                v => v,
            }
        }));
        f
    };
}

/// Convert hue (degrees) / sat (0 .. 1) / lighness (0 .. 1) ro rgb (0 .. 1)
fn hsl_to_rgb(h: Rational,
              s: Rational,
              l: Rational)
              -> (Rational, Rational, Rational) {
    println!("hsl({}, {}, {})", h, s, l);
    let one = Rational::from_integer(1);
    if s.is_zero() {
        (l, l, l)
    } else {
        fn hue2rgb(p: Rational, q: Rational, t: Rational) -> Rational {
            let t = t - t.floor();
            if t < Rational::new(1, 6) {
                p + (q - p) * Rational::from_integer(6) * t
            } else if t < Rational::new(1, 2) {
                q
            } else if t < Rational::new(2, 3) {
                p +
                (q - p) * (Rational::new(2, 3) - t) * Rational::from_integer(6)
            } else {
                p
            }
        }
        let q = if l < Rational::new(1, 2) {
            l * (one + s)
        } else {
            l + s - l * s
        };
        let p = Rational::from_integer(2) * l - q;

        (hue2rgb(p, q, h + Rational::new(1, 3)),
         hue2rgb(p, q, h),
         hue2rgb(p, q, h - Rational::new(1, 3)))
    }
}

/// Convert rgb (0 .. 1) to hue (degrees) / sat (0 .. 1) / lighness (0 .. 1)
fn rgb_to_hsl(r: Rational,
              g: Rational,
              b: Rational)
              -> (Rational, Rational, Rational) {
    let (max, min) = {
        let v = [(0, r), (0, g), (0, b)];
        (v.iter().max().unwrap().clone(), v.iter().min().unwrap().clone())
    };
    let largest = max.0;
    let (max, min) = (max.1, min.1);
    let half = Rational::new(1, 2);
    let mid = (max + min) * half;

    if max == min {
        (Rational::zero(), Rational::zero(), mid)
    } else {
        let d = max - min;
        let s = if mid > half {
            d / (Rational::from_integer(2) - max - min)
        } else {
            d / (max + min)
        };
        let h = match largest {
            0 => {
                (g - b) / d +
                if g < b {
                    Rational::from_integer(6)
                } else {
                    Rational::zero()
                }
            }
            1 => (b - r) / d + Rational::from_integer(2),
            _ => (r - g) / d + Rational::from_integer(4),
        } * Rational::new(360, 6);
        // h /= 6;
        println!("rgb({}, {}, {}) => hsl({}, {}, {})", r, g, b, h, s, mid);
        (h, s, mid)
    }
}

fn b2rat(byte: u8) -> Rational {
    Rational::from_integer(byte as isize)
}

fn to_int(v: Value) -> u8 {
    cap_u8(match v {
        Value::Numeric(v, Unit::Percent, _) => {
            (Rational::new(255, 100) * v).to_integer()
        }
        Value::Numeric(v, _, _) => v.round().to_integer(),
        v => format!("{}", v).parse().unwrap(),
    })
}

/// Convert a value in the 0 .. 1 range to u8
fn frac_to_int(v: Rational) -> u8 {
    cap_u8((Rational::from_integer(255) * v).round().to_integer())
}

fn u8_to_frac(v: u8) -> Rational {
    Rational::new(v as isize, 255)
}

fn cap_u8(n: isize) -> u8 {
    if n > 255 {
        255
    } else if n < 0 {
        0
    } else {
        n as u8
    }
}

#[test]
fn percent_to_int_0() {
    assert_eq!(to_int(percent(0)), 0)
}
#[test]
fn percent_to_int_50() {
    assert_eq!(to_int(percent(50)), 127)
}
#[test]
fn percent_to_int_100() {
    assert_eq!(to_int(percent(100)), 255)
}
#[test]
fn percent_to_int_overflow() {
    assert_eq!(to_int(percent(130)), 255)
}

#[cfg(test)]
fn percent(n: isize) -> Value {
    Value::Numeric(Rational::from_integer(n), Unit::Percent, false)
}


fn to_rational(v: Value) -> Rational {
    match v {
        Value::Numeric(v, _, _) => v,
        v => panic!("Expected rational, got {:?}", v),
    }
}

/// Gets a percentage as a fraction 0 .. 1.
/// If v is not a percentage, keep it as it is.
fn to_rational_percent(v: Value) -> Rational {
    match v {
        Value::Numeric(v, Unit::Percent, _) => v / Rational::from_integer(100),
        Value::Numeric(v, _, _) => v,
        v => panic!("Expected rational, got {:?}", v),
    }
}

#[test]
fn test_rgb() {
    use formalargs::call_args;
    use variablescope::ScopeImpl;
    assert_eq!(Value::Color(17, 0, 225, Rational::from_integer(1), None),
               FUNCTIONS.get("rgb")
                   .unwrap()
                   .call(&mut ScopeImpl::new(),
                         &call_args(b"(17, 0, 225)").unwrap().1))
}

#[test]
fn test_hsl_black() {
    assert_eq!("black", do_evaluate(&[], b"hsl(17, 32%, 0%);"))
}
#[test]
fn test_hsl_white() {
    assert_eq!("white", do_evaluate(&[], b"hsl(300, 82%, 100%);"))
}
#[test]
fn test_hsl_gray() {
    assert_eq!("grey", do_evaluate(&[], b"hsl(300, 0%, 50%);"))
}
#[test]
fn test_hsl_red() {
    assert_eq!("#f7c9c9", do_evaluate(&[], b"hsl(0, 75%, 88%);"))
}
#[test]
fn test_hsl_yellow() {
    assert_eq!("#ffff42", do_evaluate(&[], b"hsl(60, 100%, 63%);"))
}
#[test]
fn test_hsl_blue_magenta() {
    assert_eq!("#6118aa", do_evaluate(&[], b"hsl(270, 75%, 38%);"))
}

#[cfg(test)]
use super::variablescope::test::do_evaluate;
