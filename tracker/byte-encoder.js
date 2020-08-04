function ByteEncoder() {
  const range = (start, stop) => Array.from(
    { length: stop - start + 1 },
    (_, i) => start + i
  );
  
  const set = new Set([
    ...range(0x30, 0x39), // 0..9
    ...range(0x41, 0x5A), // A..Z
    ...range(0x61, 0x7A)  // a..z
  ]);

  return {
    encode: bytes =>
      [...bytes].map(i =>
        set.has(i)
          ? String.fromCharCode(i)
          : "%" + i.toString(16).toUpperCase()
      ).join("")
  }
}

module.exports = ByteEncoder;
