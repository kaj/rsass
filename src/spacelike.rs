use nom::multispace;

named!(pub spacelike<&[u8], Vec<&[u8]> >,
       many1!(alt!(multispace |
                   chain!(tag!("//") ~ c: is_not!("\n"), || c))));
