use bencode::bencode_parse;

fn main() {
    let res = bencode_parse("li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ee");

    println!("{:#?}", res);
}
