# Parse Bencode data in Rust

https://en.wikipedia.org/wiki/Bencode

### Specification
```
<BE>    ::= <DICT> | <LIST> | <INT> | <STR>

<DICT>  ::= "d" 1 * (<STR> <BE>) "e"
<LIST>  ::= "l" 1 * <BE>         "e"
<INT>   ::= "i"     <SNUM>       "e"
<STR>   ::= <NUM> ":" n * <CHAR>; where n equals the <NUM>

<SNUM>  ::= "-" <NUM> / <NUM>
<NUM>   ::= 1 * <DIGIT>
<CHAR>  ::= %
<DIGIT> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

### Usage
```rust
fn main() {
    let res = bencode_parse(
        "li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ed3:\
         foo4:spam3:bari42e6:nestedd3:baz4:boom3:zooi42eeee"
    );

    println!("{:#?}", res);
}
```

### Output
```
Ok(
    BList(
        [
            BInteger(
                12,
            ),
            BString(
                "abcd",
            ),
            BList(
                [
                    BInteger(
                        -23,
                    ),
                    BInteger(
                        34,
                    ),
                ],
            ),
            BInteger(
                4200000024,
            ),
            BString(
                "qwerty",
            ),
            BInteger(
                -42,
            ),
            BDictionary(
                {
                    "bar": BInteger(
                        42,
                    ),
                    "foo": BString(
                        "spam",
                    ),
                    "nested": BDictionary(
                        {
                            "baz": BString(
                                "boom",
                            ),
                            "zoo": BInteger(
                                42,
                            ),
                        },
                    ),
                },
            ),
        ],
    ),
)
```