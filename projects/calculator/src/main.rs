mod operations;
use std::io;

fn main() {
    loop{
        
        //I will comment the code later ;p

        println!("\n     Welcome to the calculator!");
        println!("     Please choose an operation:");
        println!("------------------------------------");
        println!("1.           Add");
        println!("2.           Subtract");
        println!("3.           Multiply");
        println!("4.           Divide");
        println!("0.           Exit");
        println!("------------------------------------");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                return;
            }
        };

        match choice {
            1 => operations::add(),
            2 => operations::subtract(),
            3 => operations::multiply(),
            4 => operations::divide(),
            0 => {
                println!("Exiting the calculator...");
                break;
            }
            _ => {
                println!("Invalid choice! Please choose a valid operation.");
                return;
            }

        };
    }
}
