use bencode::bencode_parse;

fn main() {
    let res = bencode_parse("li12e4li-23ei34eei4200000024ee");
    println!("{:?}", res);
}
