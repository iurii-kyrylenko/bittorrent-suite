/// <BE>    ::= <DICT> | <LIST> | <INT> | <STR>
///
/// <DICT>  ::= "d" 1 * (<STR> <BE>) "e"
/// <LIST>  ::= "l" 1 * <BE>         "e"
/// <INT>   ::= "i"     <SNUM>       "e"
/// <STR>   ::= <NUM> ":" n * <CHAR>; where n equals the <NUM>
///
/// <SNUM>  ::= "-" <NUM> / <NUM>
/// <NUM>   ::= 1 * <DIGIT>
/// <CHAR>  ::= %
/// <DIGIT> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
use std::str::FromStr;

use nom::{
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map_res, opt, recognize},
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_int(input: &[u8]) -> IResult<&[u8], i64> {
    let parse_bytes = delimited(
        char('i'),
        recognize(tuple((opt(char('-')), is_not("e")))),
        char('e'),
    );

    let to_int = |bytes| {
        let s = String::from_utf8_lossy(bytes);
        FromStr::from_str(&s)
    };

    map_res(parse_bytes, to_int)(input)
}
