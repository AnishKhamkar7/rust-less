use std::io;
use rand::Rng;

fn main(){
    let random_number = rand::rng().random_range(0..100);
    println!("WELCOME TO THE GUESSSING GAME");
    println!("Guess the number");

    let mut chance:i8 = 5;
    loop{
        let mut input  = String::new();
        
        
        io::stdin().read_line(&mut input).expect("Falied");

        let num:i32 = match input.trim().parse() {
            Ok(a) => a,
            Err(_) =>{
                println!("invalid Input! Please enter a valid Number");
                continue;
            }
        };

        if chance == 0 {
            println!("Sorry! You ran out of chances");
            break;
        }
        else if num == random_number {
            println!("Congratulation! You guessed it correctly in {} attempts!",chance);
            break;
        }
        else if num > random_number {
            println!("Try guessing Low");
            chance -= 1;
        }
        else {
            println!("Trye guessing Higher");
            chance -= 1;
        }
    }

}
