use nom::alpha;
use parser::util::is_name_char;
use parser::value::value_expression;
use sass::{SassString, StringPart};
use std::str::from_utf8;
use value::Quotes;

named!(pub sass_string<SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               map!(selector_string, StringPart::Raw))),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_ext<SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               extended_part)),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_dq<SassString>,
       map!(delimited!(tag!("\""),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(from_utf8(s)
                                                        .unwrap()
                                                        .to_string())) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\\\"")) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("'")) |
                           value!(StringPart::Raw("\\#".to_string()),
                                  tag!("\\#")) |
                           value!(StringPart::Raw("\\".to_string()),
                                  tag!("\\\\")) |
                           map!(extra_escape, StringPart::Raw))),
                       tag!("\"")),
            |p| SassString::new(p, Quotes::Double)));

named!(pub sass_string_sq<SassString>,
       map!(delimited!(tag!("'"),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(from_utf8(s)
                                                        .unwrap()
                                                        .to_string())) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("\\'")) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\"")) |
                           value!(StringPart::Raw("\\#".to_string()),
                                  tag!("\\#")) |
                           value!(StringPart::Raw("\\\\".to_string()),
                                  tag!("\\\\")) |
                           map!(extra_escape, StringPart::Raw)
                               )),
                       tag!("'")),
            |p| SassString::new(p, Quotes::Single)));

named!(string_part_interpolation<StringPart>,
       map!(delimited!(tag!("#{"), value_expression, tag!("}")),
            StringPart::Interpolation));

named!(simple_qstring_part<StringPart>,
       map!(is_not!("\\#'\""),
            |s| StringPart::Raw(from_utf8(s).unwrap().to_string())));

named!(selector_string<String>,
       fold_many1!(alt_complete!(selector_plain_part |
                                 selector_escaped_part |
                                 hash_no_interpolation),
                   String::new(),
                   |mut acc: String, item: &[u8]| {
                       acc.push_str(from_utf8(item).unwrap());
                       acc
                   }));
named!(selector_plain_part<&[u8]>,
       is_not!("\n\t >$\"'\\#+*/()[]{}:;,=!&@"));
named!(selector_escaped_part<&[u8]>,
       recognize!(preceded!(tag!("\\"),
                            alt!(value!((), many_m_n!(1, 3, hexpair)) |
                                 value!((), take!(1))))));
named!(hexpair,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));
named!(hash_no_interpolation<&[u8]>,
       terminated!(tag!("#"), peek!(not!(tag!("{")))));
named!(extra_escape<String>,
       map!(preceded!(tag!("\\"), alt!(alpha | tag!(" "))),
            |s| from_utf8(s).unwrap().to_string()));

named!(pub extended_part<StringPart>,
       map!(recognize!(pair!(take_while1!(is_ext_str_start_char),
                             take_while!(is_ext_str_char))),
            |v| StringPart::Raw(from_utf8(v).unwrap().into())));

fn is_ext_str_start_char(c: u8) -> bool {
    is_name_char(c) || c == b'*' || c == b'+' || c == b'.' ||
    c == b'/' || c == b':' || c == b'=' || c == b'?' || c == b'|'
}
fn is_ext_str_char(c: u8) -> bool {
    is_name_char(c) || c == b'*' || c == b'+' || c == b',' ||
    c == b'.' || c == b'/' || c == b':' || c == b'=' ||
    c == b'?' || c == b'|'
}

#[test]
fn t_extra_escape() {
    use nom::IResult::*;

    assert_eq!(extra_escape(b"\\n"),
               Done(&b""[..], "n".to_string()))
}
