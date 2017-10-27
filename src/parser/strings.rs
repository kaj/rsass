use nom::is_alphanumeric;
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

named!(string_part_interpolation<StringPart>,
       map!(delimited!(tag!("#{"), value_expression, tag!("}")),
            StringPart::Interpolation));

named!(pub sass_string_dq<SassString>,
       map!(delimited!(tag!("\""),
                       many1!(alt_complete!(
                           string_part_interpolation |
                           map!(string_part_dq, StringPart::Raw))),
                       tag!("\"")),
            |p| SassString::new(p, Quotes::Double)));

named!(string_part_dq<String>,
       map!(escaped!(is_not!("\\\""), '\\', one_of!("\"\\")),
            |s| from_utf8(s).unwrap().to_string()));

named!(pub sass_string_sq<SassString>,
       map!(delimited!(tag!("'"),
                       many1!(alt_complete!(
                           string_part_interpolation |
                           map!(string_part_sq, StringPart::Raw))),
                       tag!("'")),
            |p| SassString::new(p, Quotes::Single)));

named!(string_part_sq<String>,
       map!(escaped!(is_not!("\\'"), '\\', one_of!("'\\")),
            |s| from_utf8(s).unwrap().to_string()));

named!(selector_string<String>,
       fold_many1!(alt_complete!(selector_plain_part |
                                 selector_escaped_part |
                                 selector_id_part),
                   String::new(),
                   |mut acc: String, item: &[u8]| {
                       acc.push_str(from_utf8(item).unwrap());
                       acc
                   }));
named!(selector_plain_part<&[u8]>,
       take_while1!(is_selector_char));
named!(selector_escaped_part<&[u8]>,
       recognize!(preceded!(tag!("\\"), many_m_n!(1, 3, hexpair))));
named!(hexpair,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));
named!(selector_id_part<&[u8]>,
       terminated!(tag!("#"), peek!(not!(tag!("{")))));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'-' || chr == b'.'
}
