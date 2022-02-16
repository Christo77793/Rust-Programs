use std::io; //the library to get input
use rand::Rng; // the library to get the range fn, gen_range
use std::cmp::Ordering; // the library to get the comparing fn, gen_range

// Book number 1: Page number 128
// Book number 2: Not started

fn main() {
    println!("Main function");
    println!("");

    /*
    // custom struct fn to learn struct init shorthand

    let struct_var = struct_init(String::from("The Batman"), 10);
    print!("Movie name is {}, it is rated {} on IMDB and is of the {} genre", struct_var.name, struct_var.rating, struct_var.genre);
    */
    
    struct_use();
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

// impl stands for implementation
impl Rectangle {
    fn area_calculator(
        &self  // this actually is a short form for self: &Self
        // Self is an alias for the type that the impl block is for
    ) -> u32 {
    
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

    fn validate(&self) -> bool {

        // Checking if width * height is greater than 
        self.width > 0 && self.height > 0
    }
}

pub fn struct_use() {

    let rect1 = Rectangle {
        width: 55,
        height: 55
    };

    // if rect1.validate() {  // using custom functions
    if rect1.height() && rect1.width() {  //using getters
    
        // We can calculate since values are > 0
        println!("Values are valid");
        println!("The area of the rectangle is {}", rect1.area_calculator());
        println!("Values printed using a method fn");
    }
    else {
        println!("Values not valid");
    }
}

pub fn debug_struct() {
    
    let rect1 = Rectangle {
        width: 1,
        height: 2,
    };

    println!("Values of the instance rect1 is {:?}", rect1);
    // Since we cannot directly display the values of a struct we need to use the specifier :?
    // We also need to add "#[derive(Debug)]" to the struct we are creating an instance of
}

#[allow(non_snake_case)]
pub fn tuple_struct() {

    struct Ratings(i32, i32);

    let Batman_Ratings = Ratings(10, 10);

    let Endgame_Ratings = Ratings(4, 10);

    println!("The Batman was rated as {} out of {}", Batman_Ratings.0, Batman_Ratings.1);
    
    println!("My personal rating of Endgame is {} out of {}", Endgame_Ratings.0, Endgame_Ratings.1);
}

// working with Struct
pub struct Movie {
    name: String,
    genre: String,
    rating: u32,
}

pub fn struct_fn() {

    // Instance 1
    let mut movie1 = Movie {  // if instance is mut we can change the values of a specific field
        genre: String::from("Superhero"),
        rating: 10,
        name: String::from("The Batman")
    };  // we cannot make one specific field such as name or rating mutable, the entire instance must be mutable

    println!("Instance 1");

    // before changing the value
    println!("I am going to watch {} and I am so excited!", movie1.name);
    println!("The genre is {} and it is rated {}!", movie1.genre, movie1.rating);
    println!("");

    movie1.genre = String::from("Thriller");

    // before changing the value
    println!("I am going to watch {} and I am so excited!", movie1.name);
    println!("The genre is {} and it is rated {}!", movie1.genre, movie1.rating);
    println!("");

    // Instance 2
    let movie2 = Movie {
        genre: String::from("Action"),
        .. movie1  // This instance uses the value from the first instance {except for those we explicitly mention} becaue of this line of code
    };

    println!("Instance 2");
    println!("I am going to watch {} and I am so excited!", movie2.name);
    println!("The genre is {} and it is rated {}!", movie2.genre, movie2.rating);
    println!("");
}

/*
With stucts we can use the init shorthand to acess parameters in a quicker way,
but only if the parameter name and field name are the same
*/

pub fn struct_init(name: String, rating: u32) -> Movie {
    Movie {
        name,
        rating,
        genre: String::from("Action"),
    }
}

/*
When using indices we use the following

    let var = String::from("The Batman");

    let len = var.len();

    let var1 = &var[0 .. len];
    let var2 = &var[..]

var1 will be the same as var2
*/

pub fn reference_test() {

    let mut s = String::from("Robert Pattinson");

    println!("s is {}", s);

    change(&mut s);

    println!("s is {}", s);
}

/*
You can only reference one mutable reference at a time, the following will fail

let x = &mut s; 
let y = &mut s;

But if we use x before assigning y it will work fine

let x = &mut s; 
println!("x is {}", x);

let y = &mut s;
println!("y is {}", y);
*/

pub fn change(string_: &mut String) {

    string_.push_str(" is The Batman!");
}

pub fn referencing_function() {

    let var1 = String::from("The Batman");
    
    let var2 = string_len(&var1);

    println!("Length of '{}' is {}", var1, var2);
}

/*
We can use referencing to use the value of another variable without taking it's ownership!
*/

pub fn string_len(string_var: &String) -> usize {
    string_var.len()
}

#[allow(non_snake_case)]
pub fn ownership_Test() {
    
    let var1 = ownership_Test_1();

    println!("Checking scope of var1 {}", var1);

    let var2 = String::from("Ze Batman");

    println!("Checking scope of var2 {}", var2);

    let var3 = ownership_Test_2(var2);

    println!("Checking scope of var3 {}", var3);

    // println!("Checking scope of var2 {}", var2); // will not run as ownership of var2 has been transferred to var.
}

#[allow(non_snake_case)]
pub fn ownership_Test_1() -> String {

    let test_var = String::from("The Batman");

    test_var
}

#[allow(non_snake_case)]
pub fn ownership_Test_2(test_string:String) -> String {

    test_string
}

pub fn ownership_test() {

    let x = String::from("BLEH");

    println!("x now is {}", x); // x should give it's value now

    ownership_test_1(x);

    // println!("x now is {}", x); // x won't give it's value now as it has gone out of scope

    let y = 15;

    ownership_test_2(y);

    println!("y now is {}", y);
}

/* 
Ownership works the same as when assigning a value as when we pass a value to a function.
x's value wont be retained after passing it a function, since the String literal is stored on a heap due to it's unkown size at compile time.
So once it is passed to another function the value goes out of scope and is no longer usable.
*/

pub fn ownership_test_1(test_string: String) {
    println!("In ownership test fun {}", test_string);
} // here drop is called and the memory is freed

pub fn ownership_test_2(test_int: i32) {
    println!("In ownership test fun {}", test_int);
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

        let mut a:i32;
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