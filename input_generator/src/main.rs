use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);

    let min_len = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4usize);

    let max_len = std::env::args()
        .nth(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(32usize);

    let file = File::create("../input.bin").unwrap();
    let mut w = BufWriter::new(file);

    w.write_all(&(n as u64).to_le_bytes()).unwrap();

    let mut seed: u64 = 0xdeadbeefcafebabe;

    for _ in 0..n {
        let len = min_len + (lcg(&mut seed) as usize % (max_len - min_len + 1));
        w.write_all(&(len as u32).to_le_bytes()).unwrap();
        for _ in 0..len {
            let c = b'a' + (lcg(&mut seed) % 26) as u8;
            w.write_all(&[c]).unwrap();
        }
    }

    w.flush().unwrap();
    println!("Generated {} strings to ../input.bin", n);
}

fn lcg(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state >> 33
}
