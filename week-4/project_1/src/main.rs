// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("First root: {}", root1);
        println!("Second root: {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);

        println!("Equal roots: {}", root);
    } else {
        println!("No real roots");
    }
}