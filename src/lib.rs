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
use std::collections::HashMap;
use std::str::Chars;

pub struct BInteger(i64);

pub struct BString(String);

pub struct BList(Vec<BEncode>);

pub struct BDictionary(HashMap<String, BEncode>);

#[derive(Debug)]
pub enum BEncode {
    BInteger(i64),
    BString(String),
    BList(Vec<BEncode>),
    BDictionary(HashMap<String, BEncode>),
    BEnd,
}

#[derive(Debug)]
pub struct BError;

impl BInteger {
    fn parse(chars: &mut Chars) -> Result<i64, BError> {
        let mut digits = String::new();

        for c in chars {
            match c {
                'e' => return digits.parse().or_else(|_| Err(BError)),
                _ => digits.push(c),
            };
        }

        Err(BError)
    }
}

impl BString {
    fn parse(c: char, chars: &mut Chars) -> Result<String, BError> {
        let len = BString::get_length(c, chars)?;

        let s: String = chars.take(len).collect();

        if s.len() == len {
            Ok(s)
        } else {
            Err(BError)
        }
    }

    fn get_length(c: char, chars: &mut Chars) -> Result<usize, BError> {
        let mut digits = c.to_string();

        for c in chars {
            match c {
                ':' => return digits.parse().or_else(|_| Err(BError)),
                _ => digits.push(c),
            };
        }

        Err(BError)
    }
}

impl BList {
    fn parse(chars: &mut Chars) -> Result<Vec<BEncode>, BError> {
        let mut vec: Vec<BEncode> = Vec::new();

        loop {
            match BEncode::parse(chars)? {
                BEncode::BEnd => return Ok(vec),
                be => vec.push(be),
            };
        }
    }
}

impl BEncode {
    fn parse(chars: &mut Chars) -> Result<Self, BError> {
        match chars.next() {
            Some(c) if c >= '1' && c <= '9' => Ok(BEncode::BString(BString::parse(c, chars)?)),
            Some('i') => Ok(BEncode::BInteger(BInteger::parse(chars)?)),
            Some('l') => Ok(BEncode::BList(BList::parse(chars)?)),
            Some('e') => Ok(BEncode::BEnd),
            _ => Err(BError),
        }
    }
}

pub fn bencode_parse(encoded: &str) -> Result<BEncode, BError> {
    BEncode::parse(&mut encoded.chars())
}
