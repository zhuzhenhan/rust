use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    
    println!("Plesase input your guess");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("the secret number is {}",secret_number);
    
    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("fail to read line");
    
     let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
        
  	println!("your guessed : {}", guess);
  	
  	
  	match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    
}
