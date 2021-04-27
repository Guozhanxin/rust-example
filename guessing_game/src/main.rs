use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("welcome guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is:{}", secret_number);

    loop {

        println!("please input a numble:");
        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num)
                    .expect("failed to read line");

        let guess_num:u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ 通配符
        };
                    
        println!("you guess {}", guess_num);

        match guess_num.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
                
    }
}
