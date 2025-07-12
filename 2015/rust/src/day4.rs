#[rustfmt::skip]
const MD5_S : [u32; 64 ] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

#[rustfmt::skip]
const MD5_K : [u32; 64 ] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const MD5_A: u32 = 0x67452301;
const MD5_B: u32 = 0xefcdab89;
const MD5_C: u32 = 0x98badcfe;
const MD5_D: u32 = 0x10325476;

fn md5(input: &str) -> String {
    let input_bytes = input.as_bytes();
    let bitcount = input_bytes.len().wrapping_mul(8) as u64;

    let mut padded = Vec::from(input_bytes);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0_u8);
    }
    padded.extend(bitcount.to_le_bytes().iter());

    let padded_ints = padded
        .chunks_exact(4)
        .map(|x| u32::from_le_bytes(<[u8; 4]>::try_from(x).expect("Chunks of 4")))
        .collect::<Vec<_>>();

    let int_chunks: Vec<&[u32]> = padded_ints.chunks_exact(16).collect();

    let mut hash_a = MD5_A;
    let mut hash_b = MD5_B;
    let mut hash_c = MD5_C;
    let mut hash_d = MD5_D;

    for chunk in int_chunks {
        // Initialize hash value for this chunk:
        let mut a = hash_a;
        let mut b = hash_b;
        let mut c = hash_c;
        let mut d = hash_d;

        for i in 0..=63 {
            let f: u32;
            let g: u32;

            match i {
                0..=15 => {
                    // F := (B and C) or ((not B) and D)
                    // g := i
                    f = (b & c) | (!b & d);
                    g = i;
                }
                16..=31 => {
                    // F := (D and B) or ((not D) and C)
                    // g := (5×i + 1) mod 16
                    f = (d & b) | (!d & c);
                    g = i.wrapping_mul(5).wrapping_add(1) % 16;
                }
                32..=47 => {
                    // F := B xor C xor D
                    // g := (3×i + 5) mod 16
                    f = b ^ c ^ d;
                    g = i.wrapping_mul(3).wrapping_add(5) % 16;
                }
                48..=63 => {
                    // F := C xor (B or (not D))
                    // g := (7×i) mod 16
                    f = c ^ (b | !d);
                    g = i.wrapping_mul(7) % 16;
                }
                64_u32..=u32::MAX => unimplemented!("Value bounded by for loop"),
            }

            // F := F + A + K[i] + M[g]  // M[g] must be a 32-bit block
            // A := D
            // D := C
            // C := B
            // B := B + leftrotate(F, s[i])
            let f: u32 = f
                .wrapping_add(a)
                .wrapping_add(MD5_K[usize::try_from(i).expect("Value should be <64")])
                .wrapping_add(chunk[usize::try_from(g).expect("Value should be <64")]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                f.rotate_left(MD5_S[usize::try_from(i).expect("Value should be <64")]),
            );
        }

        hash_a = hash_a.wrapping_add(a);
        hash_b = hash_b.wrapping_add(b);
        hash_c = hash_c.wrapping_add(c);
        hash_d = hash_d.wrapping_add(d);
    }

    let mut digest = String::new();
    for b in [hash_a, hash_b, hash_c, hash_d]
        .iter()
        .flat_map(|x| x.to_le_bytes())
    {
        digest.push_str(format!("{b:02x}").as_str());
    }
    digest
}

pub fn solve(input: impl AsRef<str>) {
    let mut suffix = 0_u64;
    while !md5(format!("{}{}", input.as_ref(), suffix).as_str()).starts_with("00000") {
        suffix += 1;
    }
    println!("Part 1: {suffix}");

    while !md5(format!("{}{}", input.as_ref(), suffix).as_str()).starts_with("000000") {
        suffix += 1;
    }
    println!("Part 2: {suffix}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            md5("The quick brown fox jumps over the lazy dog"),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
        assert_eq!(
            md5("The quick brown fox jumps over the lazy dog."),
            "e4d909c290d0fb1ca068ffaddf22cbd0"
        );
    }

    #[test]
    fn test_md5_empty() {
        assert_eq!(md5(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}
