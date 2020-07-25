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
use std::{collections::BTreeMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::{length_data, many1},
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug)]
pub enum BE<'a> {
    BInt(i64),
    BStr(&'a [u8]),
    BLst(Vec<BE<'a>>),
    // BDict(BTreeMap<&'a [u8], BE<'a>>)
    BDict(BTreeMap<String, BE<'a>>),
}

pub fn parse_be(input: &[u8]) -> IResult<&[u8], BE> {
    alt((parse_int, parse_str, parse_lst, parse_dict))(input)
}

fn parse_dict(input: &[u8]) -> IResult<&[u8], BE> {
    let vec = delimited(char('d'), many1(pair(parse_key, parse_be)), char('e'));

    map_res(vec, |xs| {
        let dictionary = xs
            .into_iter()
            // Comment out next line for the variant above:
            // `BDict(BTreeMap<&'a [u8], BE<'a>>)`
            .map(|(key, value)| (String::from_utf8_lossy(key).to_string(), value))
            .collect();

        <Result<BE, ()>>::Ok(BE::BDict(dictionary))
    })(input)
}

fn parse_lst(input: &[u8]) -> IResult<&[u8], BE> {
    let vec = delimited(char('l'), many1(parse_be), char('e'));

    map_res(vec, |xs| <Result<BE, ()>>::Ok(BE::BLst(xs)))(input)
}

fn parse_int(input: &[u8]) -> IResult<&[u8], BE> {
    let digits = delimited(
        char('i'),
        recognize(pair(opt(char('-')), is_not("e"))),
        char('e'),
    );

    map_res(map_res(digits, to_int), |int| {
        <Result<BE, ()>>::Ok(BE::BInt(int))
    })(input)
}

fn parse_str(input: &[u8]) -> IResult<&[u8], BE> {
    let digits = terminated(digit1, char(':'));

    map_res(length_data(map_res(digits, to_int::<usize>)), |bytes| {
        <Result<BE, ()>>::Ok(BE::BStr(bytes))
    })(input)
}

fn parse_key(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let digits = terminated(digit1, char(':'));

    length_data(map_res(digits, to_int::<usize>))(input)
}

fn to_int<T: FromStr>(bytes: &[u8]) -> Result<T, <T as FromStr>::Err> {
    let s = String::from_utf8_lossy(bytes);
    FromStr::from_str(&s)
}
