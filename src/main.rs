use std::{cmp::Ordering, io};
use rand::Rng;
fn main(){
  println!("Guess The Number");
  let number = rand::rng().random_range(1..100);
  
  
  loop {
    println!("Enter a number:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };
    
    match n.cmp(&number){
        Ordering::Less => println!("Too Low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => {
            println!("You win"); 
                break; }
    }

    
  }
}
