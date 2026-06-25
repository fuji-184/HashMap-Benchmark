use ahash::AHashMap as HashMap;
use std::arch::asm;
use std::fs;

fn rdtsc() -> u64 {
    let mut low: u32;
    let mut high: u32;
    unsafe {
        asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));
    }
    ((high as u64) << 32) | (low as u64)
}

fn fmt_num(num: u64) -> String {
    if num < 1000 {
        return num.to_string();
    }
    format!("{}.{:03}", fmt_num(num / 1000), num % 1000)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let n: i64 = input.trim().parse().unwrap();

    let mut get: i64 = 0;
    let mut m = HashMap::new();

    let start_insert = rdtsc();
    for i in 0..n {
        m.insert(i, i);
    }
    let end_insert = rdtsc();

    let start_hit = rdtsc();
    for i in 0..n {
        if let Some(val) = m.get(&i) {
            get += val;
        }
    }
    let end_hit = rdtsc();

    let mut get2: i64 = 0;

    let start_miss = rdtsc();
    for i in n..(n * 2) {
        if let Some(val) = m.get(&i) {
            get2 += val;
        }
    }
    let end_miss = rdtsc();

    let start_update = rdtsc();
    for i in 0..n {
        m.insert(i, i * 2);
    }
    let end_update = rdtsc();

    let start_delete = rdtsc();
    for i in 0..n {
        m.remove(&i);
    }
    let end_delete = rdtsc();

    println!("N: {}\nGet: {}", n, get - get2);
    println!("Insert: {} cycles", fmt_num(end_insert - start_insert));
    println!("Get Hit: {} cycles", fmt_num(end_hit - start_hit));
    println!("Get Miss: {} cycles", fmt_num(end_miss - start_miss));
    println!("Update: {} cycles", fmt_num(end_update - start_update));
    println!("Delete: {} cycles", fmt_num(end_delete - start_delete));
}
