use std::io;
use std::io::Read;

fn main() {
    let stdin = io::stdin();
    let mut bits: usize = 0;
    let mut counts = vec![0; 32];
    let mut i: usize = 0;
    for maybe_byte in stdin.lock().bytes() {
        match maybe_byte {
            Ok(b'1') => (counts[i], i) = (counts[i] + 1, i + 1),
            Ok(b'0') => (counts[i], i) = (counts[i] - 1, i + 1),
            Ok(b'\n') => (bits, i) = (bits.max(i), 0),
            _ => (),
        }
    }
    if bits >= 64 {
        panic!("Too many bits!")
    }
    let mut gamma: u128 = 0;
    let mut epsilon: u128 = 0;
    for i in 0..bits {
        gamma <<= 1;
        epsilon <<= 1;
        if counts[i] == 0 {
            panic!("Equal bit counts!")
        }
        if counts[i] > 0 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    println!(
        "gamma: {}, epsilon: {}, power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
