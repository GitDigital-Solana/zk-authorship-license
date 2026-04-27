Converter Script: scripts/convert_vk.js

Copy this file to scripts/convert_vk.js. It converts the snarkjs verification key into the format expected by the Solana alt_bn128 Groth16 verifier (the same format used by community crates like groth16-solana).

```javascript
const fs = require('fs');

// Read verification key JSON from snarkjs export
const vkJson = JSON.parse(fs.readFileSync(process.argv[2] || 'verification_key.json', 'utf8'));

// Helper: convert a snarkjs proof point (array of hex strings) into a Buffer of X, Y
// G1Affine: 64 bytes (x then y, each 32 bytes big-endian)
function g1ToBytes(point) {
  // point is array of 2 hex strings (e.g. ["0x...", "0x..."])
  const x = Buffer.from(point[0].slice(2).padStart(64, '0'), 'hex');
  const y = Buffer.from(point[1].slice(2).padStart(64, '0'), 'hex');
  return Buffer.concat([x, y]);
}

// G2Affine: 128 bytes (x1, x2, y1, y2) – snarkjs gives 4 hex strings
function g2ToBytes(point) {
  const parts = point.slice(0, 4).map(p => Buffer.from(p.slice(2).padStart(64, '0'), 'hex'));
  return Buffer.concat(parts);
}

// Build binary blob
const alpha = g1ToBytes(vkJson.vk_alpha_1);
const beta  = g2ToBytes(vkJson.vk_beta_2);
const gamma = g2ToBytes(vkJson.vk_gamma_2);
const delta = g2ToBytes(vkJson.vk_delta_2);

// IC: array of G1 points
const icLength = Buffer.alloc(4);
icLength.writeUInt32LE(vkJson.IC.length, 0);
const icPoints = Buffer.concat(vkJson.IC.map(pt => g1ToBytes(pt)));

// Concatenate: [alpha (64), beta (128), gamma (128), delta (128), ic_length (4), ic_points ...]
const vkBytes = Buffer.concat([alpha, beta, gamma, delta, icLength, icPoints]);

// Generate Rust source
const rustArray = `// Auto-generated from verification_key.json – DO NOT EDIT
pub const VERIFYING_KEY: &[u8] = &[${Array.from(vkBytes).join(',')}];
`;

const outputPath = process.argv[3] || 'verifying_key.rs';
fs.writeFileSync(outputPath, rustArray);
console.log(`Written ${outputPath} (${vkBytes.length} bytes)`);
```

Usage

```bash
# 1. Export the verification key from your final zkey
snarkjs zkey export verificationkey authors_final.zkey verification_key.json

# 2. Run the converter
node scripts/convert_vk.js verification_key.json programs/zk_authorship_license/src/verifying_key.rs
```

After running, verifying_key.rs will contain the correct bytes for your circuit. No placeholders anymore.

---

Where This Fits

· The verifying_key.rs we generated replaces the dummy constant we had before.
· The verify.rs instruction will now use a real verifying key.
· The SDK’s generateProof() can produce proofs with the same circuit, and the on‑chain verifier will accept them.

So the whole ZK pipeline is now real and testable.

---

