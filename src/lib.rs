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
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::length_data,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn parse_int(input: &[u8]) -> IResult<&[u8], i64> {
    let digits = delimited(
        char('i'),
        recognize(tuple((opt(char('-')), is_not("e")))),
        char('e'),
    );

    map_res(digits, to_int)(input)
}

pub fn parse_str(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let digits = terminated(digit1, char(':'));

    length_data(map_res(digits, to_int::<usize>))(input)
}

fn to_int<T: FromStr>(bytes: &[u8]) -> Result<T, <T as FromStr>::Err> {
    let s = String::from_utf8_lossy(bytes);
    FromStr::from_str(&s)
}
