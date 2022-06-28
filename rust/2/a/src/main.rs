use lazy_static::lazy_static;
use nalgebra::Vector2;
use num_traits::Num;
use regex::Regex;
use std::io;
use std::io::BufRead;

fn text_to_vector<T: Num + Copy>(text: &str) -> Option<Vector2<T>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*(?P<direction>forward|down|up)\s+(?P<magnitude>\d+)\s*$")
        .unwrap();
    }
    let cap = RE.captures(text)?;
    let magnitude_str = cap.name("magnitude")?;
    let magnitude = T::from_str_radix(magnitude_str.as_str(), 10).ok()?;
    let nonmag = magnitude - magnitude;
    match cap.name("direction")?.as_str() {
        "forward" => Some(Vector2::new(magnitude, nonmag)),
        "up" => Some(Vector2::new(nonmag, nonmag-magnitude)),
        "down" => Some(Vector2::new(nonmag, magnitude)),
        &_ => None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut position = Vector2::new(0,0);
    for line in stdin.lock().lines() {
        if let Some(motion) = text_to_vector(&line.unwrap()) {
            println!("x:{} depth:{}", motion.x, motion.y);
            position += motion;
        }
    }
    println!("Final position x={} depth={}.", position.x, position.y);
    println!("x*depth={}", position.x * position.y);
}
