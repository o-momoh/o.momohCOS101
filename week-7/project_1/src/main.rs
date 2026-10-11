use std::f64::consts::PI;
use std::io;

fn main() {
    let mut input = String::new();

    println!("SHAPE CALCULATOR");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");
    println!("Enter your choice: ");

    io::stdin().read_line(&mut input).expect("Input error");

    let choice: i32 = input.trim().parse().expect("Enter a number");

    match choice {
        1 => {
            let mut h = String::new();
            let mut a = String::new();
            let mut b = String::new();

            println!("Enter height: ");
            io::stdin().read_line(&mut h).expect("Input error");

            println!("Enter first parallel side: ");
            io::stdin().read_line(&mut a).expect("Input error");

            println!("Enter second parallel side: ");
            io::stdin().read_line(&mut b).expect("Input error");

            let h: f64 = h.trim().parse().expect("Invalid number");
            let a: f64 = a.trim().parse().expect("Invalid number");
            let b: f64 = b.trim().parse().expect("Invalid number");

            let area = (h / 2.0) * (a + b);
            println!("Area of trapezium = {}", area);
        }

        2 => {
            let mut d1 = String::new();
            let mut d2 = String::new();

            println!("Enter first diagonal: ");
            io::stdin().read_line(&mut d1).expect("Input error");

            println!("Enter second diagonal: ");
            io::stdin().read_line(&mut d2).expect("Input error");

            let d1: f64 = d1.trim().parse().expect("Invalid number");
            let d2: f64 = d2.trim().parse().expect("Invalid number");

            let area = (d1 * d2) / 2.0;
            println!("Area of rhombus = {}", area);
        }

        3 => {
            let mut b = String::new();
            let mut h = String::new();

            println!("Enter base: ");
            io::stdin().read_line(&mut b).expect("Input error");

            println!("Enter height: ");
            io::stdin().read_line(&mut h).expect("Input error");

            let b: f64 = b.trim().parse().expect("Invalid number");
            let h: f64 = h.trim().parse().expect("Invalid number");

            let area = b * h;
            println!("Area of parallelogram = {}", area);
        }

        4 => {
            let mut s = String::new();

            println!("Enter side length: ");
            io::stdin().read_line(&mut s).expect("Input error");

            let s: f64 = s.trim().parse().expect("Invalid number");

            let area = 6.0 * s * s;
            println!("Surface area of cube = {}", area);
        }

        5 => {
            let mut r = String::new();
            let mut h = String::new();

            println!("Enter radius: ");
            io::stdin().read_line(&mut r).expect("Input error");

            println!("Enter height: ");
            io::stdin().read_line(&mut h).expect("Input error");

            let r: f64 = r.trim().parse().expect("Invalid number");
            let h: f64 = h.trim().parse().expect("Invalid number");

            let volume = PI * r * r * h;
            println!("Volume of cylinder = {}", volume);
        }

        _ => println!("Invalid choice"),
    }
}
