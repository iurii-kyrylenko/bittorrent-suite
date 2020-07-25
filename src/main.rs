use bencode::*;

fn main() {
    let res1 = parse_int(b"i1234567890e_abc");
    let res2 = parse_int(b"i-42e_abc");
    println!("{:?}", res1);
    println!("{:?}", res2);
}
