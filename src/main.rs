use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){
  println!("Guess The Number");
  let number = rand::rng().random_range(1..100);
  let mut v: Vec<usize> = Vec::new();
  loop {
    println!("Enter a number:");
    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line");
    
    let n: usize = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => continue
    };
    v.push(n);
    
    match n.cmp(&number){
        Ordering::Less => println!("Too Low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => {
            println!("You win"); 
            let median = median(&mut v);
            println!("{}",median);
            if median == n {
              println!("YOU ARE CRAZY GOOD")
            }
                break; }
    }

  }
  }

fn median(v: &mut Vec<usize>) -> usize {
  
  let len = v.len();
  
  if v.is_empty() {
    panic!("Length 0??");
  }
  if v.len()%2 ==0 {
    let index= (len/2)-1;
    let (_a,b,c) =  v[0..].select_nth_unstable(index);
    (*b+c[0])/2
  }
  else {
    let index=((len+1)/2)-1;
    let (_a,b,_c) =  v[0..].select_nth_unstable(index);
    *b  
  }
  
 

} 