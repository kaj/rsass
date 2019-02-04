use super::util::{is_name_char, take_char};
use super::value::value_expression;
use super::{input_to_str, input_to_string};
use crate::sass::{SassString, StringPart};
use crate::value::Quotes;
use nom::alphanumeric;
use nom::types::CompleteByteSlice as Input;
use nom::*;

named!(pub sass_string<Input, SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               map!(selector_string, StringPart::Raw))),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_ext<Input, SassString>,
       map!(
           many1!(alt_complete!(
               string_part_interpolation |
               extended_part)),
           |p| SassString::new(p, Quotes::None)));

named!(pub sass_string_dq<Input, SassString>,
       map!(delimited!(tag!("\""),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(s.to_string())) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\\\"")) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("'")) |
                           extra_escape)),
                       tag!("\"")),
            |p| SassString::new(p, Quotes::Double)));

named!(pub sass_string_sq<Input, SassString>,
       map!(delimited!(tag!("'"),
                       many0!(alt_complete!(
                           simple_qstring_part |
                           string_part_interpolation |
                           map!(hash_no_interpolation,
                                |s| StringPart::Raw(s.to_string())) |
                           value!(StringPart::Raw("'".to_string()),
                                  tag!("\\'")) |
                           value!(StringPart::Raw("\"".to_string()),
                                  tag!("\"")) |
                           extra_escape
                               )),
                       tag!("'")),
            |p| SassString::new(p, Quotes::Single)));

named!(
    string_part_interpolation<Input, StringPart>,
    map!(
        delimited!(tag!("#{"), value_expression, tag!("}")),
        StringPart::Interpolation
    )
);

named!(
    simple_qstring_part<Input, StringPart>,
    map!(map_res!(is_not!("\\#'\""), input_to_string), StringPart::Raw)
);

named!(
    selector_string<Input, String>,
    fold_many1!(
        alt_complete!(
            selector_plain_part
                | selector_escaped_part
                | hash_no_interpolation
        ),
        String::new(),
        |mut acc: String, item: &str| {
            acc.push_str(item);
            acc
        }
    )
);
named!(
    selector_plain_part<Input, &str>,
    map_res!(is_not!("\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_str)
);

named!(
    selector_escaped_part<Input, &str>,
    map_res!(
        recognize!(preceded!(
            tag!("\\"),
            alt!(value!((), many_m_n!(1, 3, hexpair)) | value!((), take!(1)))
        )),
        input_to_str
    )
);
named!(
    hexpair<Input, Input>,
    recognize!(do_parse!(
        one_of!("0123456789ABCDEFabcdef")
            >> one_of!("0123456789ABCDEFabcdef")
            >> ()
    ))
);
named!(
    hash_no_interpolation<Input, &str>,
    map_res!(terminated!(tag!("#"), peek!(not!(tag!("{")))), input_to_str)
);
named!(
    extra_escape<Input, StringPart>,
    map!(
        map_res!(
            preceded!(
                tag!("\\"),
                alt!(
                    alphanumeric
                        | tag!(" ")
                        | tag!("'")
                        | tag!("\"")
                        | tag!("\\")
                        | tag!("#")
                )
            ),
            input_to_string
        ),
        |s| StringPart::Raw(format!("\\{}", s))
    )
);

named!(
    pub extended_part<Input, StringPart>,
    map!(
        map_res!(
            recognize!(pair!(
                verify!(take_char, is_ext_str_start_char),
                many0!(verify!(take_char, is_ext_str_char))
            )),
            input_to_string
        ),
        StringPart::Raw
    )
);

fn is_ext_str_start_char(c: char) -> bool {
    is_name_char(c)
        || c == '*'
        || c == '+'
        || c == '.'
        || c == '/'
        || c == ':'
        || c == '='
        || c == '?'
        || c == '|'
}
fn is_ext_str_char(c: char) -> bool {
    is_name_char(c)
        || c == '*'
        || c == '+'
        || c == ','
        || c == '.'
        || c == '/'
        || c == ':'
        || c == '='
        || c == '?'
        || c == '|'
}
