use std::io; //the library to get input
use rand::Rng; // the library to get the range fn, gen_range
use std::cmp::Ordering; // the library to get the comparing fn, gen_range

fn main() {
    println!("Main function");
    println!("");

    if_with_let();
}

pub fn if_with_let() {

    println!("Enter true or false");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Errorrr message");

    let user_input: bool = match user_input.trim().parse() {
        Ok(boolean) => boolean,
        Err(_) => todo!(), // print statement does not work if it is not in a loop
    };

    let mut x = 0;

    println!("x before is {x}");

    if user_input == true {
        x = 5;
    } else {
        x = 3;
    }

    let y = if x>4 { 15 } else {13}; // this if else statement must have the same compatible data types

    print!("x is {x} y is {y}")
}

pub fn if_conditional() {
    
    loop {

        let mut x = String::new();

        println!("Enter a number");

        io::stdin()
            .read_line(&mut x)
            .expect("Error");

        let x: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only positive integers");
                continue;
            }
        };
        
        if x % 12 == 0 {
            println!("{x} is divisible by 12");
        } else if x % 8 == 0 {
            println!("{x} is divisible by 8");
        } else if x % 2 == 0 {
            println!("{x} is divisible by 2");
        } else {
            println!("{x} is not divisible by 12");
            println!("{x} is not divisible by 8");
            println!("{x} is not divisible by 2");
            print!("Not divisible by these numbers at all");
            break;
        }
    }
}

pub fn var_name() -> &'static str {
    "Justification"
}

// statements do not return a value; eg let
// expressions do return a value; they do not use a semicolon

// let a = ["Jesus"; 5];
// is the same as ["Jesus", "Jesus", "Jesus", "Jesus", "Jesus"]

pub fn char_vs_str() {

    // for char literals use only ''
    let char_var = 'A';

    // for string literals use only ""
    let string_var = "AAA";

    print!("{} {}", char_var, string_var);
}

pub fn mut_vs_shadowing() {

    let x = 5;

    let x = x + 1; // using let again allows us to change the value; as well as the type while retaining variable name

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
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
            // u32 is a type annotation
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