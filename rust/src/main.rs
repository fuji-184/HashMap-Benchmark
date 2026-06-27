use std::collections::HashMap;
use std::fs;
use std::io::Read;

unsafe extern "C" {
    fn init() -> i32;
    fn start();
    fn stop_and_print();
}

fn read_strings(path: &str) -> Vec<String> {
    let mut file = fs::File::open(path).unwrap();
    let mut raw = Vec::new();
    file.read_to_end(&mut raw).unwrap();

    let n = u64::from_le_bytes(raw[0..8].try_into().unwrap()) as usize;
    let mut strings = Vec::with_capacity(n);
    let mut pos = 8usize;

    for _ in 0..n {
        let len = u32::from_le_bytes(raw[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let s = std::str::from_utf8(&raw[pos..pos+len]).unwrap().to_string();
        pos += len;
        strings.push(s);
    }

    strings
}

fn main() {
    let strings = read_strings("../input.bin");
    let n = strings.len();
    let mut get: i64 = 0;
    let mut get2: i64 = 0;
    let mut m: HashMap<String, i64> = HashMap::new();

    unsafe {
        if init() == 0 {
        println!("Status Init: 0");
        }
    };

    println!("\n=== BENCHMARK: INSERT ===");
    unsafe { start(); }
    for (i, s) in strings.iter().enumerate() {
        m.insert(s.clone(), i as i64);
    }
     unsafe { stop_and_print(); } 

    println!("\n=== BENCHMARK: GET HIT ===");
    unsafe { start(); }
    for s in strings.iter() {
        if let Some(val) = m.get(s.as_str()) {
            get += val;
        }
    }
    unsafe { stop_and_print(); } 

    println!("\n=== BENCHMARK: GET MISS ===");
    unsafe { start(); }
    for s in strings.iter() {
        let miss = s.chars().rev().collect::<String>();
        if let Some(val) = m.get(miss.as_str()) {
            get2 += val;
        }
    }
    unsafe { stop_and_print(); } 

    println!("\n=== BENCHMARK: RE-INSERT ===");
    unsafe { start(); }
    for (i, s) in strings.iter().enumerate() {
        m.insert(s.clone(), i as i64 * 2);
    }
    unsafe { stop_and_print(); } 


    println!("\n=== BENCHMARK: REMOVE ===");
    unsafe { start(); }
    for s in strings.iter() {
        m.remove(s.as_str());
    }
    unsafe { stop_and_print(); } 

    println!("\nN: {}\nGet: {}", n, get - get2);
}
