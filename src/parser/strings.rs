use nom::alphanumeric;
use nom::types::CompleteByteSlice as Input;
use parser::util::is_name_char;
use parser::value::value_expression;
use sass::{SassString, StringPart};
use std::str::from_utf8;
use value::Quotes;

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

pub fn input_to_str<'a>(s: Input<'a>) -> Result<&str, Utf8Error> {
    from_utf8(&s)
}

use std::str::Utf8Error;
fn input_to_string(s: Input) -> Result<String, Utf8Error> {
    from_utf8(&s).map(|s| s.to_string())
}

named!(
    selector_string<Input, String>,
    fold_many1!(
        alt_complete!(
            selector_plain_part
                | selector_escaped_part
                | hash_no_interpolation
        ),
        String::new(),
        |mut acc: String, item: String| {
            acc.push_str(&item);
            acc
        }
    )
);
named!(
    selector_plain_part<Input, String>,
    map_res!(is_not!("\n\t >$\"'\\#+*/()[]{}:;,=!&@"), input_to_string)
);

named!(
    selector_escaped_part<Input, String>,
    map_res!(
        recognize!(preceded!(
            tag!("\\"),
            alt!(value!((), many_m_n!(1, 3, hexpair)) | value!((), take!(1)))
        )),
        input_to_string
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
    hash_no_interpolation<Input, String>,
    map_res!(terminated!(tag!("#"), peek!(not!(tag!("{")))), input_to_string)
);
named!(
    extra_escape<Input, StringPart>,
    map!(
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
        |s| StringPart::Raw(format!("\\{}", input_to_string(s).unwrap()))
    )
);

named!(pub extended_part<Input, StringPart>,
       map!(recognize!(pair!(take_while1!(is_ext_str_start_char),
                             take_while!(is_ext_str_char))),
            |v| StringPart::Raw(from_utf8(&v).unwrap().into())));

fn is_ext_str_start_char(c: u8) -> bool {
    is_name_char(c)
        || c == b'*'
        || c == b'+'
        || c == b'.'
        || c == b'/'
        || c == b':'
        || c == b'='
        || c == b'?'
        || c == b'|'
}
fn is_ext_str_char(c: u8) -> bool {
    is_name_char(c)
        || c == b'*'
        || c == b'+'
        || c == b','
        || c == b'.'
        || c == b'/'
        || c == b':'
        || c == b'='
        || c == b'?'
        || c == b'|'
}
