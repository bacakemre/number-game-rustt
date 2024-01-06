use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("Guess the number game 1-100 !!!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
 
    println!("The secret number is: {}" , secret_number);

    println!("You will not see the secret number in the real game.");

    println!("If you don't want to see the secret number, you can delete the line (12).");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed the read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed,{}", guess);
    
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            }
        
        }  

    }

    

}