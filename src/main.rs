use bencode::*;

fn main() {
    let res = parse_int(b"i1234567890e_abc");
    println!("{:?}", res);

    let res = parse_int(b"i-42e_abc");
    println!("{:?}", res);

    let res = parse_str(b"10:abcdefghij_abc");
    println!("{:?}", res);
}
