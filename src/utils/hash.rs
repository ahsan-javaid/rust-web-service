use std::time::{SystemTime, UNIX_EPOCH};

// Rotate bits left
fn rotate_left(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

// SHA-256 constants
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Initial hash values
const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

fn sha256(input: &[u8]) -> [u32; 8] {
    let mut h = H;

    // Pre-processing (padding)
    let mut padded = Vec::from(input);
    padded.push(0x80);

    while (padded.len() + 8) % 64 != 0 {
        padded.push(0x00);
    }

    let bit_len = (input.len() * 8) as u64;
    padded.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit chunk
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 64];

        for (i, chunk_part) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([chunk_part[0], chunk_part[1], chunk_part[2], chunk_part[3]]);
        }

        for i in 16..64 {
            let s0 = rotate_left(w[i - 15], 7) ^ rotate_left(w[i - 15], 18) ^ (w[i - 15] >> 3);
            let s1 = rotate_left(w[i - 2], 17) ^ rotate_left(w[i - 2], 19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_temp = h[7];

        for i in 0..64 {
            let s1 = rotate_left(e, 6) ^ rotate_left(e, 11) ^ rotate_left(e, 25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h_temp.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = rotate_left(a, 2) ^ rotate_left(a, 13) ^ rotate_left(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h_temp = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_temp);
    }

    h
}

fn generate_salt() -> String {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let nanos = since_epoch.as_nanos();
    format!("{:x}", nanos)
}

pub fn hash_password(password: &str, salt: &str) -> String {
    let combined = format!("{}{}", salt, password);
    let hash = sha256(combined.as_bytes());
    hash.iter().map(|&x| format!("{:08x}", x)).collect::<String>()
}

fn verify_password(password: &str, salt: &str, stored_hash: &str) -> bool {
    let computed_hash = hash_password(password, salt);
    computed_hash == stored_hash
}