extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        // Rust allows us to shadow the previous value of guess with a new one.
        // This feature is often used in similar situations in which you want to
        // convert a value from one type to another type. Shadowing lets us
        // reuse the guess variable name rather than forcing us to create two
        // unique variables

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        // A match expression is made up of arms. An arm consists of a pattern
        // and the code that should be run if the value given to the beginning
        // of the match expression fits that arm’s pattern. Rust takes the value
        // given to match and looks through each arm’s pattern in turn. The
        // match construct and patterns are powerful features in Rust that let
        // you express a variety of situations your code might encounter and
        // helps ensure that you handle them all. These features will be covered
        // in detail in Chapter 6 and Chapter 18, respectively.

        // Say that the user has guessed 50, and the randomly generated secret
        // number this time is 38. When the code compares 50 to 38, the cmp
        // method will return Ordering::Greater, because 50 is greater than 38.
        // Ordering::Greater is the value that the match expression gets. It
        // looks at the first arm’s pattern, Ordering::Less, but the value
        // Ordering::Greater does not match Ordering::Less, so it ignores the
        // code in that arm and moves to the next arm. The next arm’s pattern,
        // Ordering::Greater, does match Ordering::Greater! The associated code
        // in that arm will execute and print Too big! to the screen. The match
        // expression ends because it has no need to look at the last arm in
        // this particular scenario.

        match guess.cmp(&secret_number){
            Ordering::Less  => println!("Too small!"),
            Ordering::Greater  => println!("Too big!"),
            Ordering::Equal  => {
                println!("You win!");
                break;
            }
        }
    } 
}
