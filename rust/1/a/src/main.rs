use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut current = None;
    let mut increases = 0;
    for line in stdin.lock().lines() {
        if let Ok(i) = i64::from_str_radix(&line.unwrap(), 10) {
            print!("{}", i);
            if let Some(last) = current {
                if i > last {
                    println!(" (increased)");
                    increases = increases + 1;
                } else if i < last {
                    println!(" (decreased)");
                } else {
                    println!(" (unchanged)");
                }
            } else {
                println!(" (N/A - no previous measurement)");
            }
            current = Some(i);
        }
    }
    println!("Increased {} time(s).", increases);
}
