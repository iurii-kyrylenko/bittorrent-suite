use bencode::bencode_parse;
use std::{env, fs};

fn main() {
    test_inline();

    // test_from_file();
}

fn test_inline() {
    let res = bencode_parse(
        "li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ed3:\
         foo4:spam3:bari42e6:nestedd3:baz4:boom3:zooi42eeee",
    );

    println!("{:#?}", res);
}

fn test_from_file() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let bytes = fs::read(filename).expect("Cannot read the file");
    let encoded = String::from_utf8_lossy(&bytes);

    let res = bencode_parse(&encoded);

    println!("{:#?}", res);
}
