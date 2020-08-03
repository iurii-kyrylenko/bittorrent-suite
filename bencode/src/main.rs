use bencode::*;
use std::{env, fs};

fn main() {
    test_inline();

    // test_from_file();
}

fn test_inline() {
    let res = parse_be(b"i1234567890e_abc");
    println!("== 1 == {:?}", res);

    let res = parse_be(b"i-42e_abc");
    println!("== 2 == {:?}", res);

    let res = parse_be(b"10:abcdefghij_abc");
    println!("== 3 == {:?}", res);

    let res = parse_be(b"li-42e10:abcdefghiji1234567890ee_abc");
    println!("== 4 == {:?}", res);

    let res = parse_be(b"li-42eli42ei-777ee10:abcdefghiji1234567890ee_abc");
    println!("== 5 == {:#?}", res);

    let res = parse_be(b"d3:fooi42ee");
    println!("== 6 == {:#?}", res);

    let res = parse_be(
        b"li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ed3:\
        foo4:spam3:bari42e6:nestedd3:baz4:boom3:zooi42eeee",
    );
    println!("== 7 == {:#?}", res);
}

fn test_from_file() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let bytes = fs::read(filename).expect("Cannot read the file");

    let res = parse_be(&bytes);

    println!("{:?}", res);
}
