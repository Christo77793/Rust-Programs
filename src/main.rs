use std::io; //the library to get input
use rand::Rng; // the library to get the range fn, gen_range
use std::cmp::Ordering; // the library to get the comparing fn, gen_range

// Book number 1: Page number 79
// Book number 2: Not started

fn main() {
    println!("Main function");
    println!("");
    
    days_of_christmans();
}

pub fn days_of_christmans() {

    let number_of_days = 12;

    let song_lyrics = 
        [
        "My true love sent to me", //0
        "And a partridge in a pear tree", //1
        "Two turtle-doves", //2
        "Three French hens", //3
        "Four calling birds", //4
        "Five golden rings (five golden rings)", //5
        "Six geese a-laying", //6
        "Seven swans a-swimming", //7
        "Eight maids a-milking", //8
        "Nine ladies dancing", //9
        "Ten lords a-leaping", //10
        "Eleven pipers piping", //11
        "Twelve drummers drumming", //12
        "A partridge in a pear tree", //13
        ];

    for day_number in 0 .. number_of_days {

        let exact_day_number = day_number + 1;

        println!("");
        println!("Day Number: {}", exact_day_number);

        // Day 1
        if exact_day_number == 1 {

            println!("On the {}st day of Christmas", exact_day_number);
            println!("{}", song_lyrics[0]);
        
            println!("{}", song_lyrics[13]);
        }

        // Day 2
        else if exact_day_number == 2 {

            println!("On the {}nd day of Christmas", exact_day_number);
            println!("{}", song_lyrics[0]);
            
            for lyric_index in (1 ..exact_day_number + 1).rev() {
                println!("{}", song_lyrics[lyric_index]);
            }
        }

        // Day 3
        else if exact_day_number == 3 {

            println!("On the {}rd day of Christmas", exact_day_number);
            println!("{}", song_lyrics[0]);
            
            for lyric_index in (1 ..exact_day_number + 1).rev() {
                println!("{}", song_lyrics[lyric_index]);
            }
        }

        // Day 4 and onwards
        else if exact_day_number >= 4 {

            println!("On the {}th day of Christmas", exact_day_number);
            println!("{}", song_lyrics[0]);
            
            for lyric_index in (1 ..exact_day_number + 1).rev() {
                println!("{}", song_lyrics[lyric_index]);
            }
        }
    }
}

pub fn fibonnaci() {
    
    loop {

        let mut user_input = String::new();

        println!("Enter the nth number: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error Message");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter numbers only");
                continue;
            }
        };

        let mut a = 0;
        let mut b = 0;
        let mut c = 1;

        println!("Fibonacci starts");

        print!("0 1 ");

        for _x in 0 .. user_input {

            a = b + c;
            b = c;
            c = a;

            print!("{} ", a);

        }

        break;
    }
}

pub fn farenheit_celcius() {

    loop {

        let mut user_input = String::new();

        println!("Enter the temperature:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Error Message!!!");

        let user_input: f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers");
                continue;
            }
        };

        println!("Do you want to convert it to Farenheit or Celcius?");

        println!("Farnheit to Celcius: Enter 1");
        println!("Celcius to Farnheit: Enter 2");

        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Error Message!!!");

        let user_choice: f32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers");
                continue;
            }
        };

        if user_choice == 1.0 {
            let result: f32 = (((user_input - 32.0) * 5.0) / 9.0) as f32;

            println!("The temperature in Celcius is {:.32}C", result);
            println!("");
        }
        else if user_choice == 2.0 {
            let result: f32 = (((user_input * 9.0) / 5.0) + 32.0) as f32;

            println!("The temperature in Farenheit is {:.32}F", result);
            break;
        }
    }
}

pub fn for_with_arrays(){

    let array = ["Jesus", "is", "King!"];

    let mut array_index = 0;

    for x in array {
        println!("{}, and array index is: {}",x, array_index);
        
        array_index += 1;
    }
}

pub fn loop_with_inner_break(){

    let mut x = 1;

    let y = 

        loop {

            x += 4;

            if x == 3 {
                break x * 5;
            }

            if x == 5 {
                break x * 5;
            }

        };

    println!("x is {} and y is {}", x ,y);
}

pub fn loop_with_name() {
    let mut count = 0;

    'loop_1: loop {

        println!("Count is {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining is {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'loop_1;
            }
            remaining -= 1;
        }
        count += 1;
    }
    print!("Program end! Count is {}", count);
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

    let y = if x>4 {15} else {13}; // this if else statement must have the same compatible data types

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