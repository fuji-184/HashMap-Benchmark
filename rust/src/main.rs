use std::collections::HashMap;
use std::arch::asm;
use std::fs;

unsafe extern "C" {
    fn init() -> i32;
    fn start();
    fn stop_and_print();
}

fn fmt_num(num: u64) -> String {
    if num < 1000 {
        return num.to_string();
    }
    format!("{}.{:03}", fmt_num(num / 1000), num % 1000)
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let n: i64 = input.trim().parse().unwrap();

    let mut get: i64 = 0;
    let mut m = HashMap::new();
    let mut get2: i64 = 0;

    let is_perf_ready = unsafe {
        let init_status = init();
        println!("Status Init: {}", init_status);
        init_status == 0
    };

    if is_perf_ready { 
        println!("\n=== BENCHMARK: INSERT ===");
        unsafe { start(); } 
    }
    for i in 0..n {
        m.insert(i, i);
    }
    if is_perf_ready { unsafe { stop_and_print(); } }

    if is_perf_ready { 
        println!("\n=== BENCHMARK: GET 1 ===");
        unsafe { start(); } 
    }
    for i in 0..n {
        if let Some(val) = m.get(&i) {
            get += val;
        }
    }
    if is_perf_ready { unsafe { stop_and_print(); } }

    if is_perf_ready { 
        println!("\n=== BENCHMARK: GET 2 ===");
        unsafe { start(); } 
    }
    for i in n..(n * 2) {
        if let Some(val) = m.get(&i) {
            get2 += val;
        }
    }
    if is_perf_ready { unsafe { stop_and_print(); } }

    if is_perf_ready { 
        println!("\n=== BENCHMARK: RE-INSERT ===");
        unsafe { start(); } 
    }
    for i in 0..n {
        m.insert(i, i * 2);
    }
    if is_perf_ready { unsafe { stop_and_print(); } }

    if is_perf_ready { 
        println!("\n=== BENCHMARK: REMOVE ===");
        unsafe { start(); } 
    }
    for i in 0..n {
        m.remove(&i);
    }
    if is_perf_ready { unsafe { stop_and_print(); } }

    println!("\nN: {}\nGet: {}", n, get - get2);
}
