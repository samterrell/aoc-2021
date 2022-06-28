use lazy_static::lazy_static;
use num_traits::Num;
use regex::Regex;
use std::fmt::Display;
use std::io;
use std::io::BufRead;
use std::ops::{AddAssign, Add, Mul, Sub};

#[derive(Default)]
struct State<T> {
  x: T,
  depth: T,
  angle: T
}

impl<T: Display + Copy> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State(x: {:}, depth: {:}, angle: {:})", self.x, self.depth, self.angle)
    }
}

enum Command<T: Copy> {
    Forward(T),
    Down(T),
    Up(T)
}

impl<T: AddAssign> AddAssign for State<T> {
    fn add_assign(&mut self, other: State<T>) {
            self.x += other.x;
            self.depth += other.depth
    }
}

impl<T: Display + Copy> std::fmt::Display for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Forward(m) => write!(f, "forward({:})", m),
            Command::Down(m) => write!(f, "down({:})", m),
            Command::Up(m) => write!(f, "up({:})", m)
        }
    }
}

fn text_to_vector<T: Copy + Num>(text: &str) -> Option<Command<T>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*(?P<direction>forward|down|up)\s+(?P<magnitude>\d+)\s*$")
        .unwrap();
    }
    let cap = RE.captures(text)?;
    let magnitude_str = cap.name("magnitude")?;
    let magnitude = T::from_str_radix(magnitude_str.as_str(), 10).ok()?;
    match cap.name("direction")?.as_str() {
        "forward" => Some(Command::Forward(magnitude)),
        "up" => Some(Command::Up(magnitude)),
        "down" => Some(Command::Down(magnitude)),
        &_ => None
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T>> State<T> {
    fn apply(self, command: Command<T>) -> State<T> {
        match command {
            Command::Forward(f) => State{x: self.x+f, depth: self.depth + f*self.angle, angle: self.angle},
            Command::Up(u) => State{x: self.x, depth: self.depth, angle: self.angle -u},
            Command::Down(d) => State{x: self.x, depth: self.depth, angle: self.angle +d}
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut position: State<i32> = Default::default();
    for line in stdin.lock().lines() {
        if let Some(motion) = text_to_vector::<i32>(&line.unwrap()) {
            println!("{:} + {:}", position, motion);
            position = position.apply(motion);
        }
    }
    println!("Final position {:}.", position);
    println!("x * depth = {}", position.x * position.depth);
}
