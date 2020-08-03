Sent handshake:
```
infoHash:   ad453b12b551ceea954c4d828e40331a810a55fc
peerId:     2d4c54313042302d4a4a2d593479414638297064
```

Received:
```
===== [PWP] handshake =====
infoHash:   ad453b12b551ceea954c4d828e40331a810a55fc
peerId:     2d4c54313042302d4a4a2d593479414638297064
extensions: { dht: true, extended: true }

===== [PWP] bitfield =====
165: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0

===== [TCP] raw data =====
470: 13426974546f7272656e742070726f746f636f6c0000000000100005ad453b12b551ceea954c4d828e40331a810a55fc2d4c54313042302d4a4a2d593479414638297064000000e414006431323a636f6d706c6574655f61676f692d3165313a6d6431313a4c545f6d657461646174616931346531313a6c745f646f6e746861766569376531303a73686172655f6d6f646569386531313a75706c6f61645f6f6e6c7969336531323a75745f686f6c6570756e636869346531313a75745f6d65746164617461693265363a75745f7065786931656531333a6d657461646174615f73697a6569323633363165343a72657171693530306531313a75706c6f61645f6f6e6c79693165313a7631353a466f6c7820352e31382e3133393433363a796f75726970343a7f00000165000000a605ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0
```

```
=== handshake ===
00000000  13                                                           len = 19
00000001  42 69 74 54 6f 72 72 65 6e 74 20 70 72 6f 74 6f 63 6f 6c     "BitTorrent protocol"
00000014  00 00 00 00 00 10 00 05                                      reserved: extended
0000001c  AD 45 3B 12 B5 51 CE EA 95 4C 4D 82 8E 40 33 1A 81 0A 55 FC  info_hash
00000030  2D 4C 54 31 30 42 30 2D 4A 4A 2D 59 34 79 41 46 38 29 70 64  peer_id: -LT10B0...


=== extended ===
00000044  00 00 00 E4                                                  len = 228
00000048  14                                                           20 (extended message)
00000049  00                                                           ext_msg_id = 0 (handshake)

0000004a                                64 31 32 3A 63 6F                        d12:co
00000050: 6D 70 6C 65 74 65 5F 61 67 6F 69 2D 31 65 31 3A              mplete_agoi-1e1:
00000060: 6D 64 31 31 3A 4C 54 5F 6D 65 74 61 64 61 74 61              md11:LT_metadata
00000070: 69 31 34 65 31 31 3A 6C 74 5F 64 6F 6E 74 68 61              i14e11:lt_dontha
00000080: 76 65 69 37 65 31 30 3A 73 68 61 72 65 5F 6D 6F              vei7e10:share_mo
00000090: 64 65 69 38 65 31 31 3A 75 70 6C 6F 61 64 5F 6F              dei8e11:upload_o
000000a0: 6E 6C 79 69 33 65 31 32 3A 75 74 5F 68 6F 6C 65              nlyi3e12:ut_hole
000000b0: 70 75 6E 63 68 69 34 65 31 31 3A 75 74 5F 6D 65              punchi4e11:ut_me
000000c0: 74 61 64 61 74 61 69 32 65 36 3A 75 74 5F 70 65              tadatai2e6:ut_pe
000000d0: 78 69 31 65 65 31 33 3A 6D 65 74 61 64 61 74 61              xi1ee13:metadata
000000e0: 5F 73 69 7A 65 69 32 36 33 36 31 65 34 3A 72 65              _sizei26361e4:re
000000f0: 71 71 69 35 30 30 65 31 31 3A 75 70 6C 6F 61 64              qqi500e11:upload
00000100: 5F 6F 6E 6C 79 69 31 65 31 3A 76 31 35 3A 46 6F              _onlyi1e1:v15:Fo
00000110: 6C 78 20 35 2E 31 38 2E 31 33 39 34 33 36 3A 79              lx.5.18.139436:y
00000120: 6F 75 72 69 70 34 3A 7F 00 00 01 65                          ourip4:....e


d12:complete_agoi-1e1:md11:LT_metadatai14e11:lt_donthavei7e10:share_modei8e11:upload_onlyi3e12:ut_holepunchi4e11:ut_metadatai2e6:ut_pexi1ee13:metadata_sizei26361e4:reqqi500e11:upload_onlyi1e1:v15:Folx.5.18.139436:yourip4:____e

{
  "complete_ago": -1,
  "m": {
    "LT_metadata": 14,
    "lt_donthave": 7,
    "share_mode": 8,
    "upload_only": 3,
    "ut_holepunch": 4,
    "ut_metadata": 2,
    "ut_pex": 1
  },
  "metadata_size": 26361,
  "reqq": 500,
  "upload_only": 1,
  "v": "Folx 5.18.13943",
  "yourip": "\x7f\x00\x00\x01" // 128.0.0.1
}

=== bitfield ===
0000012C:                                     00 00 00 A6    len = 166
00000130: 05                                                 bitfield
00000131:    FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000140: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000150: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000160: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000170: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000180: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
00000190: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
000001a0: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
000001b0: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
000001c0: FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF    ................
000001d0: FF FF FF FF FF C0

number_of_downloaded_pieces = 164 * 8 + 2 = 1314
file_len = 43032429
piece_len = 32768
file_len = 1313 * piece_len + 8045
```
