use std::io; // brings input/output library into scope
use rand::Rng;
use std::cmp::Ordering; // Ordering is enum with variants Less, Greater, and Equal --> these are the 3 possible outcomes when you compare values

pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value should be between 1 and 100, got {value} instead")
        }
        Guess {value} 
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess...");

        // variables are immutable by default in rust, thus requiring use of the 'mut' operator to switch them to mutable
        // the :: syntax indicates that new is an associated funciton of the String type (an associated funciton is a function that's implemented on a type)
        let mut guess = String::new(); 

        io::stdin()
            // & indicates that the argument is a reference --> a way to let you code access a piece of data without needing to copy that data into memory multiple times (immutable by default)
            .read_line(&mut guess) // read_line takes user input and appends it into a string (without overwriting contents)
            .expect("Failed to read line");

        // shadow declaration of the previous guess var we declared
        // .trim() trims whitespace
        // .parse() is used to convert a string to another type
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Not a numeric guess");
                continue; // continue returns execution to the top of the loop
            }
        };

        println!("You guessed: {}", guess.value);

        // this is pattern matching at work
        // cmp method compares 2 values and can be called ona nything that can be compared
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break makes the program exit the loop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Value should be between 1 and 100")]
    fn test_invalid_guess_panics() {
        Guess::new(1000);
    }
}