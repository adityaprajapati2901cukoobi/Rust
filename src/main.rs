use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("let play a guessing game");
    println!("guess a number you like");

    loop{
        //creating a immutable variable
    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guess:{}",guess);


    //generating random numbers
    let secret_number=rand::thread_rng().gen_range(1..101);
    println!("The secret number:{}",secret_number);

    let guess:i32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };

    match guess.cmp(&secret_number){
        Ordering::Equal=>{println!("{}","you win".green());break;}
        Ordering::Greater=>println!("{}","too big".blue()),
        Ordering::Less=>println!("{}","too less".red()),
    }
    }
}
