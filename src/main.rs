use std::{io, cmp::Ordering}; 
use rand::Rng; // random number crate

fn check_loss(i: u32) -> bool {
    if i == 0 {
        println!("You loss! Try again");
        return true;
    } else {
        return false;
    }
}

fn main() { // entry point into the program
    println!("Guess the number between 1 and 100!");

    let secret_number =  rand::thread_rng().gen_range(1..101);

    let mut x: u32 = 10; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {

        println!("Please input your guess.");

        let mut guess = String::new(); 
    
        io::stdin() // allows us to handle user input.
            .read_line(&mut guess)  
            .expect("Failed to read line!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, //goes to next interation of loop if error
        };
        
        println!("You guessed: {}", guess);
    
         match guess.cmp(&secret_number) {
             Ordering::Less => {
                x = x - 1;
                if check_loss(x) {
                    done = true;
                } else {
                    println!("You have {} guess(es) left!", x);
                    println!("Too small!");
                }
             },
             Ordering:: Greater => {
                x = x - 1;
                if check_loss(x) {
                    done = true;
                } else {
                    println!("You have {} guess(es) left!", x);
                    println!("Too big!");
                }
             } ,
             Ordering::Equal => {
                println!("You Win!");
                done = true;
             }
         }
    }
}