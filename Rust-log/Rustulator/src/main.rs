use std::io;

fn main() {
    println!("Welcome to the Rustulator, a calculator in rust, made by greb");
 loop {
        let mut operator = String::new();

        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("");
        println!("1) Addition");
        println!("2) Subtraction");
        println!("3) Multiplication");
        println!("4) Division");
        println!("Choose an option (1/2/3/4)");

        io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line.");
        let operator: i32 = operator.trim().parse().expect("Not a number");

        if operator == 1 {
            println!("Welcome to the Addition Machine");

            println!("Enter the first number to add:");

            io::stdin().read_line(&mut num1).expect("failed to read line");
            let num1: f64 = num1.trim().parse().expect("not a number");

            println!("enter the second number:");

            io::stdin().read_line(&mut num2).expect("failed to read line");
            let num2: f64 = num2.trim().parse().expect("not a number");

            let result = num1 + num2;
            println!("{num1} + {num2} = {result}");

        } else if operator == 2 {
            println!("Welcome to the Subtraction Machine");

            println!("Enter the number to subtract from:");

            io::stdin().read_line(&mut num1).expect("failed to read line");
            let num1: f64 = num1.trim().parse().expect("not a number");

            println!("Enter the number to subtract:");

            io::stdin().read_line(&mut num2).expect("failed to read line");
            let num2: f64 = num2.trim().parse().expect("not a number");

            let result = num1 - num2;
            println!("{num1} - {num2} = {result}");

        } else if operator == 3 {
            println!("Welcome to the Multiplication Machine");

            println!("Enter the first number to multiply:");

            io::stdin().read_line(&mut num1).expect("failed to read line");
            let num1: f64 = num1.trim().parse().expect("not a number");

            println!("Enter the second number:");

            io::stdin().read_line(&mut num2).expect("failed to read line");
            let num2: f64 = num2.trim().parse().expect("not a number");

            let result = num1 * num2;
            println!("{num1} * {num2} = {result}");
        } else if operator == 4 {
            println!("Welcome to the Division Machine");

            println!("Enter the number to be divided:");

            io::stdin().read_line(&mut num1).expect("failed to read line");
            let num1: f64 = num1.trim().parse().expect("not a number");

            println!("enter the number to divide by:");

            io::stdin().read_line(&mut num2).expect("failed to read line");
            let num2: f64 = num2.trim().parse().expect("not a number");

            let result = num1 / num2;
            println!("{num1} / {num2} = {result}");
        } else {
            println!("invalid number");
        }
    }
}
