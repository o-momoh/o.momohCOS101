use std::io;

fn main() {
    let mut food_input = String::new();
    let mut quantity_input = String::new();

    println!("========== RESTAURANT MENU ==========");
    println!("Code\tItem\t\t\t\tPrice");
    println!("P\tPoundo Yam / Edinkaiko Soup\tN3,200");
    println!("F\tFried Rice & Chicken\t\tN3,000");
    println!("A\tAmala & Ewedu Soup\t\tN2,500");
    println!("E\tEba & Egusi Soup\t\tN2,000");
    println!("W\tWhite Rice & Stew\t\tN2,500");
    println!("=====================================");

    println!("Enter the food code:");
    io::stdin()
        .read_line(&mut food_input)
        .expect("Failed to read input");

    let food = food_input.trim().to_uppercase();

    println!("Enter the quantity:");
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: u32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a valid quantity");

    let price: u32;

    if food == "P" {
        price = 3200;
    } else if food == "F" {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "E" {
        price = 2000;
    } else if food == "W" {
        price = 2500;
    } else {
        println!("Invalid food code.");
        return;
    }

    let total = price * quantity;

    println!();
    println!("Quantity ordered: {}", quantity);
    println!("Price per item: N{}", price);
    println!("Total charge: N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount (5%): N{}", discount);
        println!("Final amount: N{}", final_total);
    } else {
        println!("No discount applied.");
        println!("Final amount: N{}", total);
    }
}