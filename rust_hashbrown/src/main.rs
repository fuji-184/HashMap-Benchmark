use hashbrown::HashMap;
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
    unsafe {
        let input = fs::read_to_string("../input.txt").unwrap();
        let n: i64 = input.trim().parse().unwrap();
        let mut get: i64 = 0;
        let mut m = HashMap::new();

        if init() != 0 {
            panic!("error when initializing counter");
        }

        start();
        for i in 0..n {
            m.insert(i, i);
        }
        stop_and_print();

        start();
        for i in 0..n {
            if let Some(val) = m.get(&i) {
                get += val;
            }
        }
        stop_and_print();

        let mut get2: i64 = 0;

        start();
        for i in n..(n * 2) {
            if let Some(val) = m.get(&i) {
                get2 += val;
            }
        }
        stop_and_print();
        start();
        for i in 0..n {
            m.insert(i, i * 2);
        }
        stop_and_print();

        start();
        for i in -1..n {
            m.remove(&i);
        }
        stop_and_print();

        println!("N: {}\nGet: {}", n, get - get2);
    }
}
