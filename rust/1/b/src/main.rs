use std::io;
use std::io::BufRead;

struct Ring<T, const N: usize> {
    buffer: [Option<T>; N],
    offset: usize,
}

impl<T: std::ops::AddAssign + std::default::Default, const N: usize> Ring<T, N>
where
    Option<T>: Copy,
{
    pub fn new() -> Self {
        Ring {
            buffer: [None; N],
            offset: 0,
        }
    }

    pub fn sum(&self) -> Option<T> {
        let mut sum: T = Default::default();
        for item in self.buffer {
            match item {
                None => return None,
                Some(val) => sum += val,
            }
        }
        return Some(sum);
    }

    pub fn push(&mut self, item: T) {
        self.buffer[self.offset] = Some(item);
        self.offset += 1;
        if self.offset >= N {
            self.offset = 0;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut ring = Ring::<i64, 3>::new();
    let mut current = None;
    let mut increases = 0;
    for line in stdin.lock().lines() {
        if let Ok(i) = i64::from_str_radix(&line.unwrap(), 10) {
            ring.push(i);
            if let Some(next) = ring.sum() {
                print!("{}", next);
                if let Some(last) = current {
                    if next > last {
                        println!(" (increased)");
                        increases = increases + 1;
                    } else if next < last {
                        println!(" (decreased)");
                    } else {
                        println!(" (unchanged)");
                    }
                } else {
                    println!(" (N/A - no previous measurement)");
                }
                current = Some(next);
            }
        }
    }
    println!("Increased {} time(s).", increases);
}
