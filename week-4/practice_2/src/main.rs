
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the three sides of the triangle:");

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f32 = input.trim().parse().expect("Invalid number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f32 = input.trim().parse().expect("Invalid number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: f32 = input.trim().parse().expect("Invalid number");

    let s = (a + b + c) / 2.0;
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    println!("Area of the triangle: {}", area);
}