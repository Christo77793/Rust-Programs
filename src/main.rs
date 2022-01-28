use std::io; //the library to get input
use rand::Rng; // the library to get the range fn, gen_range
use std::cmp::Ordering; // the library to get the comparing fn, gen_range

fn main() {
    println!("Main function");
    println!("");
}

pub fn guessing_game() {
    
    // getting a hidden number to be matched
    let hidden_number = rand::thread_rng().gen_range(1, 151);
    
    // print statment to get input
    println!("The secret number is: {}", hidden_number);

    // looping the take input part till we get the right answer
    loop{
        
        // print statment to get input
        println!("Enter your input");

        // declaring the variable (as a String) to store the guessed value
        let mut guess = String::new();

        // this takes in an input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("This failed");  //an error message if the program fails

        // converting guess from string into int
        // trim removes \r\n {when user enters an input, say 5 the program takes it as 5\r\n}
            // \r is a carriage return and is only added in windows
        // parse converts a variety of number types, which is why we specify u32 after guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only positive integers");
                continue;
            }
        };

        // comparing the secret number and the user input
        match guess.cmp(&hidden_number){
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Yay");
                break;
            }
            Ordering::Greater => println!("Too high"),
        }
    }
}