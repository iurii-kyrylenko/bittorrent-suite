const pwp = require("peer-wire-protocol");
const net = require("net");

const socket = new net.Socket();

const wire = pwp();

socket.connect(14082, "127.0.0.1", function() {
  console.log("===== [TCP] connected =====");
  console.log();

  socket
  .pipe(wire)
  .pipe(socket);

  wire.handshake(
    Buffer.from("AD453B12B551CEEA954C4D828E40331A810A55FC", "hex"),
    Buffer.from("12345678901234567890")
  );
});

// socket.on("data", function(data) {
//   console.log("===== [TCP] raw data =====");
//   console.log(`${Buffer.byteLength(data)}: ${data.toString("hex")}`);
//   console.log();
// });

socket.on("close", function() {
  console.log("===== [TCP] connection closed =====");
  console.log();
});
  
wire.on("keep-alive", function() {
	console.log("===== [PWP] keep-alive =====");
  console.log();
});

wire.on("handshake", function(infoHash, peerId, extensions) {
  console.log("===== [PWP] handshake =====");
  console.log("infoHash:  ", infoHash.toString("hex"));
  console.log("peerId:    ", peerId.toString("hex"));
  console.log("extensions:", extensions);
  console.log();
});

wire.on("bitfield", function(bitfield) {
  console.log("===== [PWP] bitfield =====");
  console.log(`${Buffer.byteLength(bitfield)}: ${bitfield.toString("hex")}`);
  console.log();

  wire.interested();
});

wire.on("choke", function() {
  console.log("===== [PWP] choke =====");
  console.log();
});

wire.on("unchoke", function() {
	console.log("===== [PWP] unchoke =====");
  console.log();

  wire.request(0, 0, 256, function(err, block) {
    if (err) {
      console.log("===== [PWP] request error =====", err);
      console.log();
      return;
    }

    console.log("===== [PWP] piece =====");
    console.log(`${Buffer.byteLength(block)}: ${block.toString("hex")}`);
    console.log();

    wire.uninterested();
  });
});

wire.on("download", function(numberOfBytes) {
  console.log("===== [PWP] download =====", numberOfBytes);
  console.log();

  wire.interested();
});

// wire.on("have", function(pieceIndex) {
//   console.log("===== [PWP] have =====", pieceIndex);
// });
// wire.on("interested", function() {
// 	console.log("===== [PWP] interested =====");
// });
// wire.on("uninterested", function() {
// 	console.log("===== [PWP] uninterested =====");
// });
// wire.on("request", function(pieceIndex, offset, length, callback) {
// 	console.log("===== [PWP] request =====", pieceIndex, offset, length);
// });
// wire.on("upload", function(numberOfBytes) {
// 	console.log("===== [PWP] upload =====", numberOfBytes);
// });
// wire.on("port", function(dhtPort) {
// 	console.log("===== [PWP] port =====", dhtPort);
// });

// setTimeout(function() {
// 	socket.destroy();
// }, 2 * 1000);
