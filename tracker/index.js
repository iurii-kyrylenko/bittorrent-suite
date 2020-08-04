const bencode = require("bencode");
const fetch = require("node-fetch");
const encoder = require("./byte-encoder")();

const tracker = "http://bt3.t-ru.org/ann";
const infoHash = "C319F71C2736E42CAB5B4AFF832B566D894725DA";
const peerId = "-IK0001-234669001175";
const ip = "188.163.49.93";
const port = 6881;
const uploaded = 0;
const downloaded = 0;
const left = 8097363760;

const url =
  `${tracker}` +
  `?info_hash=${encoder.encode(Buffer.from(infoHash, "hex"))}` +
  `&peer_id=${encoder.encode(Buffer.from(peerId))}` +
  `&ip=${ip}` +
  `&port=${port}` +
  `&uploaded=${uploaded}` +
  `&downloaded=${downloaded}` +
  `&left=${left}`;

const getPeers = (bytes) => {
  const peers = bencode.decode(bytes).peers;
  const peerCount = Buffer.byteLength(peers) / 6;
  
  return Array.from({ length: peerCount }, (_, i) => {
    const [a, b, c, d, e, f] = peers.slice(i * 6, (i + 1) * 6);
    return `${a}.${b}.${c}.${d}:${e * 256 + f}`;
  });    
};

fetch(url)
  .then(res => res.buffer())
  .then(getPeers)
  .then(console.log)
  .catch(console.error);

// const fs = require("fs");
// const bytes = fs.readFileSync("./response");
// console.log(getPeers(bytes));
