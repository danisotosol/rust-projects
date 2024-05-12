use std::io;

pub fn add(){
    println!("Enter your first number to add:");
    let mut numberOne = String::new();
    std::io::stdin().read_line(&mut numberOne).expect("Failed to read line");
    let numberOne: u32 = numberOne.trim().parse().expect("Please type a number!");

    println!("Enter your second number to add:");
    let mut numberTwo = String::new();
    std::io::stdin().read_line(&mut numberTwo).expect("Failed to read line");
    let numberTwo: u32 = numberTwo.trim().parse().expect("Please type a number!");

    let sum = numberOne + numberTwo;
    println!("The sum of {} and {} is {}", numberOne, numberTwo, sum);

}

pub fn subtract(){
    println!("Enter your first number to subtract:");
    let mut numberOne = String::new();
    std::io::stdin().read_line(&mut numberOne).expect("Failed to read line");
    let numberOne: u32 = numberOne.trim().parse().expect("Please type a number!");

    println!("Enter your second number to subtract:");
    let mut numberTwo = String::new();
    std::io::stdin().read_line(&mut numberTwo).expect("Failed to read line");
    let numberTwo: u32 = numberTwo.trim().parse().expect("Please type a number!");

    let sub = numberOne - numberTwo;
    println!("The sum of {} and {} is {}", numberOne, numberTwo, sub);
}

pub fn multiply(){
    println!("Enter your first number to multiply:");
    let mut numberOne = String::new();
    std::io::stdin().read_line(&mut numberOne).expect("Failed to read line");
    let numberOne: u32 = numberOne.trim().parse().expect("Please type a number!");

    println!("Enter your second number to multiply:");
    let mut numberTwo = String::new();
    std::io::stdin().read_line(&mut numberTwo).expect("Failed to read line");
    let numberTwo: u32 = numberTwo.trim().parse().expect("Please type a number!");

    let mul = numberOne * numberTwo;
    println!("The sum of {} and {} is {}", numberOne, numberTwo, mul);

}

pub fn divide(){
    println!("Enter your first number to divide:");
    let mut numberOne = String::new();
    std::io::stdin().read_line(&mut numberOne).expect("Failed to read line");
    let numberOne: u32 = numberOne.trim().parse().expect("Please type a number!");
    println!("Enter your second number to divide:");
    let mut numberTwo = String::new();
    std::io::stdin().read_line(&mut numberTwo).expect("Failed to read line");
    let numberTwo: u32 = numberTwo.trim().parse().expect("Please type a number!");
    let div = numberOne / numberTwo;
    println!("The sum of {} and {} is {}", numberOne, numberTwo, div);

}