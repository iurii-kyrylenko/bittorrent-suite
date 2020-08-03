# Parse Bencode data in Rust with the `nom` crate

### Applications
- Bittorent clients
- Torrent files

https://en.wikipedia.org/wiki/Bencode

According to the [BEP-3](http://bittorrent.org/beps/bep_0003.html) specification, "_All strings in a .torrent file that contains text must be UTF-8 encoded._" E.g. that includes the `name` keys in the `info` dictionary and the strings in the `path` list. On the other hand the data in the `pieces` (sequence of SHA1 hashes) is a string which is not UTF-8 encoded.

The current implementation handles BStrings as byte arrays, exept of the dictionary keys. We assume that the keys are properly UTF-8 encoded. If it's necessary, the representation of other data to UTF-8 string should be performed outside of Bencode transformation.

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
    let res = parse_be(
        b"li12e4:abcdli-23ei34eei4200000024e6:qwertyi-42ed3:\
        foo4:spam3:bari42e6:nestedd3:baz4:boom3:zooi42eeee",
    );

    println!("{:#?}", res);
}
```

### Output

```
Ok(
    (
        [], // This shows that parser hadled all input data
        BLst(
            [
                BInt(
                    12,
                ),
                BStr(
                    [
                        97,
                        98,
                        99,
                        100,
                    ],
                ),
                BLst(
                    [
                        BInt(
                            -23,
                        ),
                        BInt(
                            34,
                        ),
                    ],
                ),
                BInt(
                    4200000024,
                ),
                BStr(
                    [
                        113,
                        119,
                        101,
                        114,
                        116,
                        121,
                    ],
                ),
                BInt(
                    -42,
                ),
                BDict(
                    {
                        "bar": BInt(
                            42,
                        ),
                        "foo": BStr(
                            [
                                115,
                                112,
                                97,
                                109,
                            ],
                        ),
                        "nested": BDict(
                            {
                                "baz": BStr(
                                    [
                                        98,
                                        111,
                                        111,
                                        109,
                                    ],
                                ),
                                "zoo": BInt(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
        ),
    ),
)
```