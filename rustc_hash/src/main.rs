use rustc_hash::FxHashMap as HashMap;
use std::fs;
use std::io::Read;

unsafe extern "C" {
    fn init();
    fn start();
    fn stop();
    fn print();
    fn reset();
}

enum InputType { String, Integer }

fn detect_type(raw: &[u8]) -> InputType {
    if raw[8] == 1 { InputType::Integer } else { InputType::String }
}

fn read_strings(raw: &[u8]) -> Vec<String> {
    let n = u64::from_le_bytes(raw[0..8].try_into().unwrap()) as usize;
    let mut result = Vec::with_capacity(n);
    let mut pos = 9usize;
    for _ in 0..n {
        let len = u32::from_le_bytes(raw[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let s = std::str::from_utf8(&raw[pos..pos+len]).unwrap().to_string();
        pos += len;
        result.push(s);
    }
    result
}

fn read_integers(raw: &[u8]) -> Vec<u64> {
    let n = u64::from_le_bytes(raw[0..8].try_into().unwrap()) as usize;
    let mut result = Vec::with_capacity(n);
    let mut pos = 9usize;
    for _ in 0..n {
        let val = u64::from_le_bytes(raw[pos..pos+8].try_into().unwrap());
        pos += 8;
        result.push(val);
    }
    result
}

fn run_benchmarks_string(keys: &[String]) {
    let n = keys.len();
    let mut get: i64 = 0;
    let mut get2: i64 = 0;
    let mut m: HashMap<String, i64> = HashMap::default();

    println!("\n=== BENCHMARK: INSERT ===");
    for (i, k) in keys.iter().enumerate() {
        unsafe { start(); }
        m.insert(k.clone(), i as i64);
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: GET HIT ===");
    for k in keys.iter() {
        unsafe { start(); }
        let val = m.get(k.as_str()).copied();
        unsafe { stop(); }
        if let Some(v) = val { get += v; }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: GET MISS ===");
    for k in keys.iter() {
        let miss: String = k.chars().rev().collect();
        unsafe { start(); }
        let val = m.get(miss.as_str()).copied();
        unsafe { stop(); }
        if let Some(v) = val { get2 += v; }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: RE-INSERT ===");
    for (i, k) in keys.iter().enumerate() {
        let v = i as i64 * 2;
        unsafe { start(); }
        m.insert(k.clone(), v);
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: REMOVE ===");
    for k in keys.iter() {
        unsafe { start(); }
        m.remove(k.as_str());
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\nN: {}\nGet: {}", n, get - get2);
}

fn run_benchmarks_integer(keys: &[u64]) {
    let n = keys.len();
    let mut get: i64 = 0;
    let mut get2: i64 = 0;
    let mut m: HashMap<u64, i64> = HashMap::default();

    println!("\n=== BENCHMARK: INSERT ===");
    for (i, &k) in keys.iter().enumerate() {
        unsafe { start(); }
        m.insert(k, i as i64);
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: GET HIT ===");
    for &k in keys.iter() {
        unsafe { start(); }
        let val = m.get(&k).copied();
        unsafe { stop(); }
        if let Some(v) = val { get += v; }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: GET MISS ===");
    for &k in keys.iter() {
        let miss = k + 1;
        unsafe { start(); }
        let val = m.get(&miss).copied();
        unsafe { stop(); }
        if let Some(v) = val { get2 += v; }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: RE-INSERT ===");
    for (i, &k) in keys.iter().enumerate() {
        let v = i as i64 * 2;
        unsafe { start(); }
        m.insert(k, v);
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\n=== BENCHMARK: REMOVE ===");
    for &k in keys.iter() {
        unsafe { start(); }
        m.remove(&k);
        unsafe { stop(); }
    }
    unsafe { print(); reset(); }

    println!("\nN: {}\nGet: {}", n, get - get2);
}

fn main() {
    let mut file = fs::File::open("../input.bin").unwrap();
    let mut raw = Vec::new();
    file.read_to_end(&mut raw).unwrap();

    unsafe { init(); }

    match detect_type(&raw) {
        InputType::Integer => {
            println!("[i] Detected: integer");
            run_benchmarks_integer(&read_integers(&raw));
        }
        InputType::String => {
            println!("[i] Detected: string");
            run_benchmarks_string(&read_strings(&raw));
        }
    }
}