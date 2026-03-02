 use std::io;
 use rand::Rng;

fn main() {
    println!("Guess the number");
    let mut secrate_number = 1;
    let mut guess_input = 0;

    while guess_input != secrate_number {

    println!("Enter your guess: ");
    let mut input_string = String::new();
    let new_secrate_number = rand::thread_rng().gen_range(1, 101);

    secrate_number = new_secrate_number;

    io::stdin().read_line(&mut input_string).expect("Faild to Read line");

    guess_input = input_string.trim().parse().expect("Please Enter number only");  

    println!("You guessed it wrong {} and secrate number is {}", guess_input, new_secrate_number)

    }

    println!("You have correclty guess the number: {}",guess_input);


}
