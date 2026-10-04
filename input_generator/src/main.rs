use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Clone, Copy)]
enum InputType {
    String,
    Integer,
    IntegerSeq
}

fn print_usage() {
    eprintln!("Usage: input_generator [OPTIONS]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --type <type>      Input type: string | integer  (default: string)");
    eprintln!("  --count <n>        Number of entries             (default: 1_000_000)");
    eprintln!("  --min-len <n>      Min string length             (default: 4, string only)");
    eprintln!("  --max-len <n>      Max string length             (default: 32, string only)");
    eprintln!("  --min <n>          Min integer value             (default: 0, integer only)");
    eprintln!("  --max <n>          Max integer value             (default: u64::MAX, integer only)");
    eprintln!();
    eprintln!("Available input types:");
    eprintln!("  string             Random ASCII strings");
    eprintln!("  integer            Random u64 integers");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  input_generator");
    eprintln!("  input_generator --type string --count 1000000 --min-len 4 --max-len 32");
    eprintln!("  input_generator --type integer --count 1000000 --min 0 --max 1000000");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let mut input_type = InputType::String;
    let mut count: usize = 1_000_000;
    let mut min_len: usize = 4;
    let mut max_len: usize = 32;
    let mut min_int: u64 = 0;
    let mut max_int: u64 = u64::MAX;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--type" => {
                i += 1;
                match args.get(i).map(|s| s.as_str()) {
                    Some("string")  => input_type = InputType::String,
                    Some("integer") => input_type = InputType::Integer,
                    other => {
                        eprintln!("Error: unknown type '{}'", other.unwrap_or(""));
                        eprintln!("Available: string | integer");
                        std::process::exit(1);
                    }
                }
            }
            "--count"   => { i += 1; count   = args[i].parse().expect("invalid --count"); }
            "--min-len" => { i += 1; min_len = args[i].parse().expect("invalid --min-len"); }
            "--max-len" => { i += 1; max_len = args[i].parse().expect("invalid --max-len"); }
            "--min"     => { i += 1; min_int = args[i].parse().expect("invalid --min"); }
            "--max"     => { i += 1; max_int = args[i].parse().expect("invalid --max"); }
            other => {
                eprintln!("Error: unknown argument '{}'", other);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let mut seed: u64 = 0xdeadbeefcafebabe;
    let file = File::create("../input.bin").unwrap();
    let mut w = BufWriter::new(file);

    match input_type {
        InputType::String => {
            w.write_all(&(count as u64).to_le_bytes()).unwrap();
            w.write_all(&[0u8]).unwrap(); // type byte: 0 = string
            for _ in 0..count {
                let len = min_len + (lcg(&mut seed) as usize % (max_len - min_len + 1));
                w.write_all(&(len as u32).to_le_bytes()).unwrap();
                for _ in 0..len {
                    let c = b'a' + (lcg(&mut seed) % 26) as u8;
                    w.write_all(&[c]).unwrap();
                }
            }
            println!("Generated {} strings to ../input.bin", count);
            println!("  type    : string");
            println!("  min-len : {}", min_len);
            println!("  max-len : {}", max_len);
        }

        InputType::Integer => {
            w.write_all(&(count as u64).to_le_bytes()).unwrap();
            w.write_all(&[1u8]).unwrap(); // type byte: 1 = integer
            let range = if max_int == u64::MAX { u64::MAX } else { max_int - min_int + 1 };
            for _ in 0..count {
                let val = min_int.wrapping_add(lcg(&mut seed) % range);
                w.write_all(&val.to_le_bytes()).unwrap();
            }
            println!("Generated {} random integers to ../input.bin", count);
            println!("  type : integer");
            println!("  min  : {}", min_int);
            println!("  max  : {}", max_int);
        }

        InputType::IntegerSeq => {
            w.write_all(&(count as u64).to_le_bytes()).unwrap();
            w.write_all(&[1u8]).unwrap(); // type byte: 1 = integer
            for i in 0..count {
                w.write_all(&(i as u64).to_le_bytes()).unwrap();
            }
            println!("Generated {} sequential integers to ../input.bin", count);
            println!("  type  : integer-seq");
            println!("  range : 0..{}", count - 1);
        }
    }

    w.flush().unwrap();
}

fn lcg(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state >> 33
}