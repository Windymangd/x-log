use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let mut att = 10;
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(1..100);
    loop { 

        let mut number = String::new();

        // println!("{rand_num}");

        println!("\n Guess the secret number, you have {att} attempts left. \n");

        io::stdin()
            .read_line(&mut number)
            .expect("failed to read line (genuinely how did we get here)");

        let number: i16 = number.trim().parse().expect("not a number");

        println!("done!\n \n");
        
        if att == 0 {
            println!("you ran out of attempts");
            println!("the number was {rand_num}");
            break;
        } else if number == rand_num {
            println!("you win");
            break;
        } else if number <= rand_num {
            println!("too small");
            att -= 1;
        } else if number >= rand_num {
            println!("too big");
            att -= 1;
        }
    }



}
