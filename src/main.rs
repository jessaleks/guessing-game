use rand::random;
use std::{error, io};

fn main() {
    println!("Guess the number");
    let actual_number: i32 = (random::<f32>() * 10.0).round() as i32;
    println!("{}", actual_number);
    let mut flag = false;
    while flag == false {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");
        let number: i32 = guess.trim().parse().expect("Enter numbers only");

        println!("You guessed : {}", number);
        if number > 10 {
            println!("Your number must be higher than 0 and lower than 10")
        }
        if number < actual_number {
            println!("Your guess was too low")
        } else if number == actual_number {
            flag = true;
        } else {
            println!("Your guess was too high")
        }
    }
    println!("Your guess was correct")
}
