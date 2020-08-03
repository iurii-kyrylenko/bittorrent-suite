### Tools:

- https://www.urlencoder.org/
- https://chocobo1.github.io/bencode_online/
- https://www.fileformat.info/tool/hash.htm

### Calculate info_hash

- option-1: from magnet link:
   `magnet:?xt=urn:btih:C319F71C2736E42CAB5B4AFF832B566D894725DA&tr=http%3A%2F%2Fbt3.t-ru.org%2Fann%3Fmagnet&dn=%D0%A1%D0%B5%D0%B2%D0%B5%D1%80%D1%8F%D0%BD%D0%B5%20%2F%20De%20noorderlingen%20%2F%20The%20Northerners%20(%D0%90%D0%BB%D0%B5%D0%BA%D1%81%20%D0%B2%D0%B0%D0%BD%20%D0%92%D0%B0%D1%80%D0%BC%D0%B5%D1%80%D0%B4%D0%B0%D0%BC%20%2F%20Alex%20van%20Warmerdam)%20%5B1992%2C%20%D0%9D%D0%B8%D0%B4%D0%B5%D1%80%D0%BB%D0%B0%D0%BD%D0%B4%D1%8B%2C%20%D0%B4%D1%80%D0%B0%D0%BC%D0%B0%2C%20%D0%BA%D0%BE%D0%BC%D0%B5%D0%B4%D0%B8%D1%8F%2C%20WEB-DL%201080p%5D%20%5BAMZN%5D%20MVO%20(`

- option-2: get info dictionary from a torrent file and calculate file's SHA-1.

info-hash = `C319F71C2736E42CAB5B4AFF832B566D894725DA`

urlencode:
```
   C3 %C3
   19 %19
   F7 %F7
   1C %1C
   27 %27
   36 6
   E4 %E4
   2C %2C
   AB %AB
   5B %5B
   4A J
   FF %FF
   83 %83
   2B %2B
   56 V
   6D m
   89 %89
   47 G
   25 %25
   DA %DA
```

### Request parameters
```
info-hash = `%C3%19%F7%1C%276%E4%2C%AB%5BJ%FF%83%2BVm%89G%25%DA`

peer_id = `-IK0001-234669001175`

ip = `188.163.49.93`

port = `6881`

uploaded = `0`

downloaded = `0`

left = `8097363760`
```

### Request:
```
curl -X GET 'http://bt3.t-ru.org/ann?info_hash=%C3%19%F7%1C%276%E4%2C%AB%5BJ%FF%83%2BVm%89G%25%DA&peer_id=-IK0001-234669001175&ip=188.163.49.93&port=6881&uploaded=0&downloaded=0&left=8097363760' > res
```

### Response:
```
d8:intervali3323e12:min intervali3323e5:peers48:................................................e
   "interval": 3323,
   "min interval": 3323,
   "peers":
      0x4D, 0x58, 0xFD, 0x36, 0xC214 => 77.88.253.54:49684
      0xD9, 0x19, 0xE7, 0x5B, 0x7530 => 217.25.231.91:30000
      0x91, 0x82, 0x5C, 0x04, 0x4D9F => 145.130.92.4:19871
      0x57, 0xFF, 0xC5, 0x44, 0xC8D5 => 87.255.197.68:51413
      0x4D, 0x4B, 0x92, 0x82, 0x38F7 => 77.75.146.130:14583
      0x33, 0x4B, 0x4B, 0xF5, 0xC41A => 51.75.75.245:50202
      0x5F, 0x38, 0xD2, 0xBC, 0xC8D5 => 95.56.210.188:51413
      0x92, 0x9E, 0x3B, 0xA6, 0x33DF => 146.158.59.166:13279
```
