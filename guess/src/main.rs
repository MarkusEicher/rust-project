use std::io;
use colored::Colorize;


fn main() {
    println!("Guess a number game...");
    let mut user_input = String::new();
    let random_number = rand::random_range(1..101);
    println!("Random number is {}", random_number);

    loop {
        // let mut user_input = String::new();
        user_input.clear();
        println!("Please enter a number between 1 and 100...");
        io::stdin()
            .read_line(&mut user_input) 
            .expect("Failed to read the line");
        let user_input: i32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You entered {}", user_input);
         
        match random_number.cmp(&user_input)  {
            std::cmp::Ordering::Less => println!("{}", "Your guess is too high...".red()),
            std::cmp::Ordering::Greater => println!("{}", "Your guess is too low...".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You win!!!".green());
                break;
            },
        }
        
    };
    
    


    
}
