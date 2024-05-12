use std::io;

pub fn add(){
    println!("Enter your first number to add:");
    let mut number_one = String::new();
    std::io::stdin().read_line(&mut number_one).expect("Failed to read line");
    let number_one: f32 = number_one.trim().parse().expect("Please type a number!");

    println!("Enter your second number to add:");
    let mut number_two = String::new();
    std::io::stdin().read_line(&mut number_two).expect("Failed to read line");
    let number_two: f32 = number_two.trim().parse().expect("Please type a number!");

    let sum = number_one + number_two;
    println!("The sum of {} and {} is {}", number_one, number_two, sum);

}

pub fn subtract(){
    println!("Enter your first number to subtract:");
    let mut number_one = String::new();
    std::io::stdin().read_line(&mut number_one).expect("Failed to read line");
    let number_one: f32 = number_one.trim().parse().expect("Please type a number!");

    println!("Enter your second number to subtract:");
    let mut number_two = String::new();
    std::io::stdin().read_line(&mut number_two).expect("Failed to read line");
    let number_two: f32 = number_two.trim().parse().expect("Please type a number!");

    let sub = number_one - number_two;
    println!("The substract of {} - {} is {}", number_one, number_two, sub);
}

pub fn multiply(){
    println!("Enter your first number to multiply:");
    let mut number_one = String::new();
    std::io::stdin().read_line(&mut number_one).expect("Failed to read line");
    let number_one: f32 = number_one.trim().parse().expect("Please type a number!");

    println!("Enter your second number to multiply:");
    let mut number_two = String::new();
    std::io::stdin().read_line(&mut number_two).expect("Failed to read line");
    let number_two: f32 = number_two.trim().parse().expect("Please type a number!");

    let mul = number_one * number_two;
    println!("The product of {} * {} is {}", number_one, number_two, mul);

}

pub fn divide(){
    println!("Enter your first number to divide:");
    let mut number_one = String::new();
    std::io::stdin().read_line(&mut number_one).expect("Failed to read line");
    let number_one: f32 = number_one.trim().parse().expect("Please type a number!");
    println!("Enter your second number to divide:");
    let mut number_two = String::new();
    std::io::stdin().read_line(&mut number_two).expect("Failed to read line");
    let number_two: f32 = number_two.trim().parse().expect("Please type a number!");
    let div = number_one / number_two;
    println!("The residue of {} / {} is {}", number_one, number_two, div);

}